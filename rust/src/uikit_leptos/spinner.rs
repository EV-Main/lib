use leptos::prelude::*;

use crate::cn;

#[component]
pub fn Spinner(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!("size-4 animate-spin", class);
	view! {
		<svg
			class=cls
			role="status"
			aria-label="Loading"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
		>
			<path d="M21 12a9 9 0 1 1-6.219-8.56" />
		</svg>
	}
}
