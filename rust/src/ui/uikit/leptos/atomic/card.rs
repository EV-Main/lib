use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{CARD, CARD_ACTION, CARD_CONTENT, CARD_DESCRIPTION, CARD_FOOTER, CARD_HEADER, CARD_TITLE, Slot},
};

#[component]
pub fn Card(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD, class);
	view! {
		<div class=cls data-slot=Slot::Card.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn CardHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_HEADER, class);
	view! {
		<div class=cls data-slot=Slot::CardHeader.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn CardTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_TITLE, class);
	view! {
		<div class=cls data-slot=Slot::CardTitle.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn CardDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_DESCRIPTION, class);
	view! {
		<div class=cls data-slot=Slot::CardDescription.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn CardAction(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_ACTION, class);
	view! {
		<div class=cls data-slot=Slot::CardAction.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn CardContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_CONTENT, class);
	view! {
		<div class=cls data-slot=Slot::CardContent.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn CardFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CARD_FOOTER, class);
	view! {
		<div class=cls data-slot=Slot::CardFooter.as_str()>
			{children()}
		</div>
	}
}
