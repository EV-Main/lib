use dioxus::prelude::*;
use ev_lib_classes::{Icon, copy};

use crate::{
	cn,
	ui::uikit::{
		core::sheet_side_class,
		dioxus::{
			SHEET_CLOSE, SHEET_CONTENT, SHEET_DESCRIPTION, SHEET_FOOTER, SHEET_HEADER, SHEET_OVERLAY, SHEET_TITLE, Slot,
			primitives::{Controllable, Side, icon_shapes, use_controllable},
		},
	},
};

#[component]
pub fn Sheet(open: Option<bool>, #[props(default)] default_open: bool, on_open_change: Option<EventHandler<bool>>, children: Element) -> Element {
	let state = use_controllable(open, default_open, on_open_change);
	use_context_provider(|| SheetCtx { open: state });
	rsx! {
		div { "data-slot": Slot::Sheet.as_str(), {children} }
	}
}
#[component]
pub fn SheetTrigger(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<SheetCtx>();
	rsx! {
		button {
			r#type: "button",
			class,
			"data-slot": Slot::SheetTrigger.as_str(),
			"aria-expanded": if ctx.open.get() { "true" } else { "false" },
			onclick: move |_| ctx.open.set(true),
			{children}
		}
	}
}
/// `aria_label` names a sheet that has no `SheetTitle`, as the TS one takes
/// `aria-label` through its props.
#[component]
pub fn SheetContent(#[props(default = Side::Right)] side: Side, #[props(default)] class: String, aria_label: Option<String>, children: Element) -> Element {
	let ctx = use_context::<SheetCtx>();
	if !ctx.open.get() {
		return rsx! {};
	}
	let cls = cn!(SHEET_CONTENT, sheet_side_class(side), class);
	rsx! {
		// dep-light: native focus order, no trap/portal — see README Limitations
		div {
			class: SHEET_OVERLAY,
			"data-slot": Slot::SheetOverlay.as_str(),
			"data-state": "open",
			onclick: move |_| ctx.open.set(false),
		}
		div {
			role: "dialog",
			"aria-modal": "true",
			"aria-label": aria_label,
			class: cls,
			"data-slot": Slot::SheetContent.as_str(),
			"data-state": "open",
			tabindex: "-1",
			onclick: move |e| e.stop_propagation(),
			onkeydown: move |e| if e.key() == Key::Escape { ctx.open.set(false) },
			{children}
			button {
				r#type: "button",
				class: SHEET_CLOSE,
				"data-slot": Slot::SheetClose.as_str(),
				onclick: move |_| ctx.open.set(false),
				svg {
					xmlns: "http://www.w3.org/2000/svg",
					width: "24",
					height: "24",
					view_box: "0 0 24 24",
					fill: "none",
					stroke: "currentColor",
					stroke_width: "2",
					stroke_linecap: "round",
					stroke_linejoin: "round",
					class: "size-4",
					{icon_shapes(Icon::X)}
				}
				span { class: "sr-only", {copy::CLOSE} }
			}
		}
	}
}
#[component]
pub fn SheetClose(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<SheetCtx>();
	rsx! {
		button {
			r#type: "button",
			class,
			"data-slot": Slot::SheetClose.as_str(),
			onclick: move |_| ctx.open.set(false),
			{children}
		}
	}
}
#[component]
pub fn SheetHeader(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(SHEET_HEADER, class);
	rsx! {
		div { class: cls, "data-slot": Slot::SheetHeader.as_str(), {children} }
	}
}
#[component]
pub fn SheetFooter(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(SHEET_FOOTER, class);
	rsx! {
		div { class: cls, "data-slot": Slot::SheetFooter.as_str(), {children} }
	}
}
#[component]
pub fn SheetTitle(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(SHEET_TITLE, class);
	rsx! {
		h2 { class: cls, "data-slot": Slot::SheetTitle.as_str(), {children} }
	}
}
#[component]
pub fn SheetDescription(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(SHEET_DESCRIPTION, class);
	rsx! {
		p { class: cls, "data-slot": Slot::SheetDescription.as_str(), {children} }
	}
}
#[derive(Clone, Copy)]
struct SheetCtx {
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
				Sheet {
					SheetTrigger { "open" }
					SheetContent {
						SheetTitle { "Title" }
						SheetDescription { "Body" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"sheet-trigger\""), "{html}");
		assert!(!html.contains("Body"), "content hidden while closed: {html}");
	}

	#[test]
	fn default_open_reveals_dialog_with_role() {
		fn app() -> Element {
			rsx! {
				Sheet { default_open: true,
					SheetContent {
						SheetTitle { "Title" }
						SheetDescription { "Body" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("role=\"dialog\""), "{html}");
		assert!(html.contains("aria-modal=\"true\""), "{html}");
		assert!(html.contains("data-state=\"open\""), "{html}");
		assert!(html.contains("Title"), "{html}");
		assert!(html.contains("Body"), "{html}");
	}

	#[test]
	fn side_drives_slide_in_classes() {
		fn app() -> Element {
			rsx! {
				Sheet { default_open: true,
					SheetContent { side: Side::Left, SheetTitle { "Left" } }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("slide-in-from-left"), "{html}");
	}

	#[test]
	fn names_itself_only_when_asked() {
		fn named() -> Element {
			rsx! {
				Sheet { default_open: true,
					SheetContent { aria_label: "Фильтры", "body" }
				}
			}
		}
		fn plain() -> Element {
			rsx! {
				Sheet { default_open: true,
					SheetContent { "body" }
				}
			}
		}
		assert!(render(named).contains("aria-label=\"Фильтры\""));
		assert!(!render(plain).contains("aria-label"));
	}
}
