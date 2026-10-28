use leptos::{context::Provider, prelude::*};

pub use crate::uikit::ContextMenuItemVariant;
use crate::{
	cn,
	uikit_leptos::{
		CONTEXT_MENU_CHECK_ITEM, CONTEXT_MENU_CONTENT, CONTEXT_MENU_ITEM, CONTEXT_MENU_LABEL, CONTEXT_MENU_SEPARATOR, CONTEXT_MENU_SHORTCUT, CONTEXT_MENU_SUB_CONTENT,
		CONTEXT_MENU_SUB_TRIGGER,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn ContextMenu(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let provided = ContextMenuCtx {
		open: state,
		point: RwSignal::new((0, 0)),
	};
	view! {
		<Provider value=provided>
			<div data-slot="context-menu">{children()}</div>
		</Provider>
	}
}

#[component]
pub fn ContextMenuTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<ContextMenuCtx>();
	view! {
		<div
			class=class
			data-slot="context-menu-trigger"
			on:contextmenu=move |e| {
				e.prevent_default();
				ctx.point.set((e.client_x(), e.client_y()));
				ctx.open.set(true);
			}
		>
			{children()}
		</div>
	}
}

#[component]
pub fn ContextMenuContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	// dep-light: fixed coords from the contextmenu event + backdrop; no portal/floating — see README Limitations
	let ctx = expect_context::<ContextMenuCtx>();
	let cls = cn!(CONTEXT_MENU_CONTENT, class);
	move || {
		ctx.open.get().then(|| {
			let (x, y) = ctx.point.get();
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| ctx.open.set(false)></div>
				<div
					class=cls.clone()
					role="menu"
					tabindex="-1"
					style=format!("position: fixed; left: {x}px; top: {y}px;")
					data-slot="context-menu-content"
					data-state="open"
					data-side="bottom"
					on:keydown=move |e| {
						if e.key() == "Escape" {
							ctx.open.set(false);
						}
					}
				>
					{children()}
				</div>
			}
		})
	}
}

#[component]
pub fn ContextMenuGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=class role="group" data-slot="context-menu-group">
			{children()}
		</div>
	}
}

#[component]
pub fn ContextMenuItem(
	#[prop(optional)] variant: ContextMenuItemVariant,
	#[prop(optional)] inset: bool,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let ctx = expect_context::<ContextMenuCtx>();
	view! {
		<div
			class=cn!(CONTEXT_MENU_ITEM, class)
			role="menuitem"
			tabindex="-1"
			data-slot="context-menu-item"
			data-variant=variant.as_ref().to_owned()
			data-inset=inset.then_some("")
			data-disabled=move || disabled.get().then_some("")
			on:click=move |e| ctx.activate(disabled, on_click, e)
		>
			{children()}
		</div>
	}
}

#[component]
pub fn ContextMenuCheckboxItem(
	#[prop(optional, into)] checked: Signal<bool>,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let ctx = expect_context::<ContextMenuCtx>();
	view! {
		<div
			class=cn!(CONTEXT_MENU_CHECK_ITEM, class)
			role="menuitemcheckbox"
			tabindex="-1"
			data-slot="context-menu-checkbox-item"
			data-state=move || if checked.get() { "checked" } else { "unchecked" }
			data-disabled=move || disabled.get().then_some("")
			aria-checked=move || checked.get().to_string()
			on:click=move |e| ctx.activate(disabled, on_click, e)
		>
			<span class="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
				{move || {
					checked
						.get()
						.then(|| {
							view! {
								<svg
									class="size-4"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
									aria-hidden="true"
								>
									<path d="M20 6 9 17l-5-5"></path>
								</svg>
							}
						})
				}}
			</span>
			{children()}
		</div>
	}
}

#[component]
pub fn ContextMenuRadioGroup(
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(value, default_value, on_value_change);
	let provided = ContextMenuRadioCtx { value: state };
	view! {
		<Provider value=provided>
			<div class=class role="group" data-slot="context-menu-radio-group">
				{children()}
			</div>
		</Provider>
	}
}

