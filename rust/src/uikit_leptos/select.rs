#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit::{self, SelectKey, SelectOption},
	uikit_leptos::{
		SELECT_CONTENT_BOUNDS, SELECT_ITEM, Size,
		primitives::{Controllable, DismissReason, use_controllable, use_dismissable_layer, use_stable_id},
		select_trigger_size_class,
	},
};

// dep-light: inline positioning + backdrop; no portal/floating/drag — see README Limitations

#[component]
pub fn Select(
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	/// What a reset would put back: its option is marked, beneath the chosen one's mark.
	#[prop(optional, into)]
	reset_value: MaybeProp<String>,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let provided = SelectCtx {
		value: use_controllable(value, default_value, on_value_change),
		reset: reset_value,
		open: use_controllable(open, default_open, on_open_change),
		content_id: Signal::stored(use_stable_id("select-content")),
		trigger: NodeRef::new(),
		state: StoredValue::new(uikit::Select::default()),
		els: Els::new(),
	};
	view! {
		<Provider value=provided>
			<div class=cn!("relative", class) data-slot="select">
				{children()}
			</div>
		</Provider>
	}
}

#[component]
pub fn SelectTrigger(#[prop(optional)] size: Size, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<SelectCtx>();
	// The placeholder mark is on `SelectValue`'s span, not on this button.
	let cls = cn!(
		"border-input [&_[data-placeholder]]:text-ink-soft [&_svg:not([class*='text-'])]:text-ink-soft \
		 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-accent-error/20 aria-invalid:border-accent-error \
		 flex w-fit items-center justify-between gap-2 rounded-[var(--control-radius)] border bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs \
		 transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 \
		 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex \
		 *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 [&_svg]:pointer-events-none [&_svg]:shrink-0 \
		 [&_svg:not([class*='size-'])]:size-4",
		select_trigger_size_class(size),
		class
	);
	view! {
		<button
			type="button"
			role="combobox"
			class=cls
			data-slot="select-trigger"
			data-size=size.as_ref().to_owned()
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			aria-haspopup="listbox"
			aria-expanded=move || ctx.open.get().to_string()
			aria-controls=move || ctx.open.get().then(|| ctx.content_id.get())
			node_ref=ctx.trigger
			on:click=move |_| if ctx.open.get_untracked() { ctx.close(true) } else { ctx.open.set(true) }
			on:keydown=move |e| {
				if !ctx.open.get_untracked() && matches!(e.key().as_str(), "ArrowDown" | "ArrowUp") {
					e.prevent_default();
					ctx.open.set(true);
				}
			}
		>
			{children()}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="size-4 opacity-50"
			>
				<path d="m6 9 6 6 6-6"></path>
			</svg>
		</button>
	}
}

#[component]
pub fn SelectValue(#[prop(optional, into)] placeholder: String, #[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = expect_context::<SelectCtx>();
	let empty = Memo::new(move |_| ctx.value.get().is_empty());
	view! {
		<span
			class=class
			data-slot="select-value"
			data-placeholder=move || empty.get().then_some("true")
		>
			{move || if empty.get() { placeholder.clone() } else { ctx.value.get() }}
		</span>
	}
}

#[component]
pub fn SelectContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<SelectCtx>();
	// Focus goes back to the trigger after a key, never after a click elsewhere,
	// where it would land on the trigger for a moment and fire its blur on the way out.
	let layer = use_dismissable_layer(Signal::derive(move || ctx.open.get()), move |reason| ctx.close(reason == DismissReason::Escape));
	let cls = cn!(
		"bg-popover text-ink absolute top-full left-0 z-50 mt-1 overflow-x-hidden overflow-y-auto rounded-md border border-border shadow-md",
		SELECT_CONTENT_BOUNDS,
		class
	);
	move || {
		if !ctx.open.get() {
			// A caller's controlled `open: false` closes the list without
			// `SelectCtx::close`; an option left active would stop the next open
			// from landing, and focus would stay on the trigger.
			ctx.state.update_value(|s| s.close());
			return None;
		}
		let children = children.clone();
		Some(view! {
			<div class="fixed inset-0 z-40" on:click=move |_| layer.on_outside()></div>
			<div
				role="listbox"
				id=ctx.content_id
				class=cls.clone()
				data-slot="select-content"
				data-state="open"
				tabindex="-1"
				on:keydown=move |e| {
					if !layer.on_keydown(&e) {
						ctx.on_list_key(&e);
					}
				}
			>
				<div class="p-1">{children()}</div>
			</div>
		})
	}
}

