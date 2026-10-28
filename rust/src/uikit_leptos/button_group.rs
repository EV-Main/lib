use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{BUTTON_GROUP_BASE, BUTTON_GROUP_SEPARATOR_BASE, BUTTON_GROUP_TEXT_BASE, ButtonGroupOrientation},
};

#[component]
pub fn ButtonGroup(#[prop(optional)] orientation: ButtonGroupOrientation, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(BUTTON_GROUP_BASE, orientation.as_class(), class);
	view! {
		<div
			role="group"
			data-slot="button-group"
			data-orientation=orientation.as_ref().to_owned()
			class=cls
		>
			{children()}
		</div>
	}
}

#[component]
pub fn ButtonGroupText(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(BUTTON_GROUP_TEXT_BASE, class);
	view! { <div class=cls>{children()}</div> }
}

#[component]
pub fn ButtonGroupSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(BUTTON_GROUP_SEPARATOR_BASE, class);
	view! {
		<div
			data-slot="button-group-separator"
			data-orientation="vertical"
			role="separator"
			class=cls
		></div>
	}
}
