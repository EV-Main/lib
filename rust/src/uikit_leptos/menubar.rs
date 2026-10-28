use leptos::prelude::*;

pub use crate::uikit::MenubarItemVariant;
use crate::{
	cn,
	uikit_leptos::{
		MENUBAR_CHECKBOX_ITEM, MENUBAR_CONTENT, MENUBAR_ITEM, MENUBAR_ITEM_INDICATOR, MENUBAR_LABEL, MENUBAR_RADIO_ITEM, MENUBAR_ROOT, MENUBAR_SEPARATOR, MENUBAR_SHORTCUT,
		MENUBAR_SUB_CONTENT, MENUBAR_SUB_TRIGGER, MENUBAR_TRIGGER,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn Menubar(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(MENUBAR_ROOT, class) data-slot="menubar" role="menubar">
			{children()}
		</div>
	}
}

/// One menu in the bar. Owns its open state (the mirror of Radix's per-menu
/// root) and exposes it to its trigger/content via context.
#[component]
pub fn MenubarMenu(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	provide_context(MenubarMenuCtx { open: state });
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	view! {
		<div class="relative" data-slot="menubar-menu">
			{children()}
		</div>
	}
}

#[component]
pub fn MenubarTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<MenubarMenuCtx>();
	view! {
		<button
			type="button"
			class=cn!(MENUBAR_TRIGGER, class)
			data-slot="menubar-trigger"
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
		</button>
	}
}

#[component]
pub fn MenubarContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<MenubarMenuCtx>();
	let cls = cn!(MENUBAR_CONTENT, class);
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	move || {
		ctx.open.get().then(|| {
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| ctx.open.set(false)></div>
				<div
					class=cls.clone()
					data-slot="menubar-content"
					data-state="open"
					role="menu"
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
pub fn MenubarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=class data-slot="menubar-group" role="group">
			{children()}
		</div>
	}
}

#[component]
pub fn MenubarItem(
	#[prop(optional)] inset: bool,
	#[prop(optional)] variant: MenubarItemVariant,
	#[prop(optional, into)] class: String,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	view! {
		<div
			class=cn!(MENUBAR_ITEM, class)
			data-slot="menubar-item"
			data-inset=inset.then_some("")
			data-variant=variant.as_ref().to_owned()
			role="menuitem"
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			{children()}
		</div>
	}
}

/// `checked` is controlled: pair it with `on_click` to flip your own state. Like
/// [`MenubarItem`] — and unlike the dropdown/context checkbox items, whose TS
/// mirrors close on select — activating this does not close the menu, so a run of
/// toggles stays open. That matches the TS menubar, which spreads `onClick` onto a
/// plain div with no close context.
#[component]
pub fn MenubarCheckboxItem(
	#[prop(optional, into)] checked: Signal<bool>,
	#[prop(optional, into)] class: String,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	view! {
		<div
			class=cn!(MENUBAR_CHECKBOX_ITEM, class)
			data-slot="menubar-checkbox-item"
			role="menuitemcheckbox"
			aria-checked=move || checked.get().to_string()
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			<span class=MENUBAR_ITEM_INDICATOR>
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

/// Wraps a set of [`MenubarRadioItem`]s and publishes the selected value via
/// context, mirroring the TS radio-group state.
#[component]
pub fn MenubarRadioGroup(
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(value, default_value, on_value_change);
	provide_context(MenubarRadioCtx { value: state });
	view! {
		<div class=class data-slot="menubar-radio-group" role="group">
			{children()}
		</div>
	}
}

#[component]
pub fn MenubarRadioItem(#[prop(into)] value: String, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<MenubarRadioCtx>();
	let checked = Memo::new({
		let value = value.clone();
		move |_| ctx.value.get() == value
	});
	view! {
		<div
			class=cn!(MENUBAR_RADIO_ITEM, class)
			data-slot="menubar-radio-item"
			role="menuitemradio"
			aria-checked=move || checked.get().to_string()
			on:click=move |_| ctx.value.set(value.clone())
		>
			<span class=MENUBAR_ITEM_INDICATOR>
				{move || {
					checked
						.get()
						.then(|| {
							view! {
								<svg class="size-2 fill-current" viewBox="0 0 24 24">
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
pub fn MenubarLabel(#[prop(optional)] inset: bool, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(MENUBAR_LABEL, class) data-slot="menubar-label" data-inset=inset.then_some("")>
			{children()}
		</div>
	}
}

#[component]
pub fn MenubarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	view! {
		<div class=cn!(MENUBAR_SEPARATOR, class) data-slot="menubar-separator" role="separator"></div>
	}
}

#[component]
pub fn MenubarShortcut(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<span class=cn!(MENUBAR_SHORTCUT, class) data-slot="menubar-shortcut">
			{children()}
		</span>
	}
}

/// A nested sub-menu. Owns its own open state; rendered inline beside its
/// trigger (no separate positioning engine — see README Limitations).
#[component]
pub fn MenubarSub(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	provide_context(MenubarSubCtx { open: state });
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	view! {
		<div class="relative" data-slot="menubar-sub">
			{children()}
		</div>
	}
}

#[component]
pub fn MenubarSubTrigger(#[prop(optional)] inset: bool, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<MenubarSubCtx>();
	view! {
		<div
			class=cn!(MENUBAR_SUB_TRIGGER, class)
			data-slot="menubar-sub-trigger"
			data-inset=inset.then_some("")
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			role="menuitem"
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
			<svg
				class="ml-auto h-4 w-4"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="m9 18 6-6-6-6"></path>
			</svg>
		</div>
	}
}

#[component]
pub fn MenubarSubContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<MenubarSubCtx>();
	let cls = cn!(MENUBAR_SUB_CONTENT, class);
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	move || {
		ctx.open.get().then(|| {
			view! {
				<div
					class=cls.clone()
					data-slot="menubar-sub-content"
					data-state="open"
					role="menu"
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

#[derive(Clone, Copy)]
struct MenubarMenuCtx {
	open: Controllable<bool>,
}

#[derive(Clone, Copy)]
struct MenubarRadioCtx {
	value: Controllable<String>,
}

#[derive(Clone, Copy)]
struct MenubarSubCtx {
	open: Controllable<bool>,
}
