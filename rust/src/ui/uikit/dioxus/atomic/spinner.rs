use dioxus::prelude::*;
use ev_lib_classes::{Icon, SPINNER, copy};

use crate::{cn, ui::uikit::dioxus::primitives::icon_shapes};

#[component]
pub fn Spinner(#[props(default)] class: String) -> Element {
	let cls = cn!(SPINNER, class);
	rsx! {
		svg {
			class: cls,
			role: "status",
			"aria-label": copy::LOADING,
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "2",
			{icon_shapes(Icon::Spinner)}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

	#[test]
	fn spinner_has_status_and_base() {
		fn app() -> Element {
			rsx! { Spinner {} }
		}
		let html = render(app);
		assert!(html.contains("role=\"status\""), "{html}");
		assert!(html.contains("aria-label=\"Loading\""), "{html}");
		assert!(html.contains("animate-spin"), "{html}");
		assert!(html.contains("M21 12a9 9 0 1 1-6.219-8.56"), "{html}");
	}

	#[test]
	fn class_extends_base() {
		fn app() -> Element {
			rsx! {
				Spinner { class: "size-5" }
			}
		}
		let html = render(app);
		assert!(html.contains("size-5"), "{html}");
		assert!(!html.contains("size-4"), "override should drop base size-4: {html}");
	}
}
