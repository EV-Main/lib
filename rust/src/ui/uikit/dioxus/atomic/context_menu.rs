use dioxus::prelude::*;
pub use ev_lib_classes::MenuItemVariant;
use ev_lib_classes::{CONTEXT_MENU_ITEM_INDICATOR, Icon, MENU_RADIO_DOT};

use crate::{
	cn,
	ui::uikit::dioxus::{
		CONTEXT_MENU_CHECK_ITEM, CONTEXT_MENU_CONTENT, CONTEXT_MENU_ITEM, CONTEXT_MENU_LABEL, CONTEXT_MENU_SEPARATOR, CONTEXT_MENU_SHORTCUT, CONTEXT_MENU_SUB_CONTENT,
		CONTEXT_MENU_SUB_TRIGGER, Slot,
		primitives::{Controllable, icon_shapes, use_controllable},
	},
};

#[component]
pub fn ContextMenu(open: Option<bool>, #[props(default)] default_open: bool, on_open_change: Option<EventHandler<bool>>, children: Element) -> Element {
	let state = use_controllable(open, default_open, on_open_change);
	let point = use_signal(|| (0, 0));
	use_context_provider(|| ContextMenuCtx { open: state, point });
	rsx! {
		div { "data-slot": Slot::ContextMenu.as_str(), {children} }
	}
}
#[component]
pub fn ContextMenuTrigger(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<ContextMenuCtx>();
	let mut point = ctx.point;
	rsx! {
		div {
			class,
			"data-slot": Slot::ContextMenuTrigger.as_str(),
			oncontextmenu: move |e| {
				e.prevent_default();
				let c = e.client_coordinates();
				point.set((c.x as i32, c.y as i32));
				ctx.open.set(true);
			},
			{children}
		}
	}
}
#[component]
pub fn ContextMenuContent(#[props(default)] class: String, children: Element) -> Element {
	// dep-light: fixed coords from the contextmenu event + backdrop; no portal/floating — see README Limitations
	let ctx = use_context::<ContextMenuCtx>();
	if !ctx.open.get() {
		return rsx! {};
	}
	let (x, y) = *ctx.point.read();
	let cls = cn!(CONTEXT_MENU_CONTENT, class);
	rsx! {
		div {
			class: "fixed inset-0 z-40",
			onclick: move |_| ctx.open.set(false),
		}
		div {
			class: cls,
			role: "menu",
			tabindex: "-1",
			style: "position: fixed; left: {x}px; top: {y}px;",
			"data-slot": Slot::ContextMenuContent.as_str(),
			"data-state": "open",
			"data-side": "bottom",
			onkeydown: move |e| {
				if e.key() == Key::Escape {
					ctx.open.set(false);
				}
			},
			{children}
		}
	}
}
#[component]
pub fn ContextMenuGroup(#[props(default)] class: String, children: Element) -> Element {
	rsx! {
		div { class, role: "group", "data-slot": Slot::ContextMenuGroup.as_str(), {children} }
	}
}
#[component]
pub fn ContextMenuItem(
	#[props(default)] variant: MenuItemVariant,
	#[props(default)] inset: bool,
	#[props(default)] disabled: bool,
	onclick: Option<EventHandler<MouseEvent>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let ctx = use_context::<ContextMenuCtx>();
	let cls = cn!(CONTEXT_MENU_ITEM, class);
	rsx! {
		div {
			class: cls,
			role: "menuitem",
			tabindex: "-1",
			"data-slot": Slot::ContextMenuItem.as_str(),
			"data-variant": variant.as_ref(),
			"data-inset": inset.then_some(""),
			"data-disabled": disabled.then_some(""),
			onclick: move |e| {
				if disabled {
					return;
				}
				if let Some(h) = onclick {
					h.call(e);
				}
				ctx.open.set(false);
			},
			{children}
		}
	}
}
#[component]
pub fn ContextMenuCheckboxItem(
	#[props(default)] checked: bool,
	#[props(default)] disabled: bool,
	onclick: Option<EventHandler<MouseEvent>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let ctx = use_context::<ContextMenuCtx>();
	let cls = cn!(CONTEXT_MENU_CHECK_ITEM, class);
	rsx! {
		div {
			class: cls,
			role: "menuitemcheckbox",
			tabindex: "-1",
			"data-slot": Slot::ContextMenuCheckboxItem.as_str(),
			"data-state": if checked { "checked" } else { "unchecked" },
			"data-disabled": disabled.then_some(""),
			"aria-checked": if checked { "true" } else { "false" },
			onclick: move |e| {
				if disabled {
					return;
				}
				if let Some(h) = onclick {
					h.call(e);
				}
				ctx.open.set(false);
			},
			span { class: CONTEXT_MENU_ITEM_INDICATOR,
				if checked {
					svg {
						class: "size-4",
						view_box: "0 0 24 24",
						fill: "none",
						stroke: "currentColor",
						stroke_width: "2",
						stroke_linecap: "round",
						stroke_linejoin: "round",
						"aria-hidden": "true",
						{icon_shapes(Icon::Check)}
					}
				}
			}
			{children}
		}
	}
}
#[component]
pub fn ContextMenuRadioGroup(
	value: Option<String>,
	#[props(default)] default_value: String,
	on_value_change: Option<EventHandler<String>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let state = use_controllable(value, default_value, on_value_change);
	use_context_provider(|| ContextMenuRadioCtx { value: state });
	rsx! {
		div { class, role: "group", "data-slot": Slot::ContextMenuRadioGroup.as_str(), {children} }
	}
}
#[component]
pub fn ContextMenuRadioItem(value: String, #[props(default)] disabled: bool, #[props(default)] class: String, children: Element) -> Element {
	let menu = use_context::<ContextMenuCtx>();
	let radio = use_context::<ContextMenuRadioCtx>();
	let checked = radio.value.get() == value;
	let cls = cn!(CONTEXT_MENU_CHECK_ITEM, class);
	rsx! {
		div {
			class: cls,
			role: "menuitemradio",
			tabindex: "-1",
			"data-slot": Slot::ContextMenuRadioItem.as_str(),
			"data-state": if checked { "checked" } else { "unchecked" },
			"data-disabled": disabled.then_some(""),
			"aria-checked": if checked { "true" } else { "false" },
			onclick: move |_| {
				if disabled {
					return;
				}
				radio.value.set(value.clone());
				menu.open.set(false);
			},
			span { class: CONTEXT_MENU_ITEM_INDICATOR,
				if checked {
					svg { class: MENU_RADIO_DOT, view_box: "0 0 24 24", "aria-hidden": "true",
						{icon_shapes(Icon::Dot)}
					}
				}
			}
			{children}
		}
	}
}
#[component]
pub fn ContextMenuLabel(#[props(default)] inset: bool, #[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(CONTEXT_MENU_LABEL, class);
	rsx! {
		div { class: cls, "data-slot": Slot::ContextMenuLabel.as_str(), "data-inset": inset.then_some(""), {children} }
	}
}
#[component]
pub fn ContextMenuSeparator(#[props(default)] class: String) -> Element {
	let cls = cn!(CONTEXT_MENU_SEPARATOR, class);
	rsx! {
		div { class: cls, role: "separator", "data-slot": Slot::ContextMenuSeparator.as_str() }
	}
}
#[component]
pub fn ContextMenuShortcut(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(CONTEXT_MENU_SHORTCUT, class);
	rsx! {
		span { class: cls, "data-slot": Slot::ContextMenuShortcut.as_str(), {children} }
	}
}
/// Sub-menus open inline (nested) rather than in a separate floating layer; the
/// sub-content is revealed in place when its trigger toggles. Keyboard roving is
/// simplified to the browser's native focus order.
#[component]
pub fn ContextMenuSub(open: Option<bool>, #[props(default)] default_open: bool, on_open_change: Option<EventHandler<bool>>, children: Element) -> Element {
	let state = use_controllable(open, default_open, on_open_change);
	use_context_provider(|| ContextMenuSubCtx { open: state });
	rsx! {
		div { "data-slot": Slot::ContextMenuSub.as_str(), {children} }
	}
}
#[component]
pub fn ContextMenuSubTrigger(#[props(default)] inset: bool, #[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<ContextMenuSubCtx>();
	let open = ctx.open.get();
	let cls = cn!(CONTEXT_MENU_SUB_TRIGGER, class);
	rsx! {
		div {
			class: cls,
			role: "menuitem",
			tabindex: "-1",
			"data-slot": Slot::ContextMenuSubTrigger.as_str(),
			"data-state": if open { "open" } else { "closed" },
			"data-inset": inset.then_some(""),
			"aria-haspopup": "menu",
			"aria-expanded": if open { "true" } else { "false" },
			onclick: move |_| ctx.open.set(!ctx.open.get()),
			{children}
			svg {
				class: "ml-auto",
				view_box: "0 0 24 24",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "2",
				stroke_linecap: "round",
				stroke_linejoin: "round",
				"aria-hidden": "true",
				{icon_shapes(Icon::ChevronRight)}
			}
		}
	}
}
#[component]
pub fn ContextMenuSubContent(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<ContextMenuSubCtx>();
	if !ctx.open.get() {
		return rsx! {};
	}
	let cls = cn!(CONTEXT_MENU_SUB_CONTENT, class);
	rsx! {
		div { class: cls, role: "menu", "data-slot": Slot::ContextMenuSubContent.as_str(), "data-state": "open", {children} }
	}
}
#[derive(Clone, Copy)]
struct ContextMenuCtx {
	open: Controllable<bool>,
	point: Signal<(i32, i32)>,
}
#[derive(Clone, Copy)]
struct ContextMenuRadioCtx {
	value: Controllable<String>,
}
#[derive(Clone, Copy)]
struct ContextMenuSubCtx {
	open: Controllable<bool>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

