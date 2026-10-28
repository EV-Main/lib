use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{Orientation, SEPARATOR_BASE},
};

#[component]
pub fn Separator(#[prop(optional)] orientation: Orientation, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SEPARATOR_BASE, orientation.as_class(), class);
	view! {
		<div
			role="separator"
			data-slot="separator"
			data-orientation=orientation.as_ref().to_owned()
			class=cls
		></div>
	}
}
