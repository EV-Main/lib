//! The port's hooks over the core: controlled/uncontrolled state, hydration-stable
//! ids, roving focus and the dismissable-layer stack. Same contracts as
//! [`uikit_dioxus::primitives`](crate::uikit_dioxus::primitives) where both exist.

use std::{
	collections::BTreeMap,
	sync::{
		Arc,
		atomic::{AtomicUsize, Ordering},
	},
};

use leptos::{prelude::*, reactive::owner::Owner};

pub use crate::uikit::{DismissReason, RovingOrientation, Side};
use crate::uikit::{KeyInput, LayerStack, Roving};

impl From<&leptos::ev::KeyboardEvent> for KeyInput {
	fn from(e: &leptos::ev::KeyboardEvent) -> Self {
		Self {
			key: e.key(),
			ctrl: e.ctrl_key(),
			shift: e.shift_key(),
			alt: e.alt_key(),
			meta: e.meta_key(),
			composing: e.is_composing(),
		}
	}
}

type DismissFn = Arc<dyn Fn(DismissReason) + Send + Sync>;

/// Controlled/uncontrolled state, the mirror of TS `useControllableState`. While
/// `controlled` holds a value the component is controlled — reads return it and
/// [`Controllable::set`] only forwards to `on_change`; otherwise it owns an
/// internal signal seeded from `default`.
pub struct Controllable<T: Send + Sync + 'static> {
	internal: RwSignal<T>,
	controlled: MaybeProp<T>,
	on_change: Option<Callback<T>>,
}
impl<T: Clone + Send + Sync + 'static> Controllable<T> {
	pub fn get(&self) -> T {
		self.controlled.get().unwrap_or_else(|| self.internal.get())
	}

	pub fn get_untracked(&self) -> T {
		self.controlled.get_untracked().unwrap_or_else(|| self.internal.get_untracked())
	}

	pub fn set(&self, next: T) {
		if self.controlled.get_untracked().is_none() {
			self.internal.set(next.clone());
		}
		if let Some(on_change) = self.on_change {
			on_change.run(next);
		}
	}
}
impl<T: Send + Sync + 'static> Clone for Controllable<T> {
	fn clone(&self) -> Self {
		*self
	}
}
impl<T: Send + Sync + 'static> Copy for Controllable<T> {}

pub fn use_controllable<T: Clone + Send + Sync + 'static>(controlled: MaybeProp<T>, default: T, on_change: Option<Callback<T>>) -> Controllable<T> {
	Controllable {
		internal: RwSignal::new(controlled.get_untracked().unwrap_or(default)),
		controlled,
		on_change,
	}
}

/// A DOM id for the calling component, the same on the server and in the
/// hydrating client — the Rust side of React's `useId`.
///
/// Read off the hydration context, which both sides advance in render order
/// (inside islands as well). Without one (CSR) nothing is hydrated, so a
/// process-wide counter cannot disagree with anything.
pub fn use_stable_id(prefix: &str) -> String {
	format!("{prefix}-{}", stable_id())
}

/// Arrow-key roving focus over a group of items — the mirror of the TS
/// `useRovingFocus`. The group calls [`use_roving_focus`], publishes the result
/// on its context and wires [`RovingFocus::next`] into its `on:keydown`; each
/// item calls [`use_roving_item`] and asks [`RovingFocus::is_tab_stop`] for its
/// `tabindex`.
#[derive(Clone, Copy)]
pub struct RovingFocus {
	roving: RwSignal<Roving>,
	/// Filled as items load, so empty under SSR — only [`RovingFocus::focus`]
	/// needs them, never the rendered markup.
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	els: StoredValue<BTreeMap<usize, web_sys::HtmlElement>, LocalStorage>,
}
impl RovingFocus {
	/// The key the arrow/`Home`/`End` in `e` walks to from the item keyed `from`.
	pub fn next(&self, e: &leptos::ev::KeyboardEvent, from: &str) -> Option<String> {
		self.roving.with_untracked(|r| r.next(&e.into(), from))
	}

	/// Whether the item keyed `key` is the group's single tab stop.
	pub fn is_tab_stop(&self, key: &str, selected: &str) -> bool {
		self.roving.with(|r| r.is_tab_stop(key, selected))
	}

	/// Moves DOM focus onto the item keyed `key`. A no-op before it has loaded.
	pub fn focus(&self, key: &str) {
		let id = self.roving.with_untracked(|r| r.id_of(key));
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		if let Some(el) = id.and_then(|id| self.els.with_value(|els| els.get(&id).cloned())) {
			// Focus is best-effort: a detached element just leaves focus where it was.
			let _ = el.focus();
		}
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		let _ = id; // no DOM to focus
	}
}

pub fn use_roving_focus(orientation: RovingOrientation) -> RovingFocus {
	RovingFocus {
		roving: RwSignal::new(Roving::new(orientation)),
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		els: StoredValue::new_local(BTreeMap::new()),
	}
}

/// Registers the calling item with its group under `key`, as it renders — so the
/// order the arrows walk, and every item's `tabindex`, is already right in
/// server-rendered markup — and records its element once `node` loads.
pub fn use_roving_item<E>(roving: RovingFocus, key: Signal<String>, node: NodeRef<E>)
where
	E: leptos::tachys::html::element::ElementType + 'static,
	E::Output: leptos::wasm_bindgen::JsCast + Clone + 'static, {
	static NEXT: AtomicUsize = AtomicUsize::new(0);
	let id = NEXT.fetch_add(1, Ordering::Relaxed);
	roving.roving.update(|r| r.register(id, key.get_untracked()));
	Effect::new(move |_| {
		let key = key.get();
		roving.roving.update(|r| r.register(id, key));
	});
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	node.on_load(move |el| roving.els.update_value(|els| drop(els.insert(id, leptos::wasm_bindgen::JsCast::unchecked_into(el)))));
	#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
	let _ = node; // no DOM to focus
	on_cleanup(move || {
		roving.roving.update(|r| r.unregister(id));
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		roving.els.update_value(|els| drop(els.remove(&id)));
	});
}

