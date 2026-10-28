use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{KBD_BASE, KBD_GROUP_BASE},
};

#[component]
pub fn Kbd(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(KBD_BASE, class);
	view! {
		<kbd class=cls data-slot="kbd">
			{children()}
		</kbd>
	}
}

#[component]
pub fn KbdGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(KBD_GROUP_BASE, class);
	view! {
		<kbd class=cls data-slot="kbd-group">
			{children()}
		</kbd>
	}
}
