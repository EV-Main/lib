use leptos::prelude::*;

use crate::{cn, uikit_leptos::CONTAINER_BASE};

/// Page-width wrapper: centres content, caps it at `--page-max` and applies the
/// responsive `--page-px` gutter, so the page gutter and max width live in one
/// token-driven place.
#[component]
pub fn Container(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(CONTAINER_BASE, class);
	view! {
		<div class=cls data-slot="container">
			{children()}
		</div>
	}
}
