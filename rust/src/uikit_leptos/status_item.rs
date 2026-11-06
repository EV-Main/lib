use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{STATUS_DOT_BASE, STATUS_ITEM_BASE, STATUS_ITEM_VALUE, StatusTone},
};

/// The dot alone. Mirrors the TS `StatusDot`. It is hidden from assistive tech,
/// so the text next to it must carry the meaning.
#[component]
pub fn StatusDot(#[prop(optional)] tone: StatusTone, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(STATUS_DOT_BASE, tone.as_class(), class);
	view! {
		<span
			class=cls
			data-slot="status-dot"
			data-tone=tone.as_ref().to_owned()
			aria-hidden="true"
		></span>
	}
}

/// A dot and a short label: connection, sync, environment. Mirrors the TS
/// `StatusItem`; `value` is a figure after the label, e.g. a latency.
#[component]
pub fn StatusItem(#[prop(optional)] tone: StatusTone, #[prop(optional, into)] value: Option<ViewFn>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(STATUS_ITEM_BASE, class);
	view! {
		<span class=cls data-slot="status-item" data-tone=tone.as_ref().to_owned()>
			<StatusDot tone />
			<span>{children()}</span>
			{value.map(|value| view! { <span class=STATUS_ITEM_VALUE>{value.run()}</span> })}
		</span>
	}
}