#[component]
pub fn ContextMenuRadioItem(#[prop(into)] value: String, #[prop(optional, into)] disabled: Signal<bool>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let menu = expect_context::<ContextMenuCtx>();
	let radio = expect_context::<ContextMenuRadioCtx>();
	let checked = Memo::new({
		let value = value.clone();
		move |_| radio.value.get() == value
	});
	view! {
		<div
			class=cn!(CONTEXT_MENU_CHECK_ITEM, class)
			role="menuitemradio"
			tabindex="-1"
			data-slot="context-menu-radio-item"
			data-state=move || if checked.get() { "checked" } else { "unchecked" }
			data-disabled=move || disabled.get().then_some("")
			aria-checked=move || checked.get().to_string()
			on:click=move |_| {
				if disabled.get_untracked() {
					return;
				}
				radio.value.set(value.clone());
				menu.open.set(false);
			}
		>
			<span class="pointer-events-none absolute left-2 flex size-3.5 items-center justify-center">
				{move || {
					checked
						.get()
						.then(|| {
							view! {
								<svg class="size-2 fill-current" viewBox="0 0 24 24" aria-hidden="true">
									<circle cx="12" cy="12" r="10"></circle>
								</svg>
							}
						})
				}}
			</span>
			{children()}
		</div>
	}
}

#[component]
pub fn ContextMenuLabel(#[prop(optional)] inset: bool, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div
			class=cn!(CONTEXT_MENU_LABEL, class)
			data-slot="context-menu-label"
			data-inset=inset.then_some("")
		>
			{children()}
		</div>
	}
}

#[component]
pub fn ContextMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	view! {
		<div
			class=cn!(CONTEXT_MENU_SEPARATOR, class)
			role="separator"
			data-slot="context-menu-separator"
		></div>
	}
}

#[component]
pub fn ContextMenuShortcut(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<span class=cn!(CONTEXT_MENU_SHORTCUT, class) data-slot="context-menu-shortcut">
			{children()}
		</span>
	}
}

/// Sub-menus open inline (nested) rather than in a separate floating layer; the
/// sub-content is revealed in place when its trigger toggles. Keyboard roving is
/// simplified to the browser's native focus order.
#[component]
pub fn ContextMenuSub(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let provided = ContextMenuSubCtx { open: state };
	view! {
		<Provider value=provided>
			<div data-slot="context-menu-sub">{children()}</div>
		</Provider>
	}
}

#[component]
pub fn ContextMenuSubTrigger(#[prop(optional)] inset: bool, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<ContextMenuSubCtx>();
	view! {
		<div
			class=cn!(CONTEXT_MENU_SUB_TRIGGER, class)
			role="menuitem"
			tabindex="-1"
			data-slot="context-menu-sub-trigger"
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			data-inset=inset.then_some("")
			aria-haspopup="menu"
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
			<svg
				class="ml-auto"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="m9 18 6-6-6-6"></path>
			</svg>
		</div>
	}
}

#[component]
pub fn ContextMenuSubContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<ContextMenuSubCtx>();
	let cls = cn!(CONTEXT_MENU_SUB_CONTENT, class);
	move || {
		ctx.open.get().then(|| {
			view! {
				<div class=cls.clone() role="menu" data-slot="context-menu-sub-content" data-state="open">
					{children()}
				</div>
			}
		})
	}
}

#[derive(Clone, Copy)]
struct ContextMenuCtx {
	open: Controllable<bool>,
	point: RwSignal<(i32, i32)>,
}
impl ContextMenuCtx {
	/// An item's click: the caller's handler, then the menu closes — unless the item is disabled.
	fn activate(&self, disabled: Signal<bool>, on_click: Option<Callback<leptos::ev::MouseEvent>>, e: leptos::ev::MouseEvent) {
		if disabled.get_untracked() {
			return;
		}
		if let Some(h) = on_click {
			h.run(e);
		}
		self.open.set(false);
	}
}

#[derive(Clone, Copy)]
struct ContextMenuRadioCtx {
	value: Controllable<String>,
}

#[derive(Clone, Copy)]
struct ContextMenuSubCtx {
	open: Controllable<bool>,
}
