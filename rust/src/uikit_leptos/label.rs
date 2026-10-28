use leptos::prelude::*;

use crate::{cn, uikit_leptos::LABEL_BASE};

#[component]
pub fn Label(#[prop(optional, into)] class: String, #[prop(optional, into)] r#for: String, children: Children) -> impl IntoView {
	let cls = cn!(LABEL_BASE, class);
	view! {
		<label class=cls data-slot="label" for=r#for>
			{children()}
		</label>
	}
}
