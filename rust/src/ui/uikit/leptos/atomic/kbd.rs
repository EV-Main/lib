use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{KBD_BASE, KBD_GROUP_BASE, Slot},
};

#[component]
pub fn Kbd(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(KBD_BASE, class);
	view! {
		<kbd class=cls data-slot=Slot::Kbd.as_str()>
			{children()}
		</kbd>
	}
}

#[component]
pub fn KbdGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(KBD_GROUP_BASE, class);
	view! {
		<kbd class=cls data-slot=Slot::KbdGroup.as_str()>
			{children()}
		</kbd>
	}
}
