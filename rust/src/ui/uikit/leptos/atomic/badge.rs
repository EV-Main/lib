use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{BADGE_BASE, Slot, Tone},
};

#[component]
pub fn Badge(#[prop(optional)] tone: Tone, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(BADGE_BASE, tone.as_class(), class);
	view! {
		<span class=cls data-slot=Slot::Badge.as_str()>
			{children()}
		</span>
	}
}
