use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{BADGE_BASE, BadgeVariant},
};

#[component]
pub fn Badge(#[props(default)] variant: BadgeVariant, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(BADGE_BASE, variant.as_class(), class);
	rsx! {
		span { class: cls, "data-slot": "badge", {children} }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::{FILLED_FOCUS_RING, test_util::render};

	#[test]
	fn default_variant_renders_neutral() {
		fn app() -> Element {
			rsx! { Badge { "hi" } }
		}
		let html = render(app);
		assert!(html.contains("bg-chip-neutral-bg"), "{html}");
		assert!(html.contains("hi"));
		assert!(html.contains("data-slot=\"badge\""));
	}

	#[test]
	fn each_tone_wears_its_chip_tokens_and_ink() {
		use strum::IntoEnumIterator;

		for (variant, tone, ink) in [
			(BadgeVariant::Primary, "primary", "text-primary-ink"),
			(BadgeVariant::Positive, "positive", "text-positive-ink"),
			(BadgeVariant::Info, "info", "text-accent-info-ink"),
			(BadgeVariant::Warn, "warn", "text-accent-warn-ink"),
			(BadgeVariant::Error, "error", "text-accent-error-ink"),
		] {
			let classes = cn!(BADGE_BASE, variant.as_class());
			assert!(classes.contains(&format!("bg-chip-{tone}-bg")), "{classes}");
			assert!(classes.contains(&format!("border-chip-{tone}-border")), "{classes}");
			assert!(classes.contains(ink), "{classes}");
		}
		assert_eq!(BadgeVariant::iter().count(), 6);
	}

	/// The base size is text-caption; a caller's colour must not drop it.
	#[test]
	fn the_type_size_survives_the_tone_ink() {
		let classes = cn!(BADGE_BASE, BadgeVariant::Error.as_class());
		assert!(classes.contains("text-caption") && classes.contains("text-accent-error-ink"), "{classes}");
	}

	#[test]
	fn a_badge_link_wears_the_offset_ring() {
		fn app() -> Element {
			rsx! { Badge { variant: BadgeVariant::Error, "x" } }
		}
		let html = render(app);
		assert!(html.contains(FILLED_FOCUS_RING), "{html}");
		assert!(!html.contains("focus-visible:ring-[3px]"), "{html}");
	}

	#[test]
	fn class_override_wins() {
		fn app() -> Element {
			rsx! {
				Badge { class: "px-6", "x" }
			}
		}
		let html = render(app);
		assert!(html.contains("px-6"));
		assert!(!html.contains("px-1.5"), "override should drop base px-1.5: {html}");
	}
}
