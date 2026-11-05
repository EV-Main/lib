use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{CARD, CARD_ACTION, CARD_CONTENT, CARD_DESCRIPTION, CARD_FOOTER, CARD_HEADER, CARD_TITLE},
};

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD, class);
	view! {
		<div class=cls data-slot="card">
			{children()}
		</div>
	}
}

#[component]
pub fn CardHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_HEADER, class);
	view! {
		<div class=cls data-slot="card-header">
			{children()}
		</div>
	}
}

#[component]
pub fn CardTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_TITLE, class);
	view! {
		<div class=cls data-slot="card-title">
			{children()}
		</div>
	}
}

#[component]
pub fn CardDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_DESCRIPTION, class);
	view! {
		<div class=cls data-slot="card-description">
			{children()}
		</div>
	}
}

#[component]
pub fn CardAction(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_ACTION, class);
	view! {
		<div class=cls data-slot="card-action">
			{children()}
		</div>
	}
}

#[component]
pub fn CardContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_CONTENT, class);
	view! {
		<div class=cls data-slot="card-content">
			{children()}
		</div>
	}
}

#[component]
pub fn CardFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_FOOTER, class);
	view! {
		<div class=cls data-slot="card-footer">
			{children()}
		</div>
	}
}
