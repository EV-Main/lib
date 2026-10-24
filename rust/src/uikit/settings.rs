use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		SETTINGS_LAYOUT, SETTINGS_MAIN, SETTINGS_NAV, SETTINGS_ROW_INLINE, SETTINGS_SECTION_ACTION, SETTINGS_SECTION_BASE, SETTINGS_SECTION_BODY, SETTINGS_SECTION_BODY_FLUSH,
		SETTINGS_SECTION_BODY_PADDED, SETTINGS_SECTION_DESCRIPTION, SETTINGS_SECTION_HEADER, SETTINGS_SECTION_HEADER_FLUSH, SETTINGS_SECTION_HEADING, SETTINGS_SECTION_TITLE,
		SettingsSectionTone,
		form_row::FormRowFrame,
		primitives::{has_content, use_stable_id},
	},
};

/// Rail on the left, cards in the main column. Mirrors the TS `SettingsLayout`.
/// The main column is a `div`, not a `<main>`: the app shell already renders
/// the page's `<main>`, and a page has only one.
#[component]
pub fn SettingsLayout(nav: Element, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(SETTINGS_LAYOUT, class);
	rsx! {
		div { class: cls, "data-slot": "settings-layout",
			div { class: SETTINGS_NAV, "data-slot": "settings-nav", {nav} }
			div { class: SETTINGS_MAIN, "data-slot": "settings-main", {children} }
		}
	}
}

/// A settings card: title, a line saying what it is for, then the body.
/// Mirrors the TS `SettingsSection`. The title names the `<section>`, so the
/// card is a landmark region. `action` is a top-right control; `flush` lets
/// rows run edge to edge with their own hairlines.
#[component]
pub fn SettingsSection(
	title: Element,
	description: Option<Element>,
	action: Option<Element>,
	#[props(default)] tone: SettingsSectionTone,
	#[props(default)] flush: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let heading_id = use_stable_id("settings-section");
	let cls = cn!(SETTINGS_SECTION_BASE, tone.as_class(), class);
	let header = cn!(SETTINGS_SECTION_HEADER, if flush { SETTINGS_SECTION_HEADER_FLUSH } else { "" });
	let body = cn!(SETTINGS_SECTION_BODY, if flush { SETTINGS_SECTION_BODY_FLUSH } else { SETTINGS_SECTION_BODY_PADDED });
	rsx! {
		section { class: cls, "data-slot": "settings-section", "data-tone": tone.as_ref(), "aria-labelledby": heading_id.clone(),
			header { class: header,
				div { class: SETTINGS_SECTION_HEADING,
					h3 { id: heading_id, class: SETTINGS_SECTION_TITLE, {title} }
					if let Some(description) = description {
						p { class: SETTINGS_SECTION_DESCRIPTION, {description} }
					}
				}
				if let Some(action) = action {
					div { class: SETTINGS_SECTION_ACTION, {action} }
				}
			}
			// An empty body would still draw the flush hairline under the header.
			if has_content(&children) {
				div { class: body, {children} }
			}
		}
	}
}

