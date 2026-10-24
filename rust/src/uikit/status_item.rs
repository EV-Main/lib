use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{STATUS_DOT_BASE, STATUS_ITEM_BASE, STATUS_ITEM_VALUE, StatusTone},
};

/// The dot alone. Mirrors the TS `StatusDot`. It is hidden from assistive tech,
/// so the text next to it must carry the meaning.
#[component]
pub fn StatusDot(#[props(default)] tone: StatusTone, #[props(default)] class: String) -> Element {
	let cls = cn!(STATUS_DOT_BASE, tone.as_class(), class);
	rsx! {
		span { class: cls, "data-slot": "status-dot", "data-tone": tone.as_ref(), "aria-hidden": "true" }
	}
}

/// A dot and a short label: connection, sync, environment. Mirrors the TS
/// `StatusItem`; `value` is a figure after the label, e.g. a latency.
#[component]
pub fn StatusItem(#[props(default)] tone: StatusTone, value: Option<Element>, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(STATUS_ITEM_BASE, class);
	rsx! {
		span { class: cls, "data-slot": "status-item", "data-tone": tone.as_ref(),
			StatusDot { tone }
			span { {children} }
			if let Some(value) = value {
				span { class: STATUS_ITEM_VALUE, {value} }
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	#[test]
	fn shows_the_label_with_a_hidden_dot_of_the_tone_colour() {
		fn app() -> Element {
			rsx! { StatusItem { tone: StatusTone::Positive, "Подключено" } }
		}
		let html = render(app);
		assert!(html.contains("Подключено"), "{html}");
		assert!(html.contains("data-slot=\"status-dot\""), "{html}");
		assert!(html.contains("bg-positive"), "{html}");
		assert!(html.contains("aria-hidden=\"true\""), "{html}");
	}

	#[test]
	fn defaults_to_the_neutral_tone() {
		fn app() -> Element {
			rsx! { StatusItem { "Idle" } }
		}
		let html = render(app);
		assert!(html.starts_with("<span"), "{html}");
		assert!(html.contains("data-slot=\"status-item\" data-tone=\"neutral\""), "{html}");
		assert!(html.contains("bg-ink-soft"), "{html}");
	}

	#[test]
	fn renders_the_optional_value_after_the_label() {
		fn app() -> Element {
			rsx! { StatusItem { tone: StatusTone::Warn, value: rsx! { "49ms" }, "Latency" } }
		}
		let html = render(app);
		let label = html.find("Latency").expect("label");
		let value = html.find("49ms").expect("value");
		assert!(label < value, "{html}");
		assert!(html.contains(&format!("<span class=\"{STATUS_ITEM_VALUE}\">49ms</span>")), "{html}");
	}

	#[test]
	fn leaves_the_value_out_when_unset() {
		fn app() -> Element {
			rsx! { StatusItem { "Idle" } }
		}
		assert!(!render(app).contains("tabular-nums"));
	}

	#[test]
	fn exposes_the_dot_on_its_own() {
		fn app() -> Element {
			rsx! { StatusDot { tone: StatusTone::Error } }
		}
		let html = render(app);
		assert!(html.contains("bg-accent-error"), "{html}");
		assert!(html.contains("data-tone=\"error\""), "{html}");
	}

	#[test]
	fn each_tone_has_its_own_fill() {
		use strum::IntoEnumIterator;
		use tailwind_fuse::AsTailwindClass;
		let fills: Vec<String> = StatusTone::iter().map(|t| t.as_class().to_string()).collect();
		assert_eq!(fills, ["bg-positive", "bg-ink-soft", "bg-accent-warn", "bg-accent-error", "bg-ink-soft/50"]);
	}
}
