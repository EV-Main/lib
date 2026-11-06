use dioxus::prelude::*;
use ev_lib_classes::{Icon, SECTION_DESCRIPTOR, SECTION_DESCRIPTOR_SUMMARY};

use crate::{
	cn,
	ui::uikit::dioxus::{FIELD_DESCRIPTION, FIELD_TITLE, Slot, primitives::icon_shapes},
};

#[component]
pub fn SectionDescriptor(
	title: String,
	#[props(default = true)] icon: bool,
	#[props(default)] collapsible: bool,
	#[props(default)] default_open: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let cls = cn!(SECTION_DESCRIPTOR, class);
	if collapsible {
		let summary_cls = cn!(FIELD_TITLE, SECTION_DESCRIPTOR_SUMMARY);
		let body_cls = cn!(FIELD_DESCRIPTION, "mt-2");
		rsx! {
			details {
				class: cls,
				"data-slot": Slot::SectionDescriptor.as_str(),
				"data-collapsible": "true",
				open: default_open.then_some("true"),
				summary { class: summary_cls, "data-slot": Slot::SectionDescriptorTitle.as_str(),
					if icon {
						{info_glyph()}
					}
					span { {title} }
				}
				p { class: body_cls, "data-slot": Slot::SectionDescriptorBody.as_str(), {children} }
			}
		}
	} else {
		rsx! {
			section { class: cls, "data-slot": Slot::SectionDescriptor.as_str(),
				div { class: FIELD_TITLE, "data-slot": Slot::SectionDescriptorTitle.as_str(),
					if icon {
						{info_glyph()}
					}
					span { {title} }
				}
				p { class: FIELD_DESCRIPTION, "data-slot": Slot::SectionDescriptorBody.as_str(), {children} }
			}
		}
	}
}
fn info_glyph() -> Element {
	rsx! {
		svg {
			xmlns: "http://www.w3.org/2000/svg",
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "2",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			class: "size-4 shrink-0 text-accent-debug",
			"aria-hidden": "true",
			{icon_shapes(Icon::Info)}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

	#[test]
	fn renders_a_titled_section_with_body() {
		fn app() -> Element {
			rsx! {
				SectionDescriptor { title: "How withdrawals work", "Accept and queue." }
			}
		}
		let html = render(app);
		assert!(html.starts_with("<section"), "static variant is a section: {html}");
		assert!(html.contains("data-slot=\"section-descriptor\""), "{html}");
		assert!(html.contains("data-slot=\"section-descriptor-title\""), "{html}");
		assert!(html.contains("data-slot=\"section-descriptor-body\""), "{html}");
		assert!(html.contains("How withdrawals work"), "{html}");
		assert!(html.contains("Accept and queue."), "{html}");
	}

	#[test]
	fn collapsible_renders_a_details_disclosure() {
		fn app() -> Element {
			rsx! {
				SectionDescriptor { title: "Working with fund shares", collapsible: true, "body" }
			}
		}
		let html = render(app);
		assert!(html.contains("<details"), "{html}");
		assert!(html.contains("<summary"), "{html}");
		assert!(html.contains("data-collapsible=\"true\""), "{html}");
	}

	#[test]
	fn icon_shows_by_default_and_hides_when_disabled() {
		fn with_icon() -> Element {
			rsx! {
				SectionDescriptor { title: "t", "b" }
			}
		}
		fn without_icon() -> Element {
			rsx! {
				SectionDescriptor { title: "t", icon: false, "b" }
			}
		}
		assert!(render(with_icon).contains("<svg"), "icon on by default");
		assert!(!render(without_icon).contains("<svg"), "icon suppressed by icon:false");
	}
}
