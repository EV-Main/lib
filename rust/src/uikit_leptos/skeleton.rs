use leptos::prelude::*;

use crate::{cn, uikit_leptos::SKELETON_BASE};

#[component]
pub fn Skeleton(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SKELETON_BASE, class);
	view! { <div class=cls data-slot="skeleton"></div> }
}
