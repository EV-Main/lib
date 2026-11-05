use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{ACCORDION_CHEVRON, toggled},
	uikit_dioxus::{
		ACCORDION_CONTENT, ACCORDION_CONTENT_INNER, ACCORDION_HEADER, ACCORDION_ITEM, ACCORDION_TRIGGER, AccordionType,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn Accordion(
	#[props(default)] r#type: AccordionType,
	#[props(default)] collapsible: bool,
	value: Option<Vec<String>>,
	#[props(default)] default_value: Vec<String>,
	on_value_change: Option<EventHandler<Vec<String>>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let open = use_controllable(value, default_value, on_value_change);
	use_context_provider(|| AccordionCtx { open, kind: r#type, collapsible });
	rsx! {
		div { class, "data-slot": "accordion", {children} }
	}
}
#[component]
pub fn AccordionItem(value: String, #[props(default)] class: String, children: Element) -> Element {
	use_context_provider(|| ItemCtx { value: value.clone() });
	let cls = cn!(ACCORDION_ITEM, class);
	rsx! {
		div { class: cls, "data-slot": "accordion-item", {children} }
	}
}
#[component]
pub fn AccordionTrigger(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<AccordionCtx>();
	let item = use_context::<ItemCtx>();
	let value = item.value.clone();
	let open = ctx.is_open(&value);
	let data_state = if open { "open" } else { "closed" };
	let aria_expanded = if open { "true" } else { "false" };
	let cls = cn!(ACCORDION_TRIGGER, class);
	rsx! {
		h3 { class: ACCORDION_HEADER, "data-slot": "accordion-header",
			button {
				r#type: "button",
				class: cls,
				"data-slot": "accordion-trigger",
				"data-state": data_state,
				"aria-expanded": aria_expanded,
				onclick: move |_| ctx.toggle(&value),
				{children}
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
					class: ACCORDION_CHEVRON,
					path { d: "m6 9 6 6 6-6" }
				}
			}
		}
	}
}
#[component]
pub fn AccordionContent(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<AccordionCtx>();
	let item = use_context::<ItemCtx>();
	if !ctx.is_open(&item.value) {
		return rsx! {};
	}
	let inner = cn!(ACCORDION_CONTENT_INNER, class);
	rsx! {
		div { class: ACCORDION_CONTENT, "data-slot": "accordion-content", "data-state": "open",
			div { class: inner, {children} }
		}
	}
}
#[derive(Clone, Copy)]
struct AccordionCtx {
	open: Controllable<Vec<String>>,
	kind: AccordionType,
	collapsible: bool,
}

impl AccordionCtx {
	fn is_open(&self, value: &str) -> bool {
		self.open.get().iter().any(|v| v == value)
	}

	fn toggle(&self, value: &str) {
		self.open.set(toggled(&self.open.get(), value, self.kind == AccordionType::Multiple, self.collapsible));
	}
}

#[derive(Clone)]
struct ItemCtx {
	value: String,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::test_util::render;

	#[test]
	fn closed_item_hides_content() {
		fn app() -> Element {
			rsx! {
				Accordion {
					AccordionItem { value: "a",
						AccordionTrigger { "A" }
						AccordionContent { "body-a" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"accordion-trigger\""), "{html}");
		assert!(!html.contains("body-a"), "content hidden while closed: {html}");
	}

	#[test]
	fn open_item_via_default_value_shows_content() {
		fn app() -> Element {
			rsx! {
				Accordion { default_value: vec!["a".to_string()],
					AccordionItem { value: "a",
						AccordionTrigger { "A" }
						AccordionContent { "body-a" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"accordion-content\""), "{html}");
		assert!(html.contains("body-a"), "{html}");
		assert!(html.contains("data-state=\"open\""), "{html}");
	}

	#[test]
	fn trigger_renders_chevron_path() {
		fn app() -> Element {
			rsx! {
				Accordion {
					AccordionItem { value: "a",
						AccordionTrigger { "A" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("m6 9 6 6 6-6"), "{html}");
	}
}
