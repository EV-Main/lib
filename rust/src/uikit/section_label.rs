use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{SECTION_LABEL_BASE, SectionLabelTone},
};

/// The element a [`SectionLabel`] renders as. The TS `as` takes any element
/// type; Dioxus has no dynamic tag, so this lists the ones a label plausibly is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SectionLabelAs {
	#[default]
	P,
	Span,
	Div,
	H2,
	H3,
	H4,
}

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
	use crate::uikit::test_util::render;

	#[test]
	fn is_plain_body_s_text_not_tracked_uppercase() {
		fn app() -> Element {
			rsx! { SectionLabel { "Общие" } }
		}
		let html = render(app);
		assert!(html.starts_with("<p"), "{html}");
		assert!(html.contains("data-slot=\"section-label\""), "{html}");
		assert!(html.contains("text-body-s") && html.contains("text-ink-soft"), "{html}");
		assert!(!html.contains("uppercase") && !html.contains("tracking-widest"), "{html}");
		assert!(html.contains("Общие"), "{html}");
	}

	#[test]
	fn labels_in_two_tones_on_any_element() {
		fn app() -> Element {
			rsx! { SectionLabel { r#as: SectionLabelAs::H2, tone: SectionLabelTone::Accent, "Портфель" } }
		}
		let html = render(app);
		assert!(html.starts_with("<h2"), "{html}");
		assert!(html.contains("text-primary-ink") && html.contains("font-medium"), "{html}");
		assert!(!html.contains("text-ink-soft") && !html.contains("px-2"), "{html}");
		assert!(!html.contains("uppercase"), "{html}");
	}

	#[test]
	fn a_class_fuses_over_the_tone() {
		fn app() -> Element {
			rsx! { SectionLabel { class: "px-0", "Группа" } }
		}
		let html = render(app);
		assert!(html.contains("px-0") && !html.contains("px-2"), "{html}");
	}
}
