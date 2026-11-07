use dioxus::prelude::*;

use crate::{
	cn,
	ui::uikit::dioxus::{ALERT_BASE, ALERT_DESCRIPTION, ALERT_TITLE, Slot, Tone},
};

#[component]
pub fn Alert(#[props(default)] tone: Tone, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(ALERT_BASE, tone.as_class(), class);
	rsx! {
		div { class: cls, "data-slot": Slot::Alert.as_str(), role: "alert", {children} }
	}
}

#[component]
pub fn AlertTitle(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(ALERT_TITLE, class);
	rsx! {
		div { class: cls, "data-slot": Slot::AlertTitle.as_str(), {children} }
	}
}

#[component]
pub fn AlertDescription(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(ALERT_DESCRIPTION, class);
	rsx! {
		div { class: cls, "data-slot": Slot::AlertDescription.as_str(), {children} }
	}
}

#[cfg(test)]
mod tests {
	use tailwind_fuse::AsTailwindClass;

	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

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
	fn the_root_wears_the_tone() {
		fn app() -> Element {
			rsx! {
				Alert { tone: Tone::Error, AlertDescription { "d" } }
			}
		}
		let html = render(app);
		assert!(html.contains(Tone::Error.as_class()), "{html}");
		assert!(html.contains("text-(color:--tone-ink)"), "{html}");
		assert!(html.contains("text-ink-mid"), "the description holds the body-text floor: {html}");
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
	}
}
