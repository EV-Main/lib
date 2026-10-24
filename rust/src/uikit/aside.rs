use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		ASIDE_BODY, ASIDE_FOOTER, ASIDE_HEADER_BACK_ICON, ASIDE_HEADER_BASE, ASIDE_HEADER_CLOSE_ICON, ASIDE_HEADER_TITLE, ASIDE_PANEL_DOCKED, ASIDE_PANEL_DRAWER, ASIDE_PANEL_NARROW_ONLY,
		ASIDE_PANEL_SHEET, ASIDE_PANEL_WIDE_ONLY, ASIDE_PANEL_WIDE_ONLY_WRAPPER, ASIDE_RAIL_BASE, ASIDE_RAIL_SPACER, AsidePanelSide, Drawer, DrawerContent, DrawerDirection, IconButton,
		Sheet, SheetContent, Size, primitives::Side,
	},
};

/// The header's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AsideHeaderLabels {
	/// `aria-label` of the back button; "Back".
	pub back: Option<String>,
	/// `aria-label` of the close button; "Close".
	pub close: Option<String>,
}

/// The 44 px title row of an aside. Mirrors the TS `AsideHeader`.
///
/// The back and close buttons render only when their handler is set, so a
/// button that does nothing is never shown.
#[component]
pub fn AsideHeader(
	title: Element,
	/// Shows the back button first.
	on_back: Option<EventHandler<()>>,
	/// Shows the close button last.
	on_close: Option<EventHandler<()>>,
	/// `IconButton`s between the title and the close button: pin, more.
	actions: Option<Element>,
	#[props(default)] labels: AsideHeaderLabels,
	#[props(default)] class: String,
) -> Element {
	let back_label = labels.back.unwrap_or_else(|| String::from("Back"));
	let close_label = labels.close.unwrap_or_else(|| String::from("Close"));
	let cls = cn!(ASIDE_HEADER_BASE, class);
	rsx! {
		div { "data-slot": "aside-header", class: cls,
			if let Some(back) = on_back {
				IconButton { aria_label: back_label, size: Size::Sm, onclick: move |_| back.call(()),
					Glyph { class: ASIDE_HEADER_BACK_ICON,
						path { d: "m9 18 6-6-6-6" }
					}
				}
			}
			h2 { class: ASIDE_HEADER_TITLE, {title} }
			{actions}
			if let Some(close) = on_close {
				IconButton { aria_label: close_label, size: Size::Sm, onclick: move |_| close.call(()),
					Glyph { class: ASIDE_HEADER_CLOSE_ICON,
						path { d: "M18 6 6 18M6 6l12 12" }
					}
				}
			}
		}
	}
}

/// One button of an [`AsideRail`]. Mirrors the TS `AsideRailItem`.
#[derive(Clone, PartialEq)]
pub struct AsideRailItem {
	pub id: String,
	/// Names the button and fills its tooltip.
	pub label: String,
	pub icon: Element,
}

/// The 44 px strip of icon buttons that opens and closes the asides. Mirrors
/// the TS `AsideRail`.
///
/// The button of the open aside is pressed. Pressing it again asks for `None`,
/// so the rail closes what it opened.
#[component]
pub fn AsideRail(
	items: Vec<AsideRailItem>,
	/// Pinned to the bottom, e.g. settings.
	#[props(default)]
	end_items: Vec<AsideRailItem>,
	/// The open aside's id; `None` when all are closed.
	value: Option<String>,
	on_value_change: EventHandler<Option<String>>,
	#[props(default = String::from("Panels"))] aria_label: String,
	#[props(default)] class: String,
) -> Element {
	let cls = cn!(ASIDE_RAIL_BASE, class);
	rsx! {
		div { role: "group", "aria-label": aria_label, "data-slot": "aside-rail", class: cls,
			for item in items {
				AsideRailButton { key: "{item.id}", item, value: value.clone(), on_value_change }
			}
			span { class: ASIDE_RAIL_SPACER }
			for item in end_items {
				AsideRailButton { key: "{item.id}", item, value: value.clone(), on_value_change }
			}
		}
	}
}

#[component]
fn AsideRailButton(item: AsideRailItem, value: Option<String>, on_value_change: EventHandler<Option<String>>) -> Element {
	let open = value.as_deref() == Some(item.id.as_str());
	let id = item.id;
	rsx! {
		IconButton {
			aria_label: item.label.clone(),
			tooltip: rsx! { {item.label} },
			pressed: open,
			onclick: move |_| on_value_change.call(if open { None } else { Some(id.clone()) }),
			{item.icon}
		}
	}
}

