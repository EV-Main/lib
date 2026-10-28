//! Behaviour primitives shared by the interactive components.
//!
//! Unlike the TypeScript port, the Rust kit does **not** ship a Portal, a
//! measured floating engine, or a focus-trap: Dioxus has no renderer-agnostic
//! portal, and measuring layout needs `web-sys` (host-only, not I/O-free).
//! Overlays therefore render inline with `position: fixed`, a full-screen
//! backdrop for outside-dismiss, CSS-only placement via [`Side`], and the
//! browser's native focus order. See the README "Limitations". The primitives
//! that port cleanly — controlled/uncontrolled state, roving focus, the stack
//! of dismissable layers — live here.

use std::{
	cell::RefCell,
	collections::BTreeMap,
	rc::Rc,
	sync::atomic::{AtomicUsize, Ordering},
};

use dioxus::{dioxus_core::provide_root_context, prelude::*};

pub use crate::uikit::{DismissReason, RovingOrientation, Side};
use crate::uikit::{KeyInput, LayerStack, Placement, Roving};

type DismissFn = Rc<dyn Fn(DismissReason)>;

impl From<&KeyboardEvent> for KeyInput {
	fn from(e: &KeyboardEvent) -> Self {
		let m = e.modifiers();
		Self {
			key: e.key().to_string(),
			ctrl: m.ctrl(),
			shift: m.shift(),
			alt: m.alt(),
			meta: m.meta(),
			composing: e.is_composing(),
		}
	}
}

/// Controlled/uncontrolled state, the mirror of TS `useControllableState`. When
/// `controlled` is `Some` the component is controlled — reads return that value
/// and [`Controllable::set`] only forwards to `on_change`; otherwise it owns an
/// internal [`Signal`] seeded from `default`.
pub struct Controllable<T: Clone + PartialEq + 'static> {
	signal: Signal<T>,
	controlled: bool,
	on_change: Option<EventHandler<T>>,
}
impl<T: Clone + PartialEq + 'static> Controllable<T> {
	pub fn get(&self) -> T {
		self.signal.read().clone()
	}

	pub fn set(&self, next: T) {
		if !self.controlled {
			let mut sig = self.signal;
			sig.set(next.clone());
		}
		if let Some(handler) = &self.on_change {
			handler.call(next);
		}
	}
}

impl<T: Clone + PartialEq + 'static> Clone for Controllable<T> {
	fn clone(&self) -> Self {
		*self
	}
}
impl<T: Clone + PartialEq + 'static> Copy for Controllable<T> {}

/// Seeds a [`Controllable`] state. Keeps the internal signal in sync with the
/// controlled value across re-renders.
pub fn use_controllable<T: Clone + PartialEq + 'static>(controlled: Option<T>, default: T, on_change: Option<EventHandler<T>>) -> Controllable<T> {
	let mut signal = use_signal(|| controlled.clone().unwrap_or(default));
	if let Some(value) = controlled.clone()
		&& *signal.peek() != value
	{
		signal.set(value);
	}
	Controllable {
		signal,
		controlled: controlled.is_some(),
		on_change,
	}
}

/// A DOM id for the calling component, derived from its place in the tree —
/// the Rust side of React's `useId`.
///
/// The id is the component's `ScopeId`, which the `VirtualDom` hands out in
/// render order: the server render and the client's hydrating render of the
/// same tree mint the same ids, so `for`/`aria-*` wiring survives hydration. A
/// process-global counter would not — a long-lived server has advanced it
/// across earlier requests while a fresh client starts from zero.
///
/// Unique among the components mounted in one `VirtualDom` (a scope id is only
/// recycled after its component unmounts). Two independent `VirtualDom`s on one
/// page can collide; give them different `prefix`es.
pub fn use_stable_id(prefix: &str) -> String {
	use_hook(|| format!("{prefix}-{}", dioxus::dioxus_core::current_scope_id().0))
}