/// One overlay's place on the page's layer stack — the Rust side of the TS
/// `useDismissableLayer`. No document listener: only a key that reaches some
/// open layer's `on:keydown` dismisses anything, and "outside" is whatever
/// full-screen backdrop the overlay renders.
#[derive(Clone, Copy)]
pub struct DismissableLayer {
	id: usize,
	stack: Layers,
}
impl DismissableLayer {
	/// Handles Escape for this layer's `on:keydown`: when this layer is open the
	/// key is consumed (so no layer below hears it) and the topmost layer is
	/// dismissed. A closed layer leaves the key alone, for the layer around it.
	pub fn on_keydown(&self, e: &leptos::ev::KeyboardEvent) -> bool {
		let Some(top) = self.stack.0.with_value(|(stack, _)| stack.escape(self.id, &e.into())) else {
			return false;
		};
		e.stop_propagation();
		self.dismiss(top, DismissReason::Escape);
		true
	}

	/// A click on this layer's backdrop: dismisses the topmost layer.
	pub fn on_outside(&self) {
		if let Some(top) = self.stack.0.with_value(|(stack, _)| stack.top()) {
			self.dismiss(top, DismissReason::Outside);
		}
	}

	fn dismiss(&self, layer: usize, reason: DismissReason) {
		// Cloned out so the callback may re-enter the stack.
		let dismiss = self.stack.0.with_value(|(_, fns)| fns[&layer].clone());
		dismiss(reason);
	}
}

/// Puts the calling overlay on the layer stack while `open`, calling
/// `on_dismiss` when it is the top layer and Escape or an outside click arrives.
///
/// The stack follows renders, not handlers (see `LayerStack`): `open` is synced
/// by a render effect, which lands with the overlay's own re-render, after the
/// handler that closed it has finished. Created in the overlay's body, ahead of
/// its content, it runs before the content renders — so an overlay opening
/// inside another is pushed above it, in render order, as in the Dioxus port.
pub fn use_dismissable_layer(open: Signal<bool>, on_dismiss: impl Fn(DismissReason) + Send + Sync + 'static) -> DismissableLayer {
	static NEXT: AtomicUsize = AtomicUsize::new(0);
	let id = NEXT.fetch_add(1, Ordering::Relaxed);
	let root = std::iter::successors(Owner::current(), Owner::parent).last().expect("a component renders under an owner");
	let stack = root.with(|| {
		use_context::<Layers>().unwrap_or_else(|| {
			let layers = Layers(StoredValue::new((LayerStack::default(), BTreeMap::new())));
			provide_context(layers);
			layers
		})
	});
	let on_dismiss: DismissFn = Arc::new(on_dismiss);
	let sync = RenderEffect::new(move |_| {
		let open = open.get();
		stack.0.update_value(|(layers, fns)| {
			layers.sync(id, open);
			if open {
				fns.insert(id, on_dismiss.clone());
			} else {
				fns.remove(&id);
			}
		});
	});
	on_cleanup(move || {
		drop(sync);
		stack.0.update_value(|(layers, fns)| {
			layers.remove(id);
			fns.remove(&id);
		})
	});
	DismissableLayer { id, stack }
}

/// The number behind [`use_stable_id`], for a part whose registration id is its DOM id.
pub(crate) fn stable_id() -> usize {
	static NEXT: AtomicUsize = AtomicUsize::new(0);
	Owner::current_shared_context().map_or_else(|| NEXT.fetch_add(1, Ordering::Relaxed), |sc| sc.next_id().into_inner())
}

/// Whether a `transitionend` is `node`'s own exit `transform` finishing.
///
/// Every exiting overlay (toast, drawer) stays mounted with
/// `data-state="closed"` until its slide-out transition ends, then drops the
/// node. But `transitionend` bubbles, so the closing element also sees its
/// descendants' — a button's `transition-colors`, a child's own transform — and
/// dropping on those cuts the slide-out short. The target is compared with
/// `node` rather than `currentTarget`, which Leptos's event delegation leaves on
/// the document. Mirrors the TS guard (`e.propertyName === "transform"`).
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) fn is_transform_transition<E>(e: &leptos::ev::TransitionEvent, node: NodeRef<E>) -> bool
where
	E: leptos::tachys::html::element::ElementType + 'static,
	E::Output: leptos::wasm_bindgen::JsCast + Clone + 'static, {
	use leptos::wasm_bindgen::JsValue;
	let own = match (e.target(), node.get_untracked()) {
		(Some(target), Some(node)) => AsRef::<JsValue>::as_ref(&target) == AsRef::<JsValue>::as_ref(&node),
		_ => false,
	};
	own && e.property_name() == "transform"
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
pub(crate) fn is_transform_transition<E>(_: &leptos::ev::TransitionEvent, _: NodeRef<E>) -> bool
where
	E: leptos::tachys::html::element::ElementType + 'static,
	E::Output: leptos::wasm_bindgen::JsCast + Clone + 'static, {
	true // no DOM fires events off the web
}

/// The open layers of one page and their dismiss handlers, kept on the root
/// owner so every overlay shares it without a provider of the caller's.
#[derive(Clone, Copy)]
struct Layers(StoredValue<(LayerStack, BTreeMap<usize, DismissFn>)>);
