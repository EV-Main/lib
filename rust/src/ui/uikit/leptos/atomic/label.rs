use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{LABEL_BASE, Slot},
};

#[component]
pub fn Label(#[prop(optional, into)] class: String, #[prop(optional, into)] r#for: String, children: Children) -> impl IntoView {
	let cls = cn!(LABEL_BASE, class);
	view! {
		<label class=cls data-slot=Slot::Label.as_str() for=r#for>
			{children()}
		</label>
	}
}
