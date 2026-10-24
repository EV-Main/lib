use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{TOP_BAR_BASE, TOP_BAR_END, TOP_BAR_START},
};

/// The wide screen's utility bar over the content column: who and where, never
/// product nav (that is the rail's). Mirrors the TS `TopBar`. Layout only.
#[component]
pub fn TopBar(
	/// The app switcher, or the service's name.
	start: Option<Element>,
	/// Service stats, then an account menu last.
	end: Option<Element>,
	#[props(default)] class: String,
) -> Element {
	let cls = cn!(TOP_BAR_BASE, class);
	rsx! {
		header { class: cls, "data-slot": "top-bar",
			div { class: TOP_BAR_START, "data-slot": "top-bar-start", {start} }
			div { class: TOP_BAR_END, "data-slot": "top-bar-end", {end} }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	#[test]
	fn is_a_banner_landmark_with_both_sides() {
		fn app() -> Element {
			rsx! { TopBar { start: rsx! { "Казначейство" }, end: rsx! { "Анна Петрова" } } }
		}
		let html = render(app);
		assert!(html.starts_with("<header"), "a page header is the banner landmark: {html}");
		assert!(html.contains("data-slot=\"top-bar\""), "{html}");
		let start = html.find("data-slot=\"top-bar-start\"").expect("start slot");
		let end = html.find("data-slot=\"top-bar-end\"").expect("end slot");
		let service = html.find("Казначейство").expect("service name");
		let account = html.find("Анна Петрова").expect("account");
		assert!(start < service && service < end && end < account, "start, then end: {html}");
	}

	#[test]
	fn renders_empty_sides_when_given_nothing() {
		fn app() -> Element {
			rsx! { TopBar {} }
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"top-bar-start\""), "{html}");
		assert!(html.contains("data-slot=\"top-bar-end\""), "{html}");
	}

	#[test]
	fn class_override_wins() {
		fn app() -> Element {
			rsx! { TopBar { class: "h-10 px-3" } }
		}
		let html = render(app);
		assert!(html.contains("h-10") && html.contains("px-3"), "{html}");
		assert!(!html.contains("h-14") && !html.contains("px-8"), "{html}");
	}
}