/// A form row for settings. Mirrors the TS `SettingsRow`. `inline` keeps label
/// and control on one line at every width, control at the end: for a switch or
/// a button.
#[component]
pub fn SettingsRow(
	label: Element,
	r#for: Option<String>,
	description: Option<Element>,
	hint: Option<Element>,
	#[props(default = true)] divider: bool,
	#[props(default)] inline: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let cls = cn!(if inline { SETTINGS_ROW_INLINE } else { "" }, class);
	rsx! {
		FormRowFrame { label, r#for, description, hint, divider, inline, class: cls, {children} }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	#[test]
	fn puts_the_rail_and_the_main_column_side_by_side() {
		fn app() -> Element {
			rsx! {
				SettingsLayout { nav: rsx! { nav { "aria-label": "Sections", "Account" } },
					p { "Тело" }
				}
			}
		}
		let html = render(app);
		let nav = html.find("data-slot=\"settings-nav\"").expect("nav column");
		let rail = html.find("aria-label=\"Sections\"").expect("rail");
		let main = html.find("data-slot=\"settings-main\"").expect("main column");
		let body = html.find("Тело").expect("body");
		assert!(nav < rail && rail < main && main < body, "{html}");
		assert!(!html.contains("<main"), "{html}");
		assert!(html.contains("lg:flex-row"), "{html}");
	}

	#[test]
	fn names_a_section_by_its_title_and_shows_description_and_action() {
		fn app() -> Element {
			rsx! {
				SettingsSection { title: rsx! { "Аккаунты" }, description: rsx! { "Switch accounts" },
					action: rsx! { button { r#type: "button", "Create team" } },
					span { "row" }
				}
			}
		}
		let html = render(app);
		let labelled = html.split("aria-labelledby=\"").nth(1).and_then(|s| s.split('"').next()).expect("aria-labelledby");
		assert!(html.contains(&format!("<h3 id=\"{labelled}\" class=\"{SETTINGS_SECTION_TITLE}\">Аккаунты</h3>")), "{html}");
		assert!(html.contains("Switch accounts"), "{html}");
		assert!(
			html.contains(&format!("<div class=\"{SETTINGS_SECTION_ACTION}\"><button type=\"button\">Create team</button></div>")),
			"{html}"
		);
		assert!(html.contains("data-tone=\"default\""), "{html}");
		assert!(html.contains("border-border"), "{html}");
	}

	#[test]
	fn two_sections_get_distinct_heading_ids() {
		fn app() -> Element {
			rsx! {
				SettingsSection { title: rsx! { "A" } }
				SettingsSection { title: rsx! { "B" } }
			}
		}
		let html = render(app);
		let ids: Vec<&str> = html.split("aria-labelledby=\"").skip(1).filter_map(|s| s.split('"').next()).collect();
		assert_eq!(ids.len(), 2, "{html}");
		assert_ne!(ids[0], ids[1], "{html}");
	}

	#[test]
	fn marks_the_danger_zone_with_the_error_edge() {
		fn app() -> Element {
			rsx! { SettingsSection { title: rsx! { "Delete account" }, tone: SettingsSectionTone::Danger } }
		}
		let html = render(app);
		assert!(html.contains("border-accent-error"), "{html}");
		assert!(!html.contains("border-border"), "{html}");
		assert!(html.contains("data-tone=\"danger\""), "{html}");
	}

	#[test]
	fn leaves_the_body_out_without_children() {
		fn app() -> Element {
			rsx! { SettingsSection { title: rsx! { "Empty" }, flush: true } }
		}
		let html = render(app);
		assert!(!html.contains("border-t"), "{html}");
		assert!(html.ends_with("</header></section>"), "{html}");
	}

	#[test]
	fn a_flush_body_runs_edge_to_edge_and_a_plain_one_is_padded() {
		fn flush() -> Element {
			rsx! { SettingsSection { title: rsx! { "A" }, flush: true, "row" } }
		}
		fn padded() -> Element {
			rsx! { SettingsSection { title: rsx! { "A" }, "row" } }
		}
		let html = render(flush);
		assert!(html.contains("<div class=\"flex flex-col border-t border-border-subtle\">row</div>"), "{html}");
		assert!(html.contains("pb-3"), "{html}");
		let html = render(padded);
		assert!(html.contains("<div class=\"flex flex-col gap-4 px-4 pb-4\">row</div>"), "{html}");
		assert!(!html.contains("pb-3"), "{html}");
	}

	#[test]
	fn lays_settings_rows_out_like_form_rows_inline_for_toggles() {
		fn app() -> Element {
			rsx! {
				SettingsSection { title: rsx! { "Notifications" }, flush: true,
					SettingsRow { label: rsx! { "Email alerts" }, inline: true,
						input { r#type: "checkbox", "aria-label": "Email alerts toggle" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"form-row\""), "{html}");
		assert!(html.contains("data-inline=\"true\""), "{html}");
		assert!(html.contains("aria-label=\"Email alerts toggle\""), "{html}");
		// The inline row must win the direction over the form row's stacked base.
		assert!(html.contains("flex-row items-center"), "{html}");
		assert!(!html.contains("flex flex-col gap-1.5"), "{html}");
	}

	#[test]
	fn a_plain_settings_row_stacks_and_says_so() {
		fn app() -> Element {
			rsx! { SettingsRow { label: rsx! { "Name" }, "JD" } }
		}
		let html = render(app);
		assert!(html.contains("data-inline=\"false\""), "{html}");
		assert!(html.contains("flex flex-col gap-1.5"), "{html}");
		assert!(!html.contains("justify-between"), "{html}");
	}
}