/// Arrow-key roving focus over a group of items — the mirror of the TS
/// `useRovingFocus`, and what earns a group the right to take its items out of
/// the tab order: the group is one tab stop, and the arrows move within it.
///
/// The group calls [`use_roving_focus`], publishes the result on its context,
/// and wires [`RovingFocus::next`] into its `onkeydown`; each item calls
/// [`use_roving_item`], reports its element via `onmounted`, and asks
/// [`RovingFocus::is_tab_stop`] for its `tabindex`.
///
/// Items are keyed by the same value the group selects on, so navigating is
/// "find the neighbour of the selected key". Movement wraps at both ends, as
/// `useRovingFocus`'s default `loop: true` does.
#[derive(Clone, Copy)]
pub struct RovingFocus {
	roving: Signal<Roving>,
	/// Set on mount, so empty under SSR and on non-web renderers — only
	/// [`RovingFocus::focus`] needs them, never the rendered markup.
	els: Signal<BTreeMap<usize, Rc<MountedData>>>,
}
impl RovingFocus {
	/// The key the arrow/`Home`/`End` in `e` walks to, starting from the item
	/// keyed `from`; `None` when `e` isn't a navigation key for this
	/// orientation, or the group is empty. The caller decides what to do with
	/// it — both consumers select it and [`RovingFocus::focus`] it.
	pub fn next(&self, e: &KeyboardEvent, from: &str) -> Option<String> {
		self.roving.read().next(&e.into(), from)
	}

	/// Whether the item keyed `key` is the group's single tab stop.
	pub fn is_tab_stop(&self, key: &str, selected: &str) -> bool {
		self.roving.read().is_tab_stop(key, selected)
	}

	/// Moves DOM focus onto the item keyed `key`. A no-op before that item has
	/// mounted, and on renderers without a DOM.
	pub fn focus(&self, key: &str) {
		let id = self.roving.read().id_of(key);
		if let Some(el) = id.and_then(|id| self.els.read().get(&id).cloned()) {
			// Focus is best-effort: a detached element just leaves focus where
			// it was, which is why the result is dropped rather than surfaced.
			spawn(async move {
				let _ = el.set_focus(true).await;
			});
		}
	}

	/// Records the mounted element for the item registered as `id` — call from
	/// the item's `onmounted`.
	pub fn attach(&self, id: usize, el: Rc<MountedData>) {
		let mut els = self.els;
		els.write().insert(id, el);
	}
}

/// Seeds the [`RovingFocus`] a group owns and hands to its items via context.
pub fn use_roving_focus(orientation: RovingOrientation) -> RovingFocus {
	RovingFocus {
		roving: use_signal(|| Roving::new(orientation)),
		els: use_signal(BTreeMap::new),
	}
}
/// Registers the calling item with its group's [`RovingFocus`] under `key`,
/// returning the id to hand back to [`RovingFocus::attach`] on mount.
///
/// Registration happens during the item's first render, not on mount, so the
/// order the arrows walk — and every item's `tabindex` — is already right in
/// server-rendered markup.
pub fn use_roving_item(roving: RovingFocus, key: String) -> usize {
	let id = use_hook({
		let key = key.clone();
		move || {
			let id = NEXT_ROVING_ITEM_ID.fetch_add(1, Ordering::Relaxed);
			let mut items = roving.roving;
			items.write().register(id, key);
			id
		}
	});
	// `use_hook` only ever sees the first render, so a re-keyed item would
	// otherwise leave a stale key for the arrows to walk onto.
	use_effect(use_reactive!(|key| {
		let mut items = roving.roving;
		items.write().register(id, key);
	}));
	use_drop(move || {
		let mut items = roving.roving;
		items.write().unregister(id);
		let mut els = roving.els;
		els.write().remove(&id);
	});
	id
}
/// One overlay's place on the page's [`LayerStack`] — the Rust side of the TS
/// `useDismissableLayer`.
///
/// Unlike the TS hook there is no document listener (that needs `web-sys`):
/// only a key that reaches some open layer's `onkeydown` dismisses anything,
/// and "outside" is whatever full-screen backdrop the overlay renders.
#[derive(Clone)]
pub struct DismissableLayer {
	id: usize,
	stack: Layers,
}
impl DismissableLayer {
	/// Handles Escape for this layer's `onkeydown`: when this layer is open the
	/// key is consumed (so no layer below hears it) and the topmost layer is
	/// dismissed. A closed layer leaves the key alone, for the layer around it.
	pub fn on_keydown(&self, e: &KeyboardEvent) -> bool {
		let Some(top) = self.stack.0.borrow().0.escape(self.id, &e.into()) else {
			return false;
		};
		e.stop_propagation();
		self.dismiss(top, DismissReason::Escape);
		true
	}

