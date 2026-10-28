use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{BREADCRUMB_ELLIPSIS, BREADCRUMB_ITEM, BREADCRUMB_LINK, BREADCRUMB_LIST, BREADCRUMB_PAGE, BREADCRUMB_SEPARATOR},
};

#[component]
pub fn Breadcrumb(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<nav class=class aria-label="breadcrumb" data-slot="breadcrumb">
			{children()}
		</nav>
	}
}

#[component]
pub fn BreadcrumbList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(BREADCRUMB_LIST, class);
	view! {
		<ol class=cls data-slot="breadcrumb-list">
			{children()}
		</ol>
	}
}

#[component]
pub fn BreadcrumbItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(BREADCRUMB_ITEM, class);
	view! {
		<li class=cls data-slot="breadcrumb-item">
			{children()}
		</li>
	}
}

/// `href` is optional but wanted: an `<a>` without one is neither focusable nor
/// announced as a link, so a crumb given no target stays inert decoration —
/// which is what [`BreadcrumbPage`] is for.
#[component]
pub fn BreadcrumbLink(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] href: Option<String>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	let cls = cn!(BREADCRUMB_LINK, class);
	view! {
		<a
			class=cls
			data-slot="breadcrumb-link"
			href=href
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			{children()}
		</a>
	}
}

#[component]
pub fn BreadcrumbPage(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(BREADCRUMB_PAGE, class);
	view! {
		<span
			class=cls
			data-slot="breadcrumb-page"
			role="link"
			aria-disabled="true"
			aria-current="page"
		>
			{children()}
		</span>
	}
}

#[component]
pub fn BreadcrumbSeparator(#[prop(optional, into)] class: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
	let cls = cn!(BREADCRUMB_SEPARATOR, class);
	view! {
		<li class=cls data-slot="breadcrumb-separator" role="presentation" aria-hidden="true">
			{match children {
				Some(children) => children(),
				None => {
					view! {
						<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
							<path d="m9 18 6-6-6-6" />
						</svg>
					}
						.into_any()
				}
			}}
		</li>
	}
}

#[component]
pub fn BreadcrumbEllipsis(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(BREADCRUMB_ELLIPSIS, class);
	view! {
		<span class=cls data-slot="breadcrumb-ellipsis" role="presentation" aria-hidden="true">
			<svg class="size-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
				<circle cx="12" cy="12" r="1" />
				<circle cx="19" cy="12" r="1" />
				<circle cx="5" cy="12" r="1" />
			</svg>
			<span class="sr-only">"More"</span>
		</span>
	}
}
