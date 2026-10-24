use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		WORKSPACE_TAB_BASE, WORKSPACE_TAB_CLOSE, WORKSPACE_TAB_LABEL, WORKSPACE_TAB_SELECTED, WORKSPACE_TAB_SPACER, WORKSPACE_TAB_TRIGGER, WORKSPACE_TABS_BASE,
		primitives::{Controllable, RovingFocus, RovingOrientation, use_controllable, use_roving_focus, use_roving_item},
	},
};

/// A row of open workspaces. Mirrors the TS `WorkspaceTabs`.
///
/// Arrow keys move focus (Home/End jump) and wrap; selecting is Enter, Space
/// or a click, so moving through the row never swaps the screen. That is
/// manual activation, unlike [`Tabs`](super::Tabs), which selects on arrow:
/// opening a workspace is too heavy to happen on every key press.
///
/// The TS `onKeyDown` and the rest of the `div` props are not ported: the kit
/// spreads no props.
#[component]
pub fn WorkspaceTabs(
	value: Option<String>,
	#[props(default)] default_value: String,
	on_value_change: Option<EventHandler<String>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let value = use_controllable(value, default_value, on_value_change);
	let roving = use_roving_focus(RovingOrientation::Horizontal);
	use_context_provider(|| WorkspaceTabsCtx { value, roving });
	let cls = cn!(WORKSPACE_TABS_BASE, class);
	rsx! {
		div { role: "tablist", "aria-orientation": "horizontal", "data-slot": "workspace-tabs", class: cls, {children} }
	}
}

/// One open workspace in a [`WorkspaceTabs`] row. Mirrors the TS `WorkspaceTab`.
///
/// `icon` replaces the default layers glyph. `on_close` adds a close button,
/// and Delete or Backspace on the focused tab closes it too. `class` lands on
/// the outer pill, as in TS. The TS `onClick`/`onKeyDown` and the rest of the
/// `button` props are not ported; `on_value_change` on the row reports clicks.
#[component]
pub fn WorkspaceTab(
	value: String,
	icon: Option<Element>,
	on_close: Option<EventHandler<()>>,
	#[props(default = String::from("Close tab"))] close_label: String,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let ctx = use_context::<WorkspaceTabsCtx>();
	let roving = ctx.roving;
	let id = use_roving_item(roving, value.clone());
	let active = ctx.value.get();
	let selected = active == value;
	let tab_stop = roving.is_tab_stop(&value, &active);
	let cls = cn!(WORKSPACE_TAB_BASE, if selected { WORKSPACE_TAB_SELECTED } else { "" }, class);
	let glyph = icon.unwrap_or_else(|| rsx! { LayersIcon {} });
	let on_click_value = value.clone();
	rsx! {
		div { "data-slot": "workspace-tab", "data-state": if selected { "active" } else { "inactive" }, class: cls,
			button {
				r#type: "button",
				role: "tab",
				"data-slot": "workspace-tab-trigger",
				"aria-selected": if selected { "true" } else { "false" },
				tabindex: if tab_stop { "0" } else { "-1" },
				class: WORKSPACE_TAB_TRIGGER,
				onmounted: move |e: MountedEvent| roving.attach(id, e.data()),
				onclick: move |_| ctx.value.set(on_click_value.clone()),
				// The key is handled on the tab, not on the row as in TS: the
				// tab that hears it is the focused one, which TS has to look up
				// through `document.activeElement`.
				onkeydown: move |e: KeyboardEvent| {
					if let Some(close) = on_close.filter(|_| matches!(e.key(), Key::Delete | Key::Backspace)) {
						e.prevent_default();
						if let Some(next) = roving.neighbour(&value) {
							roving.focus(&next);
						}
						close.call(());
					} else if let Some(next) = roving.next(&e, &value) {
						e.prevent_default();
						roving.focus(&next);
					}
				},
				{glyph}
				span { class: WORKSPACE_TAB_LABEL, {children} }
			}
			if let Some(close) = on_close {
				button {
					r#type: "button",
					"data-slot": "workspace-tab-close",
					"aria-label": close_label,
					tabindex: "-1",
					class: WORKSPACE_TAB_CLOSE,
					onclick: move |_| close.call(()),
					CloseIcon {}
				}
			} else {
				span { class: WORKSPACE_TAB_SPACER, "aria-hidden": "true" }
			}
		}
	}
}

#[derive(Clone, Copy)]
struct WorkspaceTabsCtx {
	value: Controllable<String>,
	roving: RovingFocus,
}

/// The TS `UiIcon` frame: its default `size-4` and the 1.75 stroke.
#[component]
fn Glyph(paths: &'static [&'static str]) -> Element {
	rsx! {
		svg {
			class: "size-4",
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.75",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			"aria-hidden": "true",
			for d in paths {
				path { d: *d }
			}
		}
	}
}

