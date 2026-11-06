use dioxus::prelude::*;

use crate::{
	cn,
	ui::uikit::dioxus::{BADGE_BASE, Tone},
};

#[component]
pub fn Badge(#[props(default)] tone: Tone, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(BADGE_BASE, tone.as_class(), class);
	rsx! {
		span { class: cls, "data-slot": "badge", {children} }
	}
}

#[cfg(test)]
mod tests {
	use tailwind_fuse::AsTailwindClass;

	use super::*;
	use crate::ui::uikit::dioxus::{FILLED_FOCUS_RING, test_util::render};

	#[test]
	fn default_tone_is_neutral() {
		fn app() -> Element {
			rsx! { Badge { "hi" } }
		}
		let html = render(app);
		assert!(html.contains(Tone::Neutral.as_class()), "{html}");
		assert!(html.contains("hi"));
		assert!(html.contains("data-slot=\"badge\""));
	}

	/// The base size is text-caption; the tone's ink must not drop it.
	#[test]
	fn the_type_size_survives_the_tone_ink() {
		let classes = cn!(BADGE_BASE, Tone::Error.as_class());
		assert!(classes.contains("text-caption") && classes.contains("text-(color:--tone-ink)"), "{classes}");
	}

	#[test]
	fn a_badge_link_wears_the_offset_ring() {
		fn app() -> Element {
			rsx! { Badge { tone: Tone::Error, "x" } }
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
