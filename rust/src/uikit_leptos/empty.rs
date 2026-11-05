use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{EMPTY, EMPTY_CONTENT, EMPTY_DESCRIPTION, EMPTY_HEADER, EMPTY_MEDIA_BASE, EMPTY_TITLE, EmptyMediaVariant},
};

#[component]
pub fn Empty(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(EMPTY, class);
	view! {
		<div class=cls data-slot="empty">
			{children()}
		</div>
	}
}

#[component]
pub fn EmptyHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(EMPTY_HEADER, class);
	view! {
		<div class=cls data-slot="empty-header">
			{children()}
		</div>
	}
}

#[component]
pub fn EmptyMedia(#[prop(optional)] variant: EmptyMediaVariant, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(EMPTY_MEDIA_BASE, variant.as_class(), class);
	view! {
		<div class=cls data-slot="empty-icon" data-variant=variant.as_ref().to_owned()>
			{children()}
		</div>
	}
}

#[component]
pub fn EmptyTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(EMPTY_TITLE, class);
	view! {
		<div class=cls data-slot="empty-title">
			{children()}
		</div>
	}
}

#[component]
pub fn EmptyDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(EMPTY_DESCRIPTION, class);
	view! {
		<div class=cls data-slot="empty-description">
			{children()}
		</div>
	}
}

#[component]
pub fn EmptyContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(EMPTY_CONTENT, class);
	view! {
		<div class=cls data-slot="empty-content">
			{children()}
		</div>
	}
}
