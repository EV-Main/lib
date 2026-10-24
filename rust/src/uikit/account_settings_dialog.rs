use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		ACCOUNT_SETTINGS_DIALOG_CLOSE, ACCOUNT_SETTINGS_DIALOG_CONTENT, ACCOUNT_SETTINGS_DIALOG_DESCRIPTION, ACCOUNT_SETTINGS_DIALOG_ENTRY, ACCOUNT_SETTINGS_DIALOG_FOOTER,
		ACCOUNT_SETTINGS_DIALOG_HEADING, ACCOUNT_SETTINGS_DIALOG_LIST, ACCOUNT_SETTINGS_DIALOG_PANE, ACCOUNT_SETTINGS_DIALOG_PANEL, ACCOUNT_SETTINGS_DIALOG_SIDEBAR,
		ACCOUNT_SETTINGS_DIALOG_TABS, ACCOUNT_SETTINGS_DIALOG_TITLE, Dialog, DialogContent, DialogDescription, DialogTitle, IconButton, Tabs, TabsContent, TabsList, TabsOrientation,
		TabsTrigger,
		primitives::{use_controllable, use_stable_id},
	},
};

/// One entry of an [`AccountSettingsDialog`] rail. Mirrors the TS
/// `AccountSettingsSection`.
#[derive(Clone, PartialEq)]
pub struct AccountSettingsSection {
	pub id: String,
	pub label: Element,
	pub icon: Option<Element>,
	/// Fills the pane while this entry is active.
	pub content: Element,
}

/// The dialog's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AccountSettingsDialogLabels {
	/// `aria-label` of the close button; "Close".
	pub close: Option<String>,
}