#[component]
pub fn SelectItem(
	#[prop(into)] value: String,
	/// What the type-ahead matches — the text the option reads as, which the
	/// kit cannot read off `children`.
	#[prop(into)]
	text_value: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	static NEXT: AtomicUsize = AtomicUsize::new(0);
	let ctx = expect_context::<SelectCtx>();
	let id = NEXT.fetch_add(1, Ordering::Relaxed);
	// Registered as it renders, so the keys know the order before anything has mounted.
	let option = {
		let value = value.clone();
		move || SelectOption {
			value: value.clone(),
			label: text_value.clone(),
			disabled: disabled.get(),
		}
	};
	ctx.state.update_value(|s| s.register(id, untrack(&option)));
	Effect::new(move |_| {
		let option = option();
		if ctx.state.with_value(|s| s.option(id) != Some(&option)) {
			ctx.state.update_value(|s| s.register(id, option));
		}
	});
	on_cleanup(move || {
		ctx.state.update_value(|s| s.unregister(id));
		ctx.els.forget(id);
	});
	let node = NodeRef::<leptos::html::Div>::new();
	ctx.attach(id, node);
	let selected = Memo::new({
		let value = value.clone();
		move |_| ctx.value.get() == value
	});
	let reset = Memo::new({
		let value = value.clone();
		move |_| ctx.reset.get().as_deref() == Some(value.as_str())
	});
	let focused = value.clone();
	view! {
		<div
			role="option"
			class=cn!(SELECT_ITEM, class)
			data-slot="select-item"
			aria-selected=move || selected.get().to_string()
			data-reset=move || reset.get().then_some("")
			aria-disabled=move || disabled.get().then_some("true")
			data-disabled=move || disabled.get().then_some("")
			tabindex="-1"
			node_ref=node
			on:focus=move |_| {
				if ctx.state.with_value(|s| s.active() != Some(focused.as_str())) {
					ctx.state.update_value(|s| s.focused(focused.clone()));
				}
			}
			on:click=move |_| {
				if !disabled.get_untracked() {
					ctx.choose(value.clone());
				}
			}
		>
			{move || {
				selected
					.get()
					.then(|| {
						view! {
							<span class="absolute right-2 flex size-3.5 items-center justify-center">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									width="24"
									height="24"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
									class="size-4"
									aria-hidden="true"
								>
									<path d="M20 6 9 17l-5-5"></path>
								</svg>
							</span>
						}
					})
			}}
			<span>{children()}</span>
		</div>
	}
}

#[component]
pub fn SelectGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div role="group" class=class data-slot="select-group">
			{children()}
		</div>
	}
}

#[component]
pub fn SelectLabel(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!("text-ink-soft px-2 py-1.5 text-xs", class) data-slot="select-label">
			{children()}
		</div>
	}
}

#[component]
pub fn SelectSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	view! {
		<div
			class=cn!("bg-border pointer-events-none -mx-1 my-1 h-px", class)
			data-slot="select-separator"
		></div>
	}
}

#[derive(Clone, Copy)]
struct SelectCtx {
	value: Controllable<String>,
	reset: MaybeProp<String>,
	open: Controllable<bool>,
	/// The listbox's id, for the trigger's `aria-controls`.
	content_id: Signal<String>,
	/// Where focus goes back to once the list closes on a key or a choice.
	trigger: NodeRef<leptos::html::Button>,
	/// Not reactive: nothing renders from it, only handlers read it.
	state: StoredValue<uikit::Select>,
	els: Els,
}
impl SelectCtx {
	fn choose(&self, value: String) {
		self.value.set(value);
		self.close(true);
	}

	/// `restore_focus`: hand focus back to the trigger — after a key or a
	/// choice, never after a click elsewhere.
	fn close(&self, restore_focus: bool) {
		self.open.set(false);
		self.state.update_value(|s| s.close());
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		if restore_focus && let Some(el) = self.trigger.get_untracked() {
			// Best-effort: a detached trigger just leaves focus where it was.
			let _ = el.focus();
		}
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		let _ = restore_focus; // no DOM to focus
	}

	/// Records option `id`'s element once it loads; opening may land on it.
	fn attach(&self, id: usize, node: NodeRef<leptos::html::Div>) {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		{
			let ctx = *self;
			node.on_load(move |el| {
				ctx.els.els.update_value(|els| drop(els.insert(id, el.into())));
				let chosen = ctx.value.get_untracked();
				let mut landed = None;
				ctx.state.update_value(|s| landed = s.mounted(id, &chosen, &ctx.els));
				if let Some(id) = landed {
					ctx.els.focus(id);
				}
			});
		}
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		let _ = (id, node); // no DOM
	}

	fn on_list_key(&self, e: &leptos::ev::KeyboardEvent) {
		let chosen = self.value.get_untracked();
		let mut effect = None;
		self.state.update_value(|s| effect = s.key(&e.into(), &chosen, crate::uikit_host::now_ms(), &self.els));
		let Some(effect) = effect else { return };
		if effect == SelectKey::Leave {
			return self.close(false);
		}
		e.prevent_default();
		match effect {
			SelectKey::Moved(id) => self.els.focus(id),
			SelectKey::Choose(value) => self.choose(value),
			SelectKey::Held | SelectKey::Leave => {}
		}
	}
}

/// The options' elements, filled as they load: their document order, and where
/// focus lands. Empty under SSR, where nothing is placed.
#[derive(Clone, Copy)]
struct Els {
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	els: StoredValue<BTreeMap<usize, web_sys::HtmlElement>, LocalStorage>,
}
impl Els {
	fn new() -> Self {
		Self {
			#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
			els: StoredValue::new_local(BTreeMap::new()),
		}
	}

	fn forget(&self, id: usize) {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		self.els.update_value(|els| drop(els.remove(&id)));
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		let _ = id; // no DOM
	}

	fn focus(&self, id: usize) {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		if let Some(el) = self.els.with_value(|els| els.get(&id).cloned()) {
			// Best-effort: a detached option just leaves focus where it was.
			let _ = el.focus();
		}
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		let _ = id; // no DOM to focus
	}
}
impl uikit::Placement<usize> for Els {
	fn placed(&self, id: usize) -> bool {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		return self.els.with_value(|els| els.get(&id).is_some_and(|el| el.is_connected()));
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		{
			let _ = id; // no DOM
			false
		}
	}

	fn precedes(&self, a: usize, b: usize) -> bool {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		return self.els.with_value(|els| match (els.get(&a), els.get(&b)) {
			(Some(a), Some(b)) => a.compare_document_position(b) & web_sys::Node::DOCUMENT_POSITION_FOLLOWING != 0,
			_ => false,
		});
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		{
			let _ = (a, b); // no DOM
			false
		}
	}
}
