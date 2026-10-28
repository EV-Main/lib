use leptos::{context::Provider, prelude::*};

pub use crate::uikit::DropdownMenuItemVariant;
use crate::{
	cn,
	uikit_leptos::{
		DROPDOWN_MENU_CHECK_ITEM, DROPDOWN_MENU_CONTENT, DROPDOWN_MENU_ITEM, DROPDOWN_MENU_ITEM_INDICATOR, DROPDOWN_MENU_LABEL, DROPDOWN_MENU_SEPARATOR, DROPDOWN_MENU_SHORTCUT,
		DROPDOWN_MENU_SUB_CONTENT, DROPDOWN_MENU_SUB_TRIGGER,
		primitives::{Controllable, DismissableLayer, Side, use_controllable, use_dismissable_layer},
	},
};

#[component]
pub fn DropdownMenu(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let layer = use_dismissable_layer(Signal::derive(move || state.get()), move |_| state.set(false));
	let provided = DropdownMenuCtx { open: state, layer };
	view! {
		<Provider value=provided>
			<div class="relative inline-block" data-slot="dropdown-menu">
				{children()}
			</div>
		</Provider>
	}
}

#[component]
pub fn DropdownMenuTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<DropdownMenuCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="dropdown-menu-trigger"
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			aria-haspopup="menu"
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
		</button>
	}
}

#[component]
pub fn DropdownMenuContent(#[prop(optional)] side: Side, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	let ctx = expect_context::<DropdownMenuCtx>();
	let cls = cn!("absolute left-0 mt-1", DROPDOWN_MENU_CONTENT, class);
	move || {
		ctx.open.get().then(|| {
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| ctx.layer.on_outside()></div>
				<div
					class=cls.clone()
					role="menu"
					tabindex="-1"
					data-slot="dropdown-menu-content"
					data-state="open"
					data-side=side.as_ref().to_owned()
					on:keydown=move |e| {
						ctx.layer.on_keydown(&e);
					}
				>
					{children()}
				</div>
			}
		})
	}
}

#[component]
pub fn DropdownMenuGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=class role="group" data-slot="dropdown-menu-group">
			{children()}
		</div>
	}
}

#[component]
pub fn DropdownMenuItem(
	#[prop(optional)] variant: DropdownMenuItemVariant,
	#[prop(optional)] inset: bool,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let ctx = expect_context::<DropdownMenuCtx>();
	view! {
		<div
			class=cn!(DROPDOWN_MENU_ITEM, class)
			role="menuitem"
			tabindex="-1"
			data-slot="dropdown-menu-item"
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
pub fn DropdownMenuCheckboxItem(
	#[prop(optional, into)] checked: Signal<bool>,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let ctx = expect_context::<DropdownMenuCtx>();
	view! {
		<div
			class=cn!(DROPDOWN_MENU_CHECK_ITEM, class)
			role="menuitemcheckbox"
			tabindex="-1"
			data-slot="dropdown-menu-checkbox-item"
			data-state=move || if checked.get() { "checked" } else { "unchecked" }
			data-disabled=move || disabled.get().then_some("")
			aria-checked=move || checked.get().to_string()
			on:click=move |e| ctx.activate(disabled, on_click, e)
		>
			<span class=DROPDOWN_MENU_ITEM_INDICATOR>
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
pub fn DropdownMenuRadioGroup(
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(value, default_value, on_value_change);
	let provided = DropdownMenuRadioCtx { value: state };
	view! {
		<Provider value=provided>
			<div class=class role="group" data-slot="dropdown-menu-radio-group">
				{children()}
			</div>
		</Provider>
	}
}

#[component]
pub fn DropdownMenuRadioItem(#[prop(into)] value: String, #[prop(optional, into)] disabled: Signal<bool>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let menu = expect_context::<DropdownMenuCtx>();
	let radio = expect_context::<DropdownMenuRadioCtx>();
	let checked = Memo::new({
		let value = value.clone();
		move |_| radio.value.get() == value
	});
	view! {
		<div
			class=cn!(DROPDOWN_MENU_CHECK_ITEM, class)
			role="menuitemradio"
			tabindex="-1"
			data-slot="dropdown-menu-radio-item"
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
			<span class=DROPDOWN_MENU_ITEM_INDICATOR>
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
pub fn DropdownMenuLabel(#[prop(optional)] inset: bool, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div
			class=cn!(DROPDOWN_MENU_LABEL, class)
			data-slot="dropdown-menu-label"
			data-inset=inset.then_some("")
		>
			{children()}
		</div>
	}
}

#[component]
pub fn DropdownMenuSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	view! {
		<div
			class=cn!(DROPDOWN_MENU_SEPARATOR, class)
			role="separator"
			data-slot="dropdown-menu-separator"
		></div>
	}
}

#[component]
pub fn DropdownMenuShortcut(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<span class=cn!(DROPDOWN_MENU_SHORTCUT, class) data-slot="dropdown-menu-shortcut">
			{children()}
		</span>
	}
}

/// Sub-menus open inline (nested) rather than in a separate floating layer:
/// the sub-content is revealed in place when its trigger toggles, dropping the
/// nested portal Radix renders per level. Keyboard roving is simplified to the
/// browser's native focus order.
#[component]
pub fn DropdownMenuSub(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let provided = DropdownMenuSubCtx { open: state };
	view! {
		<Provider value=provided>
			<div data-slot="dropdown-menu-sub">{children()}</div>
		</Provider>
	}
}

#[component]
pub fn DropdownMenuSubTrigger(#[prop(optional)] inset: bool, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<DropdownMenuSubCtx>();
	view! {
		<div
			class=cn!(DROPDOWN_MENU_SUB_TRIGGER, class)
			role="menuitem"
			tabindex="-1"
			data-slot="dropdown-menu-sub-trigger"
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			data-inset=inset.then_some("")
			aria-haspopup="menu"
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
			<svg
				class="ml-auto size-4"
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
pub fn DropdownMenuSubContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<DropdownMenuSubCtx>();
	let cls = cn!(DROPDOWN_MENU_SUB_CONTENT, class);
	move || {
		ctx.open.get().then(|| {
			view! {
				<div class=cls.clone() role="menu" data-slot="dropdown-menu-sub-content" data-state="open">
					{children()}
				</div>
			}
		})
	}
}

#[derive(Clone, Copy)]
struct DropdownMenuCtx {
	open: Controllable<bool>,
	layer: DismissableLayer,
}
impl DropdownMenuCtx {
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
struct DropdownMenuRadioCtx {
	value: Controllable<String>,
}

#[derive(Clone, Copy)]
struct DropdownMenuSubCtx {
	open: Controllable<bool>,
}
