use leptos::prelude::*;

pub use crate::uikit::navigation_menu_trigger_style;
use crate::{
	cn,
	uikit_leptos::{
		NAVIGATION_MENU, NAVIGATION_MENU_CONTENT, NAVIGATION_MENU_INDICATOR, NAVIGATION_MENU_ITEM, NAVIGATION_MENU_LINK, NAVIGATION_MENU_LIST, NAVIGATION_MENU_TRIGGER_STYLE,
		NAVIGATION_MENU_VIEWPORT,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn NavigationMenu(#[prop(default = true)] viewport: bool, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<nav
			class=cn!(NAVIGATION_MENU, class)
			data-slot="navigation-menu"
			data-viewport=viewport.to_string()
			role="navigation"
		>
			{children()}
		</nav>
	}
}

#[component]
pub fn NavigationMenuList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<ul class=cn!(NAVIGATION_MENU_LIST, class) data-slot="navigation-menu-list" role="list">
			{children()}
		</ul>
	}
}

/// One navigation item. Owns the open state of its panel and exposes it to its
/// trigger and content via context — the mirror of Radix's per-item root.
#[component]
pub fn NavigationMenuItem(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	provide_context(NavigationMenuItemCtx { open: state });
	view! {
		<li class=cn!(NAVIGATION_MENU_ITEM, class) data-slot="navigation-menu-item">
			{children()}
		</li>
	}
}

#[component]
pub fn NavigationMenuTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<NavigationMenuItemCtx>();
	view! {
		<button
			type="button"
			class=cn!(NAVIGATION_MENU_TRIGGER_STYLE, "group", class)
			data-slot="navigation-menu-trigger"
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
			" "
			<svg
				class="relative top-[1px] ml-1 size-3 transition duration-300 group-data-[state=open]:rotate-180"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="m6 9 6 6 6-6"></path>
			</svg>
		</button>
	}
}

#[component]
pub fn NavigationMenuContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<NavigationMenuItemCtx>();
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations.
	// `relative z-50` keeps the panel above the z-40 backdrop (the shared class is z-auto
	// because the TS mirror dismisses via events and has no backdrop to outstack).
	let cls = cn!(NAVIGATION_MENU_CONTENT, "relative z-50", class);
	move || {
		ctx.open.get().then(|| {
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| ctx.open.set(false)></div>
				<div
					class=cls.clone()
					data-slot="navigation-menu-content"
					data-state="open"
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
pub fn NavigationMenuLink(#[prop(optional, into)] active: Signal<bool>, #[prop(optional, into)] href: String, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<a
			class=cn!(NAVIGATION_MENU_LINK, class)
			data-slot="navigation-menu-link"
			data-active=move || active.get().to_string()
			href=href
		>
			{children()}
		</a>
	}
}

/// Visual arrow pointing at the active trigger. Animation is simplified — it
/// renders statically (no measured position; see README Limitations).
#[component]
pub fn NavigationMenuIndicator(#[prop(optional, into)] class: String) -> impl IntoView {
	view! {
		<div class=cn!(NAVIGATION_MENU_INDICATOR, class) data-slot="navigation-menu-indicator">
			<div class="bg-border relative top-[60%] h-2 w-2 rotate-45 rounded-tl-sm shadow-md"></div>
		</div>
	}
}

/// Shared surface the open panels animate into. Animation/measured sizing is
/// simplified to a static container (see README Limitations).
#[component]
pub fn NavigationMenuViewport(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class="absolute top-full left-0 isolate z-50 flex justify-center">
			<div class=cn!(NAVIGATION_MENU_VIEWPORT, class) data-slot="navigation-menu-viewport">
				{children()}
			</div>
		</div>
	}
}

#[derive(Clone, Copy)]
struct NavigationMenuItemCtx {
	open: Controllable<bool>,
}