	#[test]
	fn closed_by_default_hides_content() {
		fn app() -> Element {
			rsx! {
				ContextMenu {
					ContextMenuTrigger { "area" }
					ContextMenuContent {
						ContextMenuItem { "Back" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"context-menu-trigger\""), "{html}");
		assert!(!html.contains("Back"), "content hidden while closed: {html}");
	}

	#[test]
	fn default_open_reveals_menu_item() {
		fn app() -> Element {
			rsx! {
				ContextMenu { default_open: true,
					ContextMenuTrigger { "area" }
					ContextMenuContent {
						ContextMenuItem { "Back" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"context-menu-content\""), "{html}");
		assert!(html.contains("role=\"menu\""), "{html}");
		assert!(html.contains("Back"), "{html}");
		assert!(html.contains("data-state=\"open\""), "{html}");
		assert!(!html.contains("data-disabled") && !html.contains("data-inset"), "the classes match on presence: {html}");
	}

	#[test]
	fn radio_item_marks_selected_value() {
		fn app() -> Element {
			rsx! {
				ContextMenu { default_open: true,
					ContextMenuContent {
						ContextMenuRadioGroup { default_value: "a".to_string(),
							ContextMenuRadioItem { value: "a", "Apple" }
							ContextMenuRadioItem { value: "b", "Banana" }
						}
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("role=\"menuitemradio\""), "{html}");
		assert!(html.contains("aria-checked=\"true\""), "{html}");
	}
}
