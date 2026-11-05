use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{BADGE_BASE, BadgeVariant},
};

#[component]
pub fn Badge(#[prop(optional)] variant: BadgeVariant, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(BADGE_BASE, variant.as_class(), class);
	view! {
		<span class=cls data-slot="badge">
			{children()}
		</span>
	}
}
