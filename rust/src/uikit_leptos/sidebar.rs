use leptos::prelude::*;

use crate::{
	cn,
	uikit::{SIDEBAR_STYLE, sidebar_state},
	uikit_leptos::{
		ButtonVariant, SIDEBAR_CONTENT, SIDEBAR_FLAT, SIDEBAR_FOOTER, SIDEBAR_GROUP, SIDEBAR_GROUP_CONTENT, SIDEBAR_GROUP_LABEL, SIDEBAR_HEADER, SIDEBAR_INNER, SIDEBAR_INSET, SIDEBAR_MENU,
		SIDEBAR_MENU_BUTTON_BASE, SIDEBAR_MENU_ITEM, SIDEBAR_RAIL, SIDEBAR_SEPARATOR, SIDEBAR_TRIGGER, SIDEBAR_WRAPPER, SidebarCollapsible, SidebarSide, SidebarVariant, Size,
		primitives::{Controllable, use_controllable},
		sidebar_menu_button_size_class,
	},
};

// omitted: mobile sheet, cookie, kbd shortcut — see README Limitations

/// Shared sidebar state, provided by [`SidebarProvider`] and read by the parts.
#[derive(Clone, Copy)]
pub struct SidebarContext {
	open: Controllable<bool>,
}

impl SidebarContext {
	pub fn open(&self) -> bool {
		self.open.get()
	}

	pub fn set_open(&self, value: bool) {
		self.open.set(value);
	}

	pub fn toggle_sidebar(&self) {
		self.open.set(!self.open.get_untracked());
	}

	pub fn state(&self) -> &'static str {
		sidebar_state(self.open.get())
	}
}

pub fn use_sidebar() -> SidebarContext {
	expect_context::<SidebarContext>()
}

#[component]
pub fn SidebarProvider(
	#[prop(default = true)] default_open: bool,
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let open = use_controllable(open, default_open, on_open_change);
	provide_context(SidebarContext { open });
	let cls = cn!(SIDEBAR_WRAPPER, class);
	view! {
		<div data-slot="sidebar-wrapper" style=SIDEBAR_STYLE class=cls>
			{children()}
		</div>
	}
}

#[component]
pub fn Sidebar(
	#[prop(optional)] side: SidebarSide,
	#[prop(optional)] variant: SidebarVariant,
	#[prop(optional)] collapsible: SidebarCollapsible,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let ctx = use_sidebar();
	if collapsible == SidebarCollapsible::None {
		let cls = cn!(SIDEBAR_FLAT, class);
		return view! {
			<div class=cls data-slot="sidebar">
				{children()}
			</div>
		}
		.into_any();
	}
	let inner = cn!(SIDEBAR_INNER, class);
	view! {
		<div
			class="group peer text-ink hidden md:block"
			data-state=move || ctx.state()
			data-collapsible=move || collapsible.data(ctx.open()).to_owned()
			data-variant=variant.as_ref().to_owned()
			data-side=side.as_ref().to_owned()
			data-slot="sidebar"
		>
			<div class=inner data-slot="sidebar-inner">
				{children()}
			</div>
		</div>
	}
	.into_any()
}

#[component]
pub fn SidebarTrigger(#[prop(optional, into)] class: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
	let ctx = use_sidebar();
	let cls = cn!(SIDEBAR_TRIGGER, class);
	view! {
		<button
			type="button"
			data-sidebar="trigger"
			data-slot="sidebar-trigger"
			aria-label="Toggle Sidebar"
			class=cls
			on:click=move |_| ctx.toggle_sidebar()
		>
			{children.map(|c| c())}
			<span class="sr-only">"Toggle Sidebar"</span>
		</button>
	}
}

#[component]
pub fn SidebarRail(#[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = use_sidebar();
	let cls = cn!(SIDEBAR_RAIL, class);
	view! {
		<button
			type="button"
			data-sidebar="rail"
			data-slot="sidebar-rail"
			aria-label="Toggle Sidebar"
			tabindex="-1"
			title="Toggle Sidebar"
			class=cls
			on:click=move |_| ctx.toggle_sidebar()
		></button>
	}
}

#[component]
pub fn SidebarInset(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_INSET, class);
	view! {
		<main class=cls data-slot="sidebar-inset">
			{children()}
		</main>
	}
}

#[component]
pub fn SidebarHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_HEADER, class);
	view! {
		<div class=cls data-slot="sidebar-header" data-sidebar="header">
			{children()}
		</div>
	}
}

#[component]
pub fn SidebarFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_FOOTER, class);
	view! {
		<div class=cls data-slot="sidebar-footer" data-sidebar="footer">
			{children()}
		</div>
	}
}

#[component]
pub fn SidebarSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SIDEBAR_SEPARATOR, class);
	view! {
		<div role="separator" class=cls data-slot="sidebar-separator" data-sidebar="separator"></div>
	}
}

#[component]
pub fn SidebarContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_CONTENT, class);
	view! {
		<div class=cls data-slot="sidebar-content" data-sidebar="content">
			{children()}
		</div>
	}
}

#[component]
pub fn SidebarGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_GROUP, class);
	view! {
		<div class=cls data-slot="sidebar-group" data-sidebar="group">
			{children()}
		</div>
	}
}

#[component]
pub fn SidebarGroupLabel(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_GROUP_LABEL, class);
	view! {
		<div class=cls data-slot="sidebar-group-label" data-sidebar="group-label">
			{children()}
		</div>
	}
}

#[component]
pub fn SidebarGroupContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_GROUP_CONTENT, class);
	view! {
		<div class=cls data-slot="sidebar-group-content" data-sidebar="group-content">
			{children()}
		</div>
	}
}

#[component]
pub fn SidebarMenu(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_MENU, class);
	view! {
		<ul class=cls data-slot="sidebar-menu" data-sidebar="menu">
			{children()}
		</ul>
	}
}

#[component]
pub fn SidebarMenuItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SIDEBAR_MENU_ITEM, class);
	view! {
		<li class=cls data-slot="sidebar-menu-item" data-sidebar="menu-item">
			{children()}
		</li>
	}
}

/// A menu entry: runs `on_click` — the consumer's navigation or action.
///
/// The TS port reaches an `<a>` through `asChild`; this port has no `Slot`, so
/// route from the handler rather than nesting an `<A>`, which would put an
/// `<a>` inside this `<button>`.
#[component]
pub fn SidebarMenuButton(
	#[prop(default = ButtonVariant::Ghost)] variant: ButtonVariant,
	#[prop(default = Size::Sm)] size: Size,
	#[prop(optional, into)] is_active: Signal<bool>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let cls = cn!(SIDEBAR_MENU_BUTTON_BASE, variant.as_class(), sidebar_menu_button_size_class(size), class);
	view! {
		<button
			type="button"
			data-slot="sidebar-menu-button"
			data-sidebar="menu-button"
			data-size=size.as_ref().to_owned()
			data-active=move || is_active.get().to_string()
			class=cls
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			{children()}
		</button>
	}
}
