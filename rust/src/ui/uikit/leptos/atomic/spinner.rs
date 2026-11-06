use ev_lib_classes::{Icon, SPINNER, copy};
use leptos::prelude::*;

use crate::{cn, ui::uikit::leptos::primitives::icon_shapes};

#[component]
pub fn Spinner(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SPINNER, class);
	view! {
		<svg
			class=cls
			role="status"
			aria-label=copy::LOADING
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
		>
			{icon_shapes(Icon::Spinner)}
		</svg>
	}
}