	/// A click on this layer's backdrop: dismisses the topmost layer, which is
	/// this one unless another opened above it.
	pub fn on_outside(&self) {
		let top = self.stack.0.borrow().0.top();
		if let Some(top) = top {
			self.dismiss(top, DismissReason::Outside);
		}
	}

	fn dismiss(&self, layer: usize, reason: DismissReason) {
		// Cloned out so the dismiss callback may render or re-enter the stack
		// without a live borrow.
		let dismiss = self.stack.0.borrow().1[&layer].clone();
		dismiss(reason);
	}
}

/// Puts the calling overlay on the layer stack while `open`, calling
/// `on_dismiss` when it is the top layer and Escape or an outside click
/// arrives.
pub fn use_dismissable_layer(open: bool, on_dismiss: impl Fn(DismissReason) + 'static) -> DismissableLayer {
	let id = use_hook(|| NEXT_LAYER_ID.fetch_add(1, Ordering::Relaxed));
	let stack = use_hook(|| try_consume_context::<Layers>().unwrap_or_else(|| provide_root_context(Layers::default())));
	{
		let mut layers = stack.0.borrow_mut();
		layers.0.sync(id, open);
		// Refreshed on each render: the handler a caller wires up now is the one that fires.
		if open {
			layers.1.insert(id, Rc::new(on_dismiss));
		} else {
			layers.1.remove(&id);
		}
	}
	use_drop({
		let stack = stack.clone();
		move || {
			let mut layers = stack.0.borrow_mut();
			layers.0.remove(id);
			layers.1.remove(&id);
		}
	});
	DismissableLayer { id, stack }
}
static NEXT_ROVING_ITEM_ID: AtomicUsize = AtomicUsize::new(0);

/// The open layers of one `VirtualDom` and their dismiss handlers. Lives in the
/// root scope, so every overlay in the tree shares it without a provider of the
/// caller's. Not a signal: it is read only from event handlers, never to render.
#[derive(Clone, Default)]
struct Layers(Rc<RefCell<(LayerStack, BTreeMap<usize, DismissFn>)>>);

static NEXT_LAYER_ID: AtomicUsize = AtomicUsize::new(0);

/// The rows' mounted elements: where they sit is for the web renderer to say;
/// elsewhere (SSR, desktop) and before mount nothing is placed.
impl Placement<usize> for BTreeMap<usize, Rc<MountedData>> {
	fn placed(&self, id: usize) -> bool {
		self.get(&id).is_some_and(|el| is_placed(el))
	}

	fn precedes(&self, a: usize, b: usize) -> bool {
		match (self.get(&a), self.get(&b)) {
			(Some(a), Some(b)) => document_precedes(a, b),
			_ => false,
		}
	}
}

/// Whether the renderer can place `el` in the document: the web renderer, for
/// an element that is still connected.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn is_placed(el: &MountedData) -> bool {
	el.downcast::<web_sys::Element>().is_some_and(|el| el.is_connected())
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
fn is_placed(_: &MountedData) -> bool {
	false
}

/// Whether `a` comes before `b` in the document. Only asked of placed elements.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn document_precedes(a: &MountedData, b: &MountedData) -> bool {
	match (a.downcast::<web_sys::Element>(), b.downcast::<web_sys::Element>()) {
		(Some(a), Some(b)) => a.compare_document_position(b) & web_sys::Node::DOCUMENT_POSITION_FOLLOWING != 0,
		_ => false,
	}
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
fn document_precedes(_: &MountedData, _: &MountedData) -> bool {
	false
}

