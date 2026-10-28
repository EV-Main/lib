use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{ALERT_BASE, ALERT_DESCRIPTION, ALERT_TITLE, AlertVariant},
};

#[component]
pub fn Alert(#[prop(optional)] variant: AlertVariant, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ALERT_BASE, variant.as_class(), class);
	view! {
		<div class=cls data-slot="alert" role="alert">
			{children()}
		</div>
	}
}

#[component]
pub fn AlertTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ALERT_TITLE, class);
	view! {
		<div class=cls data-slot="alert-title">
			{children()}
		</div>
	}
}

#[component]
pub fn AlertDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ALERT_DESCRIPTION, class);
	view! {
		<div class=cls data-slot="alert-description">
			{children()}
		</div>
	}
}