/// How an [`AsidePanel`] takes its place. Mirrors the TS `AsidePanelMode`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AsidePanelMode {
	/// Sits in the layout and takes room from the content.
	#[default]
	Docked,
	/// Floats over a scrim on a [`Sheet`].
	Overlay,
}

/// Where an [`AsidePanel`] gives up its wide form for a bottom [`Drawer`].
/// The TS `collapseBelow` takes `"md" | false`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AsideCollapse {
	/// Below the `md` breakpoint (768 px).
	#[default]
	Md,
	/// At no width: for a framed preview or an embedded layout, where the
	/// viewport says nothing about the room.
	Never,
}

/// A side panel. Mirrors the TS `AsidePanel`.
///
/// `Docked` sits in the layout and takes room from the content; `Overlay`
/// floats over a scrim on a [`Sheet`]. Below `md` both use a bottom
/// [`Drawer`]. The edge that faces the content is the only border. A closed
/// panel renders nothing.
///
/// The TS port picks the form with `matchMedia`. This port has no
/// `matchMedia`: it renders the wide form and the drawer together, and the
/// `md` breakpoint classes show one of them. So with [`AsideCollapse::Md`]
/// an open panel mounts `children` twice, and a child with state keeps it
/// only for the form it is in. [`AsideCollapse::Never`] mounts them once.
///
/// The overlays keep the kit's limits: rendered inline with `position: fixed`,
/// no portal and no focus trap. A docked panel is dismissable by Escape and
/// the scrim only on its drawer, below `md`.
#[component]
pub fn AsidePanel(
	/// Names the panel.
	aria_label: String,
	#[props(default)] mode: AsidePanelMode,
	/// Which edge of the screen it hugs.
	#[props(default)]
	side: AsidePanelSide,
	#[props(default = true)] open: bool,
	/// Escape and the scrim call it with `false`.
	on_open_change: Option<EventHandler<bool>>,
	/// An [`AsideHeader`].
	header: Option<Element>,
	/// Sticky under the body: actions.
	footer: Option<Element>,
	#[props(default)] collapse_below: AsideCollapse,
	/// Docked 380 (260 for a left navigation), overlay 420: a width class
	/// overrides it.
	#[props(default)]
	class: String,
	children: Element,
) -> Element {
	let body = || {
		rsx! {
			{header.clone()}
			div { "data-slot": "aside-body", class: ASIDE_BODY, {children.clone()} }
			if let Some(footer) = footer.clone() {
				div { "data-slot": "aside-footer", class: ASIDE_FOOTER, {footer} }
			}
		}
	};
	let docked = |visibility: &str| {
		rsx! {
			aside {
				"aria-label": aria_label.clone(),
				"data-slot": "aside-panel",
				"data-mode": "docked",
				"data-side": side.as_ref(),
				class: cn!(ASIDE_PANEL_DOCKED, side.as_class(), &class, visibility),
				{body()}
			}
		}
	};
	let sheet = || {
		let sheet_side = match side {
			AsidePanelSide::Left => Side::Left,
			AsidePanelSide::Right => Side::Right,
		};
		rsx! {
			Sheet { open, on_open_change,
				SheetContent { side: sheet_side, aria_label: aria_label.clone(), class: cn!(ASIDE_PANEL_SHEET, &class), {body()} }
			}
		}
	};
	// Mounted even while closed, so a drawer that closes plays its exit.
	let drawer = || {
		rsx! {
			div { "data-slot": "aside-panel-narrow", class: ASIDE_PANEL_NARROW_ONLY,
				Drawer { open, on_open_change, direction: DrawerDirection::Bottom,
					DrawerContent { aria_label: aria_label.clone(), class: ASIDE_PANEL_DRAWER, {body()} }
				}
			}
		}
	};
	match (mode, collapse_below) {
		(AsidePanelMode::Docked, AsideCollapse::Never) => {
			if !open {
				return rsx! {};
			}
			docked("")
		}
		(AsidePanelMode::Docked, AsideCollapse::Md) => rsx! {
			if open {
				{docked(ASIDE_PANEL_WIDE_ONLY)}
			}
			{drawer()}
		},
		(AsidePanelMode::Overlay, AsideCollapse::Never) => sheet(),
		(AsidePanelMode::Overlay, AsideCollapse::Md) => rsx! {
			div { "data-slot": "aside-panel-wide", class: ASIDE_PANEL_WIDE_ONLY_WRAPPER, {sheet()} }
			{drawer()}
		},
	}
}