/// Whether a `transitionend` is the element's own exit `transform` finishing.
///
/// Every exiting overlay (toast, drawer) stays mounted with
/// `data-state="closed"` until its slide-out transition ends, then drops the
/// node. But `transitionend` bubbles, so the closing element also sees its
/// descendants' — a button's 150ms `transition-colors`, a child's opacity fade —
/// and dropping on those cuts the slide-out short. Mirrors the TS guard
/// (`e.propertyName === "transform"`).
///
/// Dioxus 0.7's `TransitionData` exposes no `property_name` accessor, so the name
/// is read off the concrete web event. Web-only: another renderer can't report it
/// and falls through to accepting the event, which is at worst the unguarded
/// behaviour — never a node stranded on screen.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) fn is_transform_transition(e: &Event<TransitionData>) -> bool {
	e.downcast::<web_sys::TransitionEvent>().is_none_or(|t| {
		// transitionend bubbles: a descendant's own transform (a progress bar,
		// a hovered button) must not pass for the node's exit
		let own = t.target().is_some_and(|target| t.current_target().is_some_and(|current| current == target));
		own && t.property_name() == "transform"
	})
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
pub(crate) fn is_transform_transition(_: &Event<TransitionData>) -> bool {
	true
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::{
		Dialog, DialogContent, Popover, PopoverContent, PopoverTrigger,
		test_util::{render, render_after_click, render_after_keys},
	};

	fn popover_in_dialog() -> Element {
		rsx! {
			Dialog { default_open: true,
				DialogContent { show_close_button: false,
					Popover { default_open: true,
						PopoverTrigger { "more" }
						PopoverContent { "panel" }
					}
				}
			}
		}
	}

	#[test]
	fn escape_goes_to_the_top_layer_even_from_a_lower_one() {
		// The sweep reaches the dialog's own `onkeydown` as well as the popover's.
		let html = render_after_keys(popover_in_dialog, &[Key::Escape]);
		assert!(!html.contains("data-slot=\"popover-content\""), "{html}");
		assert!(html.contains("role=\"dialog\""), "the dialog below stays: {html}");
		let html = render_after_keys(popover_in_dialog, &[Key::Escape, Key::Escape]);
		assert!(!html.contains("role=\"dialog\""), "the next Escape is the dialog's: {html}");
	}

	#[test]
	fn a_click_on_a_lower_scrim_dismisses_the_top_layer() {
		let html = render_after_click(popover_in_dialog);
		assert!(!html.contains("data-slot=\"popover-content\""), "{html}");
		assert!(html.contains("role=\"dialog\""), "the dialog's scrim hands the click to the popover: {html}");
	}

	#[test]
	fn a_closed_layer_leaves_escape_to_the_one_around_it() {
		fn app() -> Element {
			rsx! {
				Dialog { default_open: true,
					DialogContent { show_close_button: false,
						Popover {
							PopoverTrigger { "more" }
							PopoverContent { "panel" }
						}
					}
				}
			}
		}
		let html = render_after_keys(app, &[Key::Escape]);
		assert!(!html.contains("role=\"dialog\""), "{html}");
	}

	#[test]
	fn uncontrolled_uses_default_then_updates() {
		fn app() -> Element {
			let state = use_controllable::<bool>(None, false, None);
			let label = if state.get() { "on" } else { "off" };
			rsx! {
				button {
					onclick: move |_| state.set(true),
					{label}
				}
			}
		}
		// On first render the default is observed.
		assert!(render(app).contains("off"));
	}

	#[test]
	fn items_register_during_render_so_ssr_markup_has_a_tab_stop() {
		fn app() -> Element {
			let roving = use_roving_focus(RovingOrientation::Vertical);
			use_context_provider(|| roving);
			rsx! {
				Item { value: "a" }
				Item { value: "b" }
			}
		}
		#[component]
		fn Item(value: String) -> Element {
			let roving = use_context::<RovingFocus>();
			use_roving_item(roving, value.clone());
			// Nothing selected, so only the first item may be the tab stop.
			let stop = roving.is_tab_stop(&value, "");
			rsx! {
				button { tabindex: if stop { "0" } else { "-1" } }
			}
		}
		let html = render(app);
		assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
		assert!(html.starts_with("<button tabindex=\"0\""), "the first item takes the stop: {html}");
	}

	#[test]
	fn controlled_reflects_external_value() {
		fn app() -> Element {
			let state = use_controllable::<bool>(Some(true), false, None);
			let label = if state.get() { "on" } else { "off" };
			rsx! { span { {label} } }
		}
		assert!(render(app).contains("on"));
	}
}
