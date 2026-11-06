use ev_lib_classes::copy;
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{CHAT_TYPING_BASE, CHAT_TYPING_DOT, CHAT_TYPING_STILL, Slot},
};

/// The wait before the first token: three dots whose opacity ripples. Mirrors
/// the TS `ChatTyping`.
///
/// The ripple is CSS only (theme.css, keyed on `data-dot`); under reduced
/// motion the same sheet swaps the dots for the still "…". It is a status
/// region, so a screen reader hears `label` once when it appears.
#[component]
pub fn ChatTyping(#[prop(default = copy::CHAT_TYPING.to_owned(), into)] label: String, #[prop(optional, into)] class: String) -> impl IntoView {
	view! {
		<span
			role="status"
			aria-label=label
			data-slot=Slot::ChatTyping.as_str()
			class=cn!(CHAT_TYPING_BASE, class)
		>
			{(0..3)
				.map(|_| view! { <span aria-hidden="true" data-dot="" class=CHAT_TYPING_DOT></span> })
				.collect_view()}
			<span aria-hidden="true" data-still="" class=CHAT_TYPING_STILL>
				"…"
			</span>
		</span>
	}
}
