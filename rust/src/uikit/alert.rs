use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{ALERT_BASE, ALERT_DESCRIPTION, ALERT_TITLE, AlertVariant},
};

#[component]
pub fn Alert(#[props(default)] variant: AlertVariant, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(ALERT_BASE, variant.as_class(), class);
	rsx! {
		div { class: cls, "data-slot": "alert", role: "alert", {children} }
	}
}

#[component]
pub fn AlertTitle(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(ALERT_TITLE, class);
	rsx! {
		div { class: cls, "data-slot": "alert-title", {children} }
	}
}

#[component]
pub fn AlertDescription(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(ALERT_DESCRIPTION, class);
	rsx! {
		div { class: cls, "data-slot": "alert-description", {children} }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	#[test]
	fn default_variant_renders_base_and_slot() {
		fn app() -> Element {
			rsx! { Alert { "body" } }
		}
		let html = render(app);
		assert!(html.contains("bg-card"), "{html}");
		assert!(html.contains("data-slot=\"alert\""), "{html}");
		assert!(html.contains("role=\"alert\""), "{html}");
		assert!(html.contains("body"));
	}

	#[test]
	fn each_tone_wears_its_ink_and_chip_edge() {
		fn app() -> Element {
			rsx! {
				Alert { variant: AlertVariant::Error, "e" }
				Alert { variant: AlertVariant::Positive, "p" }
				Alert { variant: AlertVariant::Warn, "w" }
				Alert { variant: AlertVariant::Info, "i" }
			}
		}
		let html = render(app);
		for (ink, edge) in [
			("text-accent-error-ink", "border-chip-error-border"),
			("text-positive-ink", "border-chip-positive-border"),
			("text-accent-warn-ink", "border-chip-warn-border"),
			("text-accent-info-ink", "border-chip-info-border"),
		] {
			assert!(html.contains(ink) && html.contains(edge), "{ink} {edge}: {html}");
		}
	}

	/// The role tint stays on the root (icon + title inherit it); the description
	/// child is lifted to ink so body copy and nested controls hold the contrast floor.
	#[test]
	fn tinted_variants_keep_description_in_ink() {
		fn app() -> Element {
			rsx! {
				Alert { variant: AlertVariant::Error, AlertDescription { "d" } }
				Alert { variant: AlertVariant::Positive, AlertDescription { "s" } }
				Alert { variant: AlertVariant::Warn, AlertDescription { "w" } }
				Alert { variant: AlertVariant::Info, AlertDescription { "i" } }
			}
		}
		let html = render(app);
		assert_eq!(html.matches("*:data-[slot=alert-description]:text-ink-mid").count(), 4, "{html}");
		assert!(!html.contains("/90"), "{html}");
	}

	#[test]
	fn title_and_description_slots() {
		fn app() -> Element {
			rsx! {
				Alert {
					AlertTitle { "t" }
					AlertDescription { "d" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"alert-title\""), "{html}");
		assert!(html.contains("data-slot=\"alert-description\""), "{html}");
		assert!(html.contains("text-ink-soft"), "{html}");
	}
}
