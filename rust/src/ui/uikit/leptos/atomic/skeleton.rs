use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{SKELETON_BASE, Slot},
};

#[component]
pub fn Skeleton(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SKELETON_BASE, class);
	view! { <div class=cls data-slot=Slot::Skeleton.as_str()></div> }
}
