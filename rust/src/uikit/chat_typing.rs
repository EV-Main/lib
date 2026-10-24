use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{CHAT_TYPING_BASE, CHAT_TYPING_DOT, CHAT_TYPING_STILL},
};

/// The wait before the first token: three dots whose opacity ripples. Mirrors
/// the TS `ChatTyping`.
///
/// The ripple is CSS only (theme.css, keyed on `data-dot`); under reduced
/// motion the same sheet swaps the dots for the still "…". It is a status
/// region, so a screen reader hears `label` once when it appears.
#[component]
pub fn ChatTyping(#[props(default = String::from("Assistant is thinking"))] label: String, #[props(default)] class: String) -> Element {
	rsx! {
		span { role: "status", "aria-label": label, "data-slot": "chat-typing", class: cn!(CHAT_TYPING_BASE, class),
			for _ in 0..3 {
				span { "aria-hidden": "true", "data-dot": "", class: CHAT_TYPING_DOT }
			}
			span { "aria-hidden": "true", "data-still": "", class: CHAT_TYPING_STILL, "…" }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	#[test]
	fn a_named_status_with_three_decorative_dots_and_a_still_ellipsis() {
		fn app() -> Element {
			rsx! { ChatTyping {} }
		}
		let html = render(app);
		assert!(html.contains("role=\"status\"") && html.contains("aria-label=\"Assistant is thinking\""), "{html}");
		assert_eq!(html.matches("aria-hidden=\"true\" data-dot=\"\"").count(), 3, "{html}");
		assert!(html.contains("data-still=\"\" class=\"text-body leading-none\">…</span>"), "{html}");
	}

	#[test]
	fn takes_a_custom_label() {
		fn app() -> Element {
			rsx! { ChatTyping { label: "Ищу в документах" } }
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Ищу в документах\""), "{html}");
		assert!(!html.contains("Assistant is thinking"), "{html}");
	}
}
