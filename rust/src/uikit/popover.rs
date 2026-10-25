use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		POPOVER_CONTENT,
		primitives::{Controllable, DismissableLayer, Side, use_controllable, use_dismissable_layer},
	},
};

#[component]
pub fn Popover(open: Option<bool>, #[props(default)] default_open: bool, on_open_change: Option<EventHandler<bool>>, children: Element) -> Element {
	let state = use_controllable(open, default_open, on_open_change);
	let layer = use_dismissable_layer(state.get(), move |_| state.set(false));
	use_context_provider({
		let layer = layer.clone();
		move || PopoverCtx { open: state, layer }
	});
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	rsx! {
		div {
			class: "relative inline-block",
			onkeydown: move |e| {
				layer.on_keydown(&e);
			},
			{children}
		}
	}
}

/// `aria_label` names a trigger that holds only a glyph: the kit has no
/// `asChild`, so an icon trigger cannot borrow the name of an `IconButton`.
#[component]
pub fn PopoverTrigger(#[props(default)] class: String, aria_label: Option<String>, children: Element) -> Element {
	let ctx = use_context::<PopoverCtx>();
	let open = ctx.open.get();
	rsx! {
		button {
			r#type: "button",
			class,
			"data-slot": "popover-trigger",
			"aria-label": aria_label,
			"aria-expanded": if open { "true" } else { "false" },
			onclick: move |_| ctx.open.set(!ctx.open.get()),
			{children}
		}
	}
}

/// Optional anchor for API parity with the TS `PopoverAnchor`. Inline placement
/// makes it a passive marker (the content positions against the wrapper).
#[component]
pub fn PopoverAnchor(#[props(default)] class: String, children: Element) -> Element {
	rsx! {
		div { class, "data-slot": "popover-anchor", {children} }
	}
}

/// `align` pins the panel's edge to the trigger's: `start` is the left edge,
/// `end` the right, anything else centres it. Without a measuring layer this is
/// all the placement there is, and a wide panel under a trigger at the edge of
/// its container needs `start` or `end` to stay in view.
///
/// `role` and `aria_labelledby` are for content that is a dialog of its own,
/// such as a form: the TS port forwards them as plain props.
#[component]
pub fn PopoverContent(
	#[props(default)] side: Side,
	#[props(default = String::from("center"))] align: String,
	role: Option<String>,
	aria_labelledby: Option<String>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let ctx = use_context::<PopoverCtx>();
	if !ctx.open.get() {
		return rsx! {};
	}
	let placement = match align.as_str() {
		"start" => "absolute top-full left-0 mt-1",
		"end" => "absolute top-full right-0 mt-1",
		_ => "absolute top-full left-1/2 -translate-x-1/2 mt-1",
	};
	let cls = cn!(POPOVER_CONTENT, placement, class);
	let layer = ctx.layer.clone();
	rsx! {
		div {
			class: "fixed inset-0 z-40",
			onclick: move |_| layer.on_outside(),
		}
		div {
			role,
			"aria-labelledby": aria_labelledby,
			class: cls,
			"data-slot": "popover-content",
			"data-state": "open",
			"data-side": side.as_ref(),
			"data-align": align,
			{children}
		}
	}
}

#[derive(Clone)]
struct PopoverCtx {
	open: Controllable<bool>,
	layer: DismissableLayer,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	#[test]
	fn align_pins_the_panel_to_a_trigger_edge() {
		fn start() -> Element {
			rsx! { Popover { default_open: true, PopoverTrigger { "open" } PopoverContent { align: "start", "panel" } } }
		}
		fn end() -> Element {
			rsx! { Popover { default_open: true, PopoverTrigger { "open" } PopoverContent { align: "end", "panel" } } }
		}
		fn centred() -> Element {
			rsx! { Popover { default_open: true, PopoverTrigger { "open" } PopoverContent { "panel" } } }
		}
		let (start, end, centred) = (render(start), render(end), render(centred));
		assert!(start.contains("left-0") && !start.contains("-translate-x-1/2"), "{start}");
		assert!(end.contains("right-0") && !end.contains("-translate-x-1/2"), "{end}");
		assert!(centred.contains("left-1/2") && centred.contains("-translate-x-1/2"), "{centred}");
	}

	#[test]
	fn closed_by_default_hides_content() {
		fn app() -> Element {
			rsx! {
				Popover {
					PopoverTrigger { "open" }
					PopoverContent { "panel" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"popover-trigger\""), "{html}");
		assert!(!html.contains("panel"), "content hidden while closed: {html}");
		assert!(html.contains("aria-expanded=\"false\""), "{html}");
	}

	#[test]
	fn default_open_reveals_content() {
		fn app() -> Element {
			rsx! {
				Popover { default_open: true,
					PopoverTrigger { "open" }
					PopoverContent { "panel" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"popover-content\""), "{html}");
		assert!(html.contains("panel"), "{html}");
		assert!(html.contains("data-state=\"open\""), "{html}");
		assert!(html.contains("data-side=\"bottom\""), "{html}");
		assert!(html.contains("aria-expanded=\"true\""), "{html}");
	}
}