#[component]
fn LayersIcon() -> Element {
	rsx! {
		Glyph {
			paths: &[
				"m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z",
				"m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65M22 12.65l-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65",
			],
		}
	}
}

#[component]
fn CloseIcon() -> Element {
	rsx! {
		Glyph { paths: &["M18 6 6 18M6 6l12 12"] }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::{render, render_after_click, render_after_keydown, render_after_keys};

	/// The opening tag of the tab button whose title is `title`, so a test reads
	/// that tab's attributes and no other's.
	fn tab<'a>(html: &'a str, title: &str) -> &'a str {
		let end = html.find(&format!(">{title}<")).unwrap_or_else(|| panic!("no tab {title}: {html}"));
		let start = html[..end].rfind("<button").expect("tab button");
		let tag_end = start + html[start..].find('>').expect("tag end");
		&html[start..tag_end]
	}

	/// The TS `setup()`: three tabs, the middle one closable, and a counter of
	/// its closes rendered as text so a test can read it back.
	fn three_tabs() -> Element {
		let mut closed = use_signal(|| 0);
		rsx! {
			WorkspaceTabs { default_value: "a",
				WorkspaceTab { value: "a", "Торговля" }
				WorkspaceTab { value: "b", on_close: move |_| closed += 1, close_label: "Закрыть Портфель", "Портфель" }
				WorkspaceTab { value: "c", "Отчёты" }
			}
			p { "closed:{closed}" }
		}
	}

	#[test]
	fn exposes_a_tablist_whose_selected_tab_is_the_only_tab_stop() {
		let html = render(three_tabs);
		assert!(html.contains("role=\"tablist\""), "{html}");
		assert!(html.contains("aria-orientation=\"horizontal\""), "{html}");
		assert!(tab(&html, "Торговля").contains("aria-selected=\"true\""), "{html}");
		assert!(tab(&html, "Торговля").contains("tabindex=\"0\""), "{html}");
		assert!(tab(&html, "Портфель").contains("aria-selected=\"false\""), "{html}");
		assert!(tab(&html, "Портфель").contains("tabindex=\"-1\""), "{html}");
		assert!(tab(&html, "Отчёты").contains("tabindex=\"-1\""), "{html}");
		assert_eq!(html.matches("tabindex=\"0\"").count(), 1, "{html}");
	}

	#[test]
	fn gives_the_first_tab_the_tab_stop_when_no_tab_is_selected() {
		fn app() -> Element {
			rsx! {
				WorkspaceTabs { default_value: "gone",
					WorkspaceTab { value: "a", "Торговля" }
					WorkspaceTab { value: "b", "Портфель" }
				}
			}
		}
		let html = render(app);
		assert!(tab(&html, "Торговля").contains("tabindex=\"0\""), "{html}");
		assert!(tab(&html, "Портфель").contains("tabindex=\"-1\""), "{html}");
		assert!(!html.contains("aria-selected=\"true\""), "{html}");
	}

	/// The accessible name of a tab is its title alone: the glyph is hidden,
	/// and the close button is a sibling, not part of the tab.
	#[test]
	fn names_the_tab_by_its_title_and_the_close_button_by_its_label() {
		let html = render(three_tabs);
		assert!(html.contains("aria-hidden=\"true\""), "{html}");
		assert!(html.contains("aria-label=\"Закрыть Портфель\""), "{html}");
		assert_eq!(html.matches("aria-label=").count(), 1, "only the closable tab has a close button: {html}");
		assert_eq!(html.matches("role=\"tab\"").count(), 3, "{html}");
		// The close button is out of the tab order: the row is one tab stop.
		let close = &html[html.find("data-slot=\"workspace-tab-close\"").expect("close")..];
		assert!(close[..close.find('>').expect("tag end")].contains("tabindex=\"-1\""), "{html}");
	}

	#[test]
	fn selects_a_tab_on_click() {
		// Clicks reach every element in mount order, so the last tab wins.
		let html = render_after_click(three_tabs);
		assert!(tab(&html, "Отчёты").contains("aria-selected=\"true\""), "{html}");
		assert!(tab(&html, "Отчёты").contains("tabindex=\"0\""), "{html}");
		assert!(tab(&html, "Торговля").contains("aria-selected=\"false\""), "{html}");
	}

	#[test]
	fn reports_the_clicked_tab_and_stays_put_when_controlled() {
		fn app() -> Element {
			let mut picked = use_signal(String::new);
			rsx! {
				WorkspaceTabs { value: "a", on_value_change: move |v| picked.set(v),
					WorkspaceTab { value: "a", "Торговля" }
					WorkspaceTab { value: "c", "Отчёты" }
				}
				p { "picked:{picked}" }
			}
		}
		let html = render_after_click(app);
		assert!(html.contains("picked:c"), "{html}");
		assert!(tab(&html, "Торговля").contains("aria-selected=\"true\""), "the owner did not move the value: {html}");
	}

	#[test]
	fn arrows_and_home_end_move_focus_without_selecting() {
		for key in [Key::ArrowLeft, Key::ArrowRight, Key::Home, Key::End] {
			let html = render_after_keydown(three_tabs, key.clone());
			assert!(tab(&html, "Торговля").contains("aria-selected=\"true\""), "{key:?}: {html}");
			assert!(tab(&html, "Торговля").contains("tabindex=\"0\""), "{key:?}: {html}");
		}
	}

	#[test]
	fn closes_through_the_labelled_button() {
		// The sweep also clicks each tab button; only the close button closes.
		let html = render_after_click(three_tabs);
		assert!(html.contains("closed:1"), "{html}");
	}

	#[test]
	fn closes_a_closable_tab_on_delete_and_backspace_and_ignores_them_elsewhere() {
		// Every tab hears each key; only the closable one closes, once per key.
		let html = render_after_keys(three_tabs, &[Key::Delete, Key::Backspace]);
		assert!(html.contains("closed:2"), "{html}");
	}

	#[test]
	fn other_keys_do_not_close() {
		for key in [Key::Enter, Key::Escape, Key::ArrowRight] {
			let html = render_after_keydown(three_tabs, key.clone());
			assert!(html.contains("closed:0"), "{key:?}: {html}");
		}
	}

	/// The focus target for Delete, which SSR cannot observe as DOM focus: the
	/// next tab, or the previous one when the closed tab is the last.
	#[test]
	fn hands_focus_to_the_next_tab_else_the_previous_on_close() {
		#[component]
		fn Probe() -> Element {
			let roving = use_context::<WorkspaceTabsCtx>().roving;
			let show = |key: &str| roving.neighbour(key).unwrap_or_else(|| "none".to_string());
			rsx! { p { "after-b:{show(\"b\")} after-c:{show(\"c\")}" } }
		}
		fn app() -> Element {
			rsx! {
				WorkspaceTabs { default_value: "a",
					WorkspaceTab { value: "a", "Торговля" }
					WorkspaceTab { value: "b", "Портфель" }
					WorkspaceTab { value: "c", "Отчёты" }
					Probe {}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("after-b:c after-c:b"), "{html}");
	}

	#[test]
	fn a_lone_tab_has_no_neighbour_to_take_focus() {
		#[component]
		fn Probe() -> Element {
			let roving = use_context::<WorkspaceTabsCtx>().roving;
			let next = roving.neighbour("a").unwrap_or_else(|| "none".to_string());
			rsx! { p { "after-a:{next}" } }
		}
		fn app() -> Element {
			rsx! {
				WorkspaceTabs { default_value: "a",
					WorkspaceTab { value: "a", "Торговля" }
					Probe {}
				}
			}
		}
		assert!(render(app).contains("after-a:none"));
	}

	#[test]
	fn marks_the_open_tab_and_fills_only_it() {
		let html = render(three_tabs);
		assert_eq!(html.matches("data-state=\"active\"").count(), 1, "{html}");
		assert_eq!(html.matches("hover:bg-secondary").count(), 1, "{html}");
		// A tab without a close button keeps the title's right inset.
		assert_eq!(html.matches("class=\"w-2\"").count(), 2, "{html}");
	}

	#[test]
	fn truncates_a_long_title_instead_of_widening_the_row() {
		fn app() -> Element {
			rsx! {
				WorkspaceTabs {
					WorkspaceTab { value: "a", "Сводный отчёт по доходности портфеля за квартал" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("<span class=\"truncate\">Сводный отчёт по доходности портфеля за квартал</span>"), "{html}");
		assert!(html.contains("min-w-0"), "{html}");
	}

	#[test]
	fn a_custom_icon_replaces_the_layers_glyph() {
		fn app() -> Element {
			rsx! {
				WorkspaceTabs {
					WorkspaceTab { value: "a", icon: rsx! { svg { "data-testid": "own" } }, "Торговля" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-testid=\"own\""), "{html}");
		assert_eq!(html.matches("<svg").count(), 1, "{html}");
	}

	#[test]
	fn class_lands_on_the_pill_and_wins() {
		fn app() -> Element {
			rsx! {
				WorkspaceTabs { class: "gap-2",
					WorkspaceTab { value: "a", class: "h-8", "Торговля" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("class=\"flex items-center gap-2\""), "{html}");
		let pill = &html[html.find("data-slot=\"workspace-tab\"").expect("pill")..];
		let pill = &pill[..pill.find('>').expect("tag end")];
		assert!(pill.contains("h-8") && !pill.contains("h-7"), "{pill}");
	}
}