/// The account modal: a vertical tab rail on the left, the active section on
/// the right. Mirrors the TS `AccountSettingsDialog`.
///
/// Sections are the caller's, so Profile and Security (or any other) are
/// built from [`SettingsSection`](super::SettingsSection) and
/// [`SettingsRow`](super::SettingsRow) inside `content`. The rail selects on
/// arrow, as [`Tabs`] does. With no `value` the first section starts active.
///
/// It is a [`Dialog`], so it keeps the kit's overlay limits: rendered inline
/// with `position: fixed`, no portal and no focus trap.
#[component]
pub fn AccountSettingsDialog(
	open: Option<bool>,
	#[props(default)] default_open: bool,
	on_open_change: Option<EventHandler<bool>>,
	/// The heading of the content pane; it also names the dialog.
	title: Element,
	description: Option<Element>,
	sections: Vec<AccountSettingsSection>,
	value: Option<String>,
	default_value: Option<String>,
	on_value_change: Option<EventHandler<String>>,
	/// Pinned under the rail entries: sign out, language.
	sidebar_footer: Option<Element>,
	#[props(default)] labels: AccountSettingsDialogLabels,
	#[props(default)] class: String,
) -> Element {
	// Owned here rather than left to the `Dialog`, because the close button
	// is an `IconButton`, which cannot reach the dialog's private state the
	// way `DialogClose` does.
	let open = use_controllable(open, default_open, on_open_change);
	let title_id = use_stable_id("account-settings-title");
	let description_id = use_stable_id("account-settings-description");
	let described_by = description.as_ref().map(|_| description_id.clone());
	let first = sections.first().map(|s| s.id.clone()).unwrap_or_default();
	let default_value = default_value.unwrap_or(first);
	let close_label = labels.close.unwrap_or_else(|| String::from("Close"));
	let cls = cn!(ACCOUNT_SETTINGS_DIALOG_CONTENT, class);
	rsx! {
		Dialog { open: open.get(), on_open_change: move |next| open.set(next),
			DialogContent { show_close_button: false, aria_labelledby: title_id.clone(), aria_describedby: described_by, class: cls,
				Tabs {
					orientation: TabsOrientation::Vertical,
					value,
					default_value,
					on_value_change,
					class: ACCOUNT_SETTINGS_DIALOG_TABS,
					div { "data-slot": "account-settings-dialog-sidebar", class: ACCOUNT_SETTINGS_DIALOG_SIDEBAR,
						TabsList { class: ACCOUNT_SETTINGS_DIALOG_LIST,
							for s in sections.iter() {
								TabsTrigger { key: "{s.id}", value: s.id.clone(), class: ACCOUNT_SETTINGS_DIALOG_ENTRY,
									if let Some(icon) = s.icon.clone() {
										{icon}
									}
									{s.label.clone()}
								}
							}
						}
						if let Some(footer) = sidebar_footer {
							div { "data-slot": "account-settings-dialog-footer", class: ACCOUNT_SETTINGS_DIALOG_FOOTER, {footer} }
						}
					}
					div { "data-slot": "account-settings-dialog-pane", class: ACCOUNT_SETTINGS_DIALOG_PANE,
						div { "data-slot": "account-settings-dialog-heading", class: ACCOUNT_SETTINGS_DIALOG_HEADING,
							DialogTitle { id: title_id, class: ACCOUNT_SETTINGS_DIALOG_TITLE, {title} }
							if let Some(description) = description {
								DialogDescription { id: description_id, class: ACCOUNT_SETTINGS_DIALOG_DESCRIPTION, {description} }
							}
						}
						for s in sections.iter() {
							TabsContent { key: "{s.id}", value: s.id.clone(), class: ACCOUNT_SETTINGS_DIALOG_PANEL, {s.content.clone()} }
						}
					}
				}
				IconButton { aria_label: close_label, class: ACCOUNT_SETTINGS_DIALOG_CLOSE, onclick: move |_| open.set(false),
					svg {
						class: "size-4",
						view_box: "0 0 24 24",
						fill: "none",
						stroke: "currentColor",
						stroke_width: "1.75",
						stroke_linecap: "round",
						stroke_linejoin: "round",
						"aria-hidden": "true",
						path { d: "M18 6 6 18M6 6l12 12" }
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::{render, render_after_click, render_after_keydown};

	fn sections() -> Vec<AccountSettingsSection> {
		vec![
			AccountSettingsSection {
				id: "profile".into(),
				label: rsx! { "Профиль" },
				icon: None,
				content: rsx! { p { "Панель профиля" } },
			},
			AccountSettingsSection {
				id: "security".into(),
				label: rsx! { "Безопасность" },
				icon: None,
				content: rsx! { p { "Панель безопасности" } },
			},
		]
	}

	/// The opening tag of the tab whose label is `label`.
	fn tab<'a>(html: &'a str, label: &str) -> &'a str {
		let end = html.find(&format!(">{label}<")).unwrap_or_else(|| panic!("no tab {label}: {html}"));
		let start = html[..end].rfind("<button").expect("tab button");
		let tag_end = start + html[start..].find('>').expect("tag end");
		&html[start..tag_end]
	}

	fn attr<'a>(html: &'a str, name: &str) -> &'a str {
		let at = html.find(&format!("{name}=\"")).unwrap_or_else(|| panic!("no {name}: {html}")) + name.len() + 2;
		&html[at..at + html[at..].find('"').expect("attr end")]
	}

	fn open_dialog() -> Element {
		rsx! {
			AccountSettingsDialog { open: true, title: rsx! { "Настройки аккаунта" }, sections: sections() }
		}
	}

	#[test]
	fn is_a_modal_named_by_its_title_with_the_first_section_shown() {
		let html = render(open_dialog);
		assert!(html.contains("role=\"dialog\""), "{html}");
		assert!(html.contains("aria-modal=\"true\""), "{html}");
		let title_id = attr(&html, "aria-labelledby");
		assert!(html.contains(&format!("<h2 id=\"{title_id}\"")), "{html}");
		assert!(html.contains(">Настройки аккаунта</h2>"), "{html}");
		assert!(html.contains("Панель профиля"), "{html}");
		assert!(!html.contains("Панель безопасности"), "{html}");
		assert!(!html.contains("aria-describedby"), "no description, no dangling reference: {html}");
	}

	#[test]
	fn is_described_by_its_description() {
		fn app() -> Element {
			rsx! {
				AccountSettingsDialog { open: true, title: rsx! { "Настройки" }, description: rsx! { "Имя, почта и вход" }, sections: sections() }
			}
		}
		let html = render(app);
		let description_id = attr(&html, "aria-describedby");
		assert!(html.contains(&format!("<p id=\"{description_id}\"")), "{html}");
		assert!(html.contains("Имя, почта и вход"), "{html}");
		assert_ne!(description_id, attr(&html, "aria-labelledby"), "{html}");
	}

	#[test]
	fn stays_closed_until_opened() {
		fn app() -> Element {
			rsx! {
				AccountSettingsDialog { title: rsx! { "Настройки аккаунта" }, sections: sections() }
			}
		}
		let html = render(app);
		assert!(!html.contains("role=\"dialog\""), "{html}");
		assert!(!html.contains("Панель профиля"), "{html}");
	}

	#[test]
	fn switches_the_pane_from_the_vertical_tab_rail() {
		let html = render(open_dialog);
		assert!(html.contains("aria-orientation=\"vertical\""), "{html}");
		// Clicks reach every element in mount order, so the last tab wins.
		let html = render_after_click(open_dialog);
		assert!(html.contains("Панель безопасности"), "{html}");
		assert!(!html.contains("Панель профиля"), "{html}");
		assert!(tab(&html, "Безопасность").contains("aria-selected=\"true\""), "{html}");
	}

	#[test]
	fn moves_between_tabs_with_the_arrow_keys() {
		let html = render_after_keydown(open_dialog, Key::ArrowDown);
		assert!(tab(&html, "Безопасность").contains("tabindex=\"0\""), "{html}");
		assert!(tab(&html, "Профиль").contains("tabindex=\"-1\""), "{html}");
	}

	#[test]
	fn shows_the_given_section_and_reports_switches_when_controlled() {
		fn app() -> Element {
			let mut log = use_signal(Vec::<String>::new);
			rsx! {
				AccountSettingsDialog {
					open: true,
					title: rsx! { "Настройки аккаунта" },
					sections: sections(),
					value: "security",
					on_value_change: move |v| log.write().push(v),
				}
				p { "log:{log.read().join(\",\")}" }
			}
		}
		let html = render(app);
		assert!(html.contains("Панель безопасности") && !html.contains("Панель профиля"), "{html}");
		let html = render_after_click(app);
		assert!(html.contains("log:profile,security<"), "{html}");
		assert!(
			html.contains("Панель безопасности") && !html.contains("Панель профиля"),
			"the owner did not move the value: {html}"
		);
	}

	/// The sweep clicks the scrim as well as the close button, so a dialog
	/// that closes through both reports twice; without the button it is once.
	#[test]
	fn closes_through_the_labelled_button_and_reports_it() {
		fn app() -> Element {
			let mut closes = use_signal(|| 0);
			rsx! {
				AccountSettingsDialog {
					open: true,
					on_open_change: move |open: bool| {
						if !open {
							closes += 1;
						}
					},
					title: rsx! { "Настройки аккаунта" },
					sections: sections(),
					labels: AccountSettingsDialogLabels { close: Some("Закрыть окно".into()) },
				}
				p { "closes:{closes}" }
			}
		}
		assert!(render(app).contains("aria-label=\"Закрыть окно\""));
		let html = render_after_click(app);
		assert!(html.contains("closes:2"), "{html}");
		assert!(html.contains("role=\"dialog\""), "controlled: the owner keeps it open: {html}");
	}

	#[test]
	fn names_the_close_button_close_by_default_and_hides_the_dialog_close() {
		let html = render(open_dialog);
		assert!(html.contains("aria-label=\"Close\""), "{html}");
		assert!(!html.contains("data-slot=\"dialog-close\""), "{html}");
		assert!(html.contains("absolute right-4 top-4"), "{html}");
	}

	#[test]
	fn closes_on_escape_when_uncontrolled() {
		fn app() -> Element {
			let mut reported = use_signal(String::new);
			rsx! {
				AccountSettingsDialog {
					default_open: true,
					on_open_change: move |open: bool| reported.set(open.to_string()),
					title: rsx! { "Настройки аккаунта" },
					sections: sections(),
				}
				p { "reported:{reported}" }
			}
		}
		let html = render_after_keydown(app, Key::Escape);
		assert!(!html.contains("role=\"dialog\""), "{html}");
		assert!(html.contains("reported:false"), "{html}");
	}

	#[test]
	fn renders_the_sidebar_footer() {
		fn app() -> Element {
			rsx! {
				AccountSettingsDialog {
					open: true,
					title: rsx! { "Настройки аккаунта" },
					sections: sections(),
					sidebar_footer: rsx! { button { r#type: "button", "Выйти" } },
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"account-settings-dialog-footer\""), "{html}");
		assert!(html.contains(">Выйти</button>"), "{html}");
		assert!(!render(open_dialog).contains("account-settings-dialog-footer"));
	}

	#[test]
	fn fuses_the_layout_over_the_dialog_and_tab_looks() {
		let html = render(open_dialog);
		let content = &html[html.find("role=\"dialog\"").expect("dialog")..];
		let content = &content[..content.find('>').expect("tag end")];
		assert!(content.contains("p-0") && !content.contains("p-6") && !content.contains(" grid "), "{content}");
		assert!(tab(&html, "Профиль").contains("data-[state=active]:bg-active"), "{html}");
		assert!(tab(&html, "Профиль").contains("border-0") && !tab(&html, "Профиль").contains("border-b "), "{html}");
		assert!(html.contains("class=\"font-display text-h1 font-light text-ink\" data-slot=\"dialog-title\""), "{html}");
	}
}