/// The TS `UiIcon` frame: a 24-unit lucide glyph drawn in the current ink.
#[component]
fn Glyph(class: &'static str, children: Element) -> Element {
	rsx! {
		svg {
			class,
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.75",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			"aria-hidden": "true",
			{children}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::{
		cell::RefCell,
		sync::atomic::{AtomicUsize, Ordering},
	};

	use super::*;
	use crate::uikit::test_util::{Harness, render, render_focused};

	/// The whole opening tag that holds `marker`, whatever the attribute order.
	fn opening_tag<'a>(html: &'a str, marker: &str) -> &'a str {
		let at = html.find(marker).unwrap_or_else(|| panic!("{marker} rendered: {html}"));
		let start = html[..at].rfind('<').expect("tag start");
		let end = at + html[at..].find('>').expect("tag end");
		&html[start..end]
	}

	#[test]
	fn the_header_shows_the_title_and_only_the_buttons_it_has_handlers_for() {
		static CLOSES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! { AsideHeader { title: rsx! { "Список наблюдения" }, on_close: |_| { CLOSES.fetch_add(1, Ordering::SeqCst); } } }
		}
		let mut h = Harness::new(app);
		let html = h.html();
		assert!(html.contains("Список наблюдения</h2>"), "{html}");
		assert!(opening_tag(&html, "Список наблюдения").contains(ASIDE_HEADER_TITLE), "{html}");
		assert!(html.contains("aria-label=\"Close\""), "{html}");
		assert!(!html.contains("aria-label=\"Back\""), "{html}");
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 1, "only the close button: {html}");
		h.click(clicks[0]);
		assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn the_header_puts_the_back_button_first() {
		static BACKS: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				AsideHeader {
					title: rsx! { "Изменить оповещение" },
					on_back: |_| { BACKS.fetch_add(1, Ordering::SeqCst); },
					on_close: |_| {},
					labels: AsideHeaderLabels { back: Some("Назад".into()), ..Default::default() },
				}
			}
		}
		let mut h = Harness::new(app);
		let html = h.html();
		let back = html.find("aria-label=\"Назад\"").expect("back button");
		let title = html.find("Изменить оповещение").expect("title");
		let close = html.find("aria-label=\"Close\"").expect("close button");
		assert!(back < title && title < close, "{html}");
		// Creation order is not DOM order, so the ids cannot say which button
		// is which; the back-only header pins the wiring instead.
		fn back_only() -> Element {
			rsx! { AsideHeader { title: rsx! { "Изменить оповещение" }, on_back: |_| { BACKS.fetch_add(1, Ordering::SeqCst); } } }
		}
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 2, "{html}");
		let mut only = Harness::new(back_only);
		let back = only.listening("click");
		assert_eq!(back.len(), 1, "{}", only.html());
		assert!(!only.html().contains("aria-label=\"Close\""), "{}", only.html());
		only.click(back[0]);
		assert_eq!(BACKS.load(Ordering::SeqCst), 1);
		for id in clicks {
			h.click(id);
		}
		assert_eq!(BACKS.load(Ordering::SeqCst), 2, "one of the two buttons goes back");
	}

	fn rail_items() -> Vec<AsideRailItem> {
		vec![
			AsideRailItem {
				id: "list".into(),
				label: "Список наблюдения".into(),
				icon: rsx! { svg {} },
			},
			AsideRailItem {
				id: "chat".into(),
				label: "Ассистент".into(),
				icon: rsx! { svg {} },
			},
		]
	}

	#[test]
	fn the_rail_marks_the_open_asides_button_as_pressed() {
		fn app() -> Element {
			rsx! { AsideRail { items: rail_items(), value: "chat".to_string(), on_value_change: |_| {} } }
		}
		let html = render(app);
		let group = opening_tag(&html, "data-slot=\"aside-rail\"");
		assert!(group.contains("role=\"group\"") && group.contains("aria-label=\"Panels\""), "{group}");
		assert!(opening_tag(&html, "aria-label=\"Ассистент\"").contains("aria-pressed=true"), "{html}");
		assert!(opening_tag(&html, "aria-label=\"Список наблюдения\"").contains("aria-pressed=false"), "{html}");
	}

	#[test]
	fn the_rail_opens_another_aside_and_closes_the_open_one_on_a_second_press() {
		thread_local! {
			static ASKED: RefCell<Vec<Option<String>>> = const { RefCell::new(Vec::new()) };
		}
		fn app() -> Element {
			rsx! { AsideRail { items: rail_items(), value: "chat".to_string(), on_value_change: |v| ASKED.with(|a| a.borrow_mut().push(v)) } }
		}
		let mut h = Harness::new(app);
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 2, "{}", h.html());
		h.click(clicks[0]);
		h.click(clicks[1]);
		ASKED.with(|a| assert_eq!(*a.borrow(), vec![Some("list".to_string()), None]));
	}

	#[test]
	fn the_rail_names_the_buttons_by_tooltip_and_keeps_end_items_after_the_spacer() {
		fn app() -> Element {
			rsx! {
				AsideRail {
					items: rail_items(),
					end_items: vec![AsideRailItem { id: "set".into(), label: "Настройки".into(), icon: rsx! { svg {} } }],
					value: None,
					on_value_change: |_| {},
				}
			}
		}
		let html = render(app);
		let at = |s: &str| html.find(s).unwrap_or_else(|| panic!("{s}: {html}"));
		let spacer = at(&format!("class=\"{ASIDE_RAIL_SPACER}\""));
		assert!(at("aria-label=\"Список наблюдения\"") < at("aria-label=\"Ассистент\""), "{html}");
		assert!(at("aria-label=\"Ассистент\"") < spacer && spacer < at("aria-label=\"Настройки\""), "{html}");
		assert!(!opening_tag(&html, "aria-label=\"Настройки\"").contains("aria-pressed=true"), "{html}");
		let html = render_focused(app);
		assert!(html.contains("role=\"tooltip\""), "{html}");
		assert!(html.contains(">Настройки<"), "{html}");
	}

	#[test]
	fn docked_sits_in_the_layout_with_the_border_on_the_inner_edge() {
		fn app() -> Element {
			rsx! {
				AsidePanel {
					aria_label: "Список наблюдения",
					header: rsx! { AsideHeader { title: rsx! { "Список наблюдения" } } },
					footer: rsx! { button { r#type: "button", "Сохранить" } },
					p { "Тело" }
				}
			}
		}
		let html = render(app);
		let panel = opening_tag(&html, "data-slot=\"aside-panel\"");
		assert!(panel.starts_with("<aside"), "{panel}");
		assert!(panel.contains("aria-label=\"Список наблюдения\""), "{panel}");
		assert!(panel.contains("data-mode=\"docked\"") && panel.contains("data-side=\"right\""), "{panel}");
		assert!(panel.contains("border-l") && !panel.contains("border-r"), "{panel}");
		let aside = &html[html.find("<aside").expect("aside")..html.find("</aside>").expect("aside end")];
		assert!(aside.contains("Тело") && aside.contains("Сохранить"), "{aside}");
		assert!(aside.contains("data-slot=\"aside-footer\""), "{aside}");
	}

	/// The port of the TS "below md falls back to a drawer": the breakpoint
	/// classes, not `matchMedia`, hide the docked form below `md` and the
	/// drawer above it.
	#[test]
	fn docked_below_md_falls_back_to_a_drawer() {
		fn app() -> Element {
			rsx! { AsidePanel { aria_label: "Список наблюдения", p { "Тело" } } }
		}
		let html = render(app);
		let panel = opening_tag(&html, "data-slot=\"aside-panel\"");
		assert!(panel.contains("hidden md:flex"), "{panel}");
		let narrow = html.find("data-slot=\"aside-panel-narrow\"").expect("narrow form");
		assert!(opening_tag(&html, "data-slot=\"aside-panel-narrow\"").contains("md:hidden"), "{html}");
		let drawer = opening_tag(&html, "data-slot=\"drawer-content\"");
		assert!(html.find("data-slot=\"drawer-content\"").expect("drawer") > narrow, "{html}");
		assert!(drawer.contains("aria-label=\"Список наблюдения\""), "{drawer}");
		assert!(drawer.contains("max-h-[85dvh]"), "{drawer}");
	}

	#[test]
	fn collapse_never_keeps_a_docked_panel_docked_at_every_width() {
		fn app() -> Element {
			rsx! { AsidePanel { aria_label: "Список наблюдения", collapse_below: AsideCollapse::Never, p { "Тело" } } }
		}
		let html = render(app);
		let panel = opening_tag(&html, "data-slot=\"aside-panel\"");
		assert!(!panel.contains("hidden"), "{panel}");
		assert!(!html.contains("drawer-content"), "{html}");
		assert_eq!(html.matches("Тело").count(), 1, "{html}");
	}

	#[test]
	fn the_drawer_path_reports_escape_and_a_scrim_press_as_a_close() {
		static CLOSES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				AsidePanel {
					aria_label: "Список наблюдения",
					on_open_change: |open: bool| { if !open { CLOSES.fetch_add(1, Ordering::SeqCst); } },
					p { "Тело" }
				}
			}
		}
		let mut h = Harness::new(app);
		let keys = h.listening("keydown");
		// Only the drawer: wide, a docked panel is not dismissable.
		assert_eq!(keys.len(), 1, "{}", h.html());
		h.key(keys[0], Key::Escape);
		assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 1, "the scrim: {}", h.html());
		h.click(clicks[0]);
		assert_eq!(CLOSES.load(Ordering::SeqCst), 2);
	}

	#[test]
	fn docked_on_the_left_borders_the_right_edge() {
		fn app() -> Element {
			rsx! { AsidePanel { aria_label: "Навигация", side: AsidePanelSide::Left, "x" } }
		}
		let html = render(app);
		let panel = opening_tag(&html, "data-slot=\"aside-panel\"");
		assert!(panel.contains("border-r") && !panel.contains("border-l"), "{panel}");
		assert!(panel.contains("data-side=\"left\""), "{panel}");
	}

	#[test]
	fn a_closed_docked_panel_renders_nothing() {
		fn app() -> Element {
			rsx! { AsidePanel { aria_label: "Навигация", side: AsidePanelSide::Left, open: false, "Тело" } }
		}
		fn never() -> Element {
			rsx! { AsidePanel { aria_label: "Навигация", open: false, collapse_below: AsideCollapse::Never, "Тело" } }
		}
		for html in [render(app), render(never)] {
			assert!(!html.contains("<aside"), "{html}");
			assert!(!html.contains("Тело"), "{html}");
		}
	}

	#[test]
	fn overlay_on_a_wide_screen_is_a_modal_sheet_that_reports_dismissal() {
		static CLOSES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				AsidePanel {
					aria_label: "Удалить оповещение",
					mode: AsidePanelMode::Overlay,
					collapse_below: AsideCollapse::Never,
					on_open_change: |open: bool| { if !open { CLOSES.fetch_add(1, Ordering::SeqCst); } },
					header: rsx! { AsideHeader { title: rsx! { "Удалить оповещение" } } },
					p { "Точно?" }
				}
			}
		}
		let mut h = Harness::new(app);
		let html = h.html();
		let dialog = opening_tag(&html, "data-slot=\"sheet-content\"");
		assert!(dialog.contains("role=\"dialog\"") && dialog.contains("aria-modal=\"true\""), "{dialog}");
		assert!(dialog.contains("aria-label=\"Удалить оповещение\""), "{dialog}");
		assert!(dialog.contains("sm:max-w-105"), "{dialog}");
		assert!(html.contains("Точно?"), "{html}");
		assert!(!html.contains("drawer-content"), "{html}");
		let keys = h.listening("keydown");
		assert_eq!(keys.len(), 1, "{html}");
		h.key(keys[0], Key::Escape);
		assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
	}

	/// The port of the TS "overlay below md uses a drawer": the sheet shows
	/// from `md` up and the drawer below it.
	#[test]
	fn overlay_below_md_uses_a_drawer() {
		fn app() -> Element {
			rsx! { AsidePanel { aria_label: "Удалить оповещение", mode: AsidePanelMode::Overlay, p { "Точно?" } } }
		}
		let html = render(app);
		let wide = html.find("data-slot=\"aside-panel-wide\"").expect("wide form");
		let narrow = html.find("data-slot=\"aside-panel-narrow\"").expect("narrow form");
		assert!(opening_tag(&html, "data-slot=\"aside-panel-wide\"").contains("hidden md:contents"), "{html}");
		assert!(opening_tag(&html, "data-slot=\"aside-panel-narrow\"").contains("contents md:hidden"), "{html}");
		let sheet = html.find("data-slot=\"sheet-content\"").expect("sheet");
		let drawer = html.find("data-slot=\"drawer-content\"").expect("drawer");
		assert!(wide < sheet && sheet < narrow && narrow < drawer, "{html}");
	}

	#[test]
	fn a_closed_overlay_renders_nothing() {
		fn app() -> Element {
			rsx! { AsidePanel { aria_label: "Удалить оповещение", mode: AsidePanelMode::Overlay, open: false, p { "Точно?" } } }
		}
		let html = render(app);
		assert!(!html.contains("Точно?"), "{html}");
	}
}
