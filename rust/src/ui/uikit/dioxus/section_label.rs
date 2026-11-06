use dioxus::prelude::*;

pub use crate::ui::uikit::core::SectionLabelAs;
use crate::{
	cn,
	ui::uikit::dioxus::{SECTION_LABEL_BASE, SectionLabelTone},
};

/// The quiet label above a figure, a group or a section. Mirrors the TS
/// `SectionLabel` from `page-frame.tsx`.
#[component]
pub fn SectionLabel(#[props(default)] r#as: SectionLabelAs, #[props(default)] tone: SectionLabelTone, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(SECTION_LABEL_BASE, tone.as_class(), class);
	match r#as {
		SectionLabelAs::P => rsx! {
			p { class: cls, "data-slot": "section-label", {children} }
		},
		SectionLabelAs::Span => rsx! {
			span { class: cls, "data-slot": "section-label", {children} }
		},
		SectionLabelAs::Div => rsx! {
			div { class: cls, "data-slot": "section-label", {children} }
		},
		SectionLabelAs::H2 => rsx! {
			h2 { class: cls, "data-slot": "section-label", {children} }
		},
		SectionLabelAs::H3 => rsx! {
			h3 { class: cls, "data-slot": "section-label", {children} }
		},
		SectionLabelAs::H4 => rsx! {
			h4 { class: cls, "data-slot": "section-label", {children} }
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

	#[test]
	fn is_tracked_uppercase_text() {
		fn app() -> Element {
			rsx! { SectionLabel { "Общие" } }
		}
		let html = render(app);
		assert!(html.starts_with("<p"), "{html}");
		assert!(html.contains("data-slot=\"section-label\""), "{html}");
		assert!(html.contains("uppercase") && html.contains("tracking-widest") && html.contains("text-ink-soft"), "{html}");
		assert!(html.contains("Общие"), "{html}");
	}

	#[test]
	fn labels_in_two_tones_on_any_element() {
		fn app() -> Element {
			rsx! { SectionLabel { r#as: SectionLabelAs::H2, tone: SectionLabelTone::Accent, "Портфель" } }
		}
		let html = render(app);
		assert!(html.starts_with("<h2"), "{html}");
		assert!(html.contains("text-primary-ink") && !html.contains("text-ink-soft"), "{html}");
	}

	#[test]
	fn a_class_fuses_over_the_tone() {
		fn app() -> Element {
			rsx! { SectionLabel { class: "text-ink", "Группа" } }
		}
		let html = render(app);
		assert!(html.contains("text-ink") && !html.contains("text-ink-soft"), "{html}");
	}
}
