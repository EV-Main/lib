use ev_lib_classes::{Icon, PAGINATION_CONTENT, PAGINATION_ELLIPSIS, PAGINATION_LINK_LABEL, PAGINATION_NEXT_LINK, PAGINATION_PREVIOUS_LINK, PAGINATION_ROOT, copy};
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{ButtonVariant, Size, button_classes, primitives::icon_shapes},
};

#[component]
pub fn Pagination(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(PAGINATION_ROOT, class);
	view! {
		<nav role="navigation" aria-label=copy::PAGINATION data-slot="pagination" class=cls>
			{children()}
		</nav>
	}
}

#[component]
pub fn PaginationContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(PAGINATION_CONTENT, class);
	view! {
		<ul data-slot="pagination-content" class=cls>
			{children()}
		</ul>
	}
}

#[component]
pub fn PaginationItem(children: Children) -> impl IntoView {
	view! { <li data-slot="pagination-item">{children()}</li> }
}

/// `href` is optional but wanted: an `<a>` without one is neither focusable nor
/// announced as a link, so a link given neither `href` nor `on_click` is inert.
#[component]
pub fn PaginationLink(
	#[prop(optional, into)] is_active: Signal<bool>,
	#[prop(optional)] size: Size,
	#[prop(default = true)] icon: bool,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] href: Option<String>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	#[prop(optional, into)] aria_label: Option<String>,
	children: Children,
) -> impl IntoView {
	link(is_active, size, icon, class, href, on_click, aria_label, children())
}

#[component]
pub fn PaginationPrevious(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] href: Option<String>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
	let chevron = view! {
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			{icon_shapes(Icon::ChevronLeft)}
		</svg>
		<span class=PAGINATION_LINK_LABEL>{copy::PAGINATION_PREVIOUS}</span>
	};
	let cls = cn!(PAGINATION_PREVIOUS_LINK, class);
	link(
		false.into(),
		Size::default(),
		false,
		cls,
		href,
		on_click,
		Some(copy::PAGINATION_PREVIOUS_PAGE.into()),
		chevron.into_any(),
	)
}

#[component]
pub fn PaginationNext(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] href: Option<String>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
	let chevron = view! {
		<span class=PAGINATION_LINK_LABEL>{copy::PAGINATION_NEXT}</span>
		<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
			{icon_shapes(Icon::ChevronRight)}
		</svg>
	};
	let cls = cn!(PAGINATION_NEXT_LINK, class);
	link(
		false.into(),
		Size::default(),
		false,
		cls,
		href,
		on_click,
		Some(copy::PAGINATION_NEXT_PAGE.into()),
		chevron.into_any(),
	)
}

#[component]
pub fn PaginationEllipsis(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(PAGINATION_ELLIPSIS, class);
	view! {
		<span aria-hidden="true" data-slot="pagination-ellipsis" class=cls>
			<svg class="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				{icon_shapes(Icon::Ellipsis)}
			</svg>
			<span class="sr-only">{copy::PAGINATION_MORE}</span>
		</span>
	}
}

// Previous/Next forward their optional `href`/`on_click` as-is, which the
// `PaginationLink` builder (it strips `Option`) cannot take.
#[allow(clippy::too_many_arguments)]
fn link(
	is_active: Signal<bool>,
	size: Size,
	icon: bool,
	class: String,
	href: Option<String>,
	on_click: Option<Callback<leptos::ev::MouseEvent>>,
	aria_label: Option<String>,
	children: AnyView,
) -> impl IntoView {
	let cls = move || {
		let variant = if is_active.get() { ButtonVariant::Outline } else { ButtonVariant::Ghost };
		button_classes(&variant, size, icon, None, &class)
	};
	view! {
		<a
			aria-current=move || is_active.get().then_some("page")
			aria-label=aria_label
			data-slot="pagination-link"
			data-active=move || is_active.get().to_string()
			class=cls
			href=href
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			{children}
		</a>
	}
}
