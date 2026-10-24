use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{FORM_ROW_BASE, FORM_ROW_CONTROL, FORM_ROW_DESCRIPTION, FORM_ROW_DIVIDER, FORM_ROW_HINT, FORM_ROW_LABEL, FORM_ROW_LABEL_COLUMN},
};

/// Label on the left, the value or its control on the right; they stack on a
/// phone. Mirrors the TS `FormRow`. A plain `div`, not a list item: its label
/// column has a fixed width so controls line up down the page.
///
/// `r#for` is the id of the control in `children`; with it the label is a
/// `<label for>`, without it a `<span>`, because a `<label>` that labels
/// nothing confuses assistive tech.
#[component]
pub fn FormRow(
	label: Element,
	r#for: Option<String>,
	description: Option<Element>,
	hint: Option<Element>,
	#[props(default = true)] divider: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	rsx! {
		FormRowFrame { label, r#for, description, hint, divider, class, {children} }
	}
}

/// The markup behind [`FormRow`]. `inline` is the TS `SettingsRow`'s
/// `data-inline`, which the TS spreads onto the row through `...props`; Dioxus
/// has no such spread here, so the settings row reaches the frame directly.
#[component]
pub(crate) fn FormRowFrame(
	label: Element,
	r#for: Option<String>,
	description: Option<Element>,
	hint: Option<Element>,
	divider: bool,
	inline: Option<bool>,
	class: String,
	children: Element,
) -> Element {
	let cls = cn!(FORM_ROW_BASE, if divider { FORM_ROW_DIVIDER } else { "" }, class);
	rsx! {
		div {
			class: cls,
			"data-slot": "form-row",
			"data-divider": if divider { "true" } else { "false" },
			"data-inline": inline.map(|on| if on { "true" } else { "false" }),
			div { class: FORM_ROW_LABEL_COLUMN, "data-slot": "form-row-label",
				if let Some(id) = r#for {
					label { class: FORM_ROW_LABEL, r#for: id, {label} }
				} else {
					span { class: FORM_ROW_LABEL, {label} }
				}
				if let Some(description) = description {
					span { class: FORM_ROW_DESCRIPTION, {description} }
				}
			}
			div { class: FORM_ROW_CONTROL, "data-slot": "form-row-control",
				{children}
				if let Some(hint) = hint {
					span { class: FORM_ROW_HINT, {hint} }
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	#[test]
	fn labels_the_control_it_wraps_when_given_for() {
		fn app() -> Element {
			rsx! {
				FormRow { label: rsx! { "Почта" }, r#for: "email",
					input { id: "email" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains(&format!("<label class=\"{FORM_ROW_LABEL}\" for=\"email\">Почта</label>")), "{html}");
		assert!(html.contains("id=\"email\""), "{html}");
	}

	#[test]
	fn renders_a_span_label_without_for() {
		fn app() -> Element {
			rsx! { FormRow { label: rsx! { "Имя" }, "JD" } }
		}
		let html = render(app);
		assert!(html.contains(&format!("<span class=\"{FORM_ROW_LABEL}\">Имя</span>")), "{html}");
		assert!(!html.contains("<label"), "{html}");
	}

	#[test]
	fn renders_description_under_the_label_and_hint_under_the_control() {
		fn app() -> Element {
			rsx! {
				FormRow { label: rsx! { "Name" }, description: rsx! { "Shown on invoices" }, hint: rsx! { "Max 40 characters" },
					span { "JD" }
				}
			}
		}
		let html = render(app);
		let label_col = html.find("data-slot=\"form-row-label\"").expect("label column");
		let control_col = html.find("data-slot=\"form-row-control\"").expect("control column");
		let description = html.find("Shown on invoices").expect("description");
		let control = html.find("JD").expect("control");
		let hint = html.find("Max 40 characters").expect("hint");
		assert!(label_col < description && description < control_col, "{html}");
		assert!(control_col < control && control < hint, "{html}");
	}

	#[test]
	fn leaves_description_and_hint_out_when_unset() {
		fn app() -> Element {
			rsx! { FormRow { label: rsx! { "A" }, "x" } }
		}
		assert!(!render(app).contains("text-caption"));
	}

	#[test]
	fn draws_a_divider_by_default() {
		fn app() -> Element {
			rsx! { FormRow { label: rsx! { "A" }, "x" } }
		}
		let html = render(app);
		assert!(html.contains("border-b border-border-subtle"), "{html}");
		assert!(html.contains("data-divider=\"true\""), "{html}");
		assert!(!html.contains("data-inline"), "{html}");
	}

	#[test]
	fn drops_the_divider_on_request() {
		fn app() -> Element {
			rsx! { FormRow { label: rsx! { "A" }, divider: false, "x" } }
		}
		let html = render(app);
		assert!(!html.contains("border-b"), "{html}");
		assert!(html.contains("data-divider=\"false\""), "{html}");
	}

	#[test]
	fn class_override_wins() {
		fn app() -> Element {
			rsx! { FormRow { label: rsx! { "A" }, class: "px-8", "x" } }
		}
		let html = render(app);
		assert!(html.contains("px-8"), "{html}");
		assert!(!html.contains("px-4"), "{html}");
	}
}
