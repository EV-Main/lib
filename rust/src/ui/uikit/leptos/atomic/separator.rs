use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{Orientation, SEPARATOR_BASE, Slot, separator_orientation_class},
};

#[component]
pub fn Separator(#[prop(optional)] orientation: Orientation, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SEPARATOR_BASE, separator_orientation_class(orientation), class);
	view! {
		<div
			role="separator"
			data-slot=Slot::Separator.as_str()
			data-orientation=orientation.as_ref().to_owned()
			class=cls
		></div>
	}
}
