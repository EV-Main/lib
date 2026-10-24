use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{ICON_BUTTON_BASE, ICON_BUTTON_PRESSED, IconButtonVariant, Size, Tooltip, TooltipContent, button_classes, tooltip::TooltipCtx},
};

/// A square button holding one glyph. Mirrors the TS `IconButton`.
///
/// `aria_label` is required because a glyph alone names nothing. `pressed`
/// makes it a toggle: left unset, `aria-pressed` stays off, so a plain action
/// is not announced as a toggle. With a `tooltip` the same `<button>` anchors
/// the tooltip, the way the TS one sits under `TooltipTrigger asChild`.
///
/// `size` takes the shared [`Size`] scale; the TS type narrows it to
/// `sm | md | touch`, and the kit adds no per-component size enums.
#[component]
pub fn IconButton(
	aria_label: String,
	#[props(default)] variant: IconButtonVariant,
	#[props(default)] size: Size,
	pressed: Option<bool>,
	tooltip: Option<Element>,
	#[props(default)] class: String,
	#[props(default)] disabled: bool,
	/// `button` by default, unlike [`Button`](super::Button): a toolbar glyph
	/// inside a form must not submit it.
	#[props(default = String::from("button"))]
	r#type: String,
	onclick: Option<EventHandler<MouseEvent>>,
	children: Element,
) -> Element {
	let cls = cn!(
		button_classes(&variant.button_variant(), size, true, None, ""),
		ICON_BUTTON_BASE,
		if pressed == Some(true) { ICON_BUTTON_PRESSED } else { "" },
		class
	);
	let Some(tip) = tooltip else {
		return rsx! {
			IconButtonFace { aria_label, cls, variant, pressed, disabled, r#type, onclick, tipped: false, {children} }
		};
	};
	rsx! {
		Tooltip {
			IconButtonFace { aria_label, cls, variant, pressed, disabled, r#type, onclick, tipped: true, {children} }
			TooltipContent { {tip} }
		}
	}
}

#[component]
fn IconButtonFace(
	aria_label: String,
	cls: String,
	variant: IconButtonVariant,
	pressed: Option<bool>,
	disabled: bool,
	r#type: String,
	onclick: Option<EventHandler<MouseEvent>>,
	tipped: bool,
	children: Element,
) -> Element {
	// Only its own tooltip: a plain IconButton placed inside some other
	// tooltip's subtree must not open that one.
	let tip = try_use_context::<TooltipCtx>().filter(|_| tipped);
	let show = move |open: bool| {
		if let Some(t) = tip {
			t.open.set(open);
		}
	};
	rsx! {
		button {
			r#type,
			class: cls,
			"data-slot": "icon-button",
			"data-variant": variant.as_ref(),
			"aria-label": aria_label,
			"aria-pressed": pressed,
			disabled,
			onclick: move |e| { if let Some(h) = onclick { h.call(e); } },
			onmouseenter: move |_| show(true),
			onmouseleave: move |_| show(false),
			onfocus: move |_| show(true),
			onblur: move |_| show(false),
			{children}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::{
		ButtonVariant,
		test_util::{render, render_focused},
	};

	#[test]
	fn is_a_named_button_that_does_not_submit_forms() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "Поиск", svg {} } }
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Поиск\""), "{html}");
		assert!(html.contains("type=\"button\""), "{html}");
		assert!(html.contains("data-slot=\"icon-button\""), "{html}");
		assert!(html.contains("data-variant=\"ghost\""), "{html}");
	}

	#[test]
	fn a_caller_can_still_make_it_submit() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "Send", r#type: "submit", svg {} } }
		}
		let html = render(app);
		assert!(html.contains("type=\"submit\""), "{html}");
	}

	#[test]
	fn marks_the_toggled_on_state_for_assistive_tech_and_styling() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "Pin", pressed: true, svg {} } }
		}
		let html = render(app);
		assert!(html.contains("aria-pressed=true"), "{html}");
		assert!(html.contains("bg-active"), "{html}");
		// The ghost hover would otherwise wash the pressed fill out.
		assert!(!html.contains("hover:bg-hover"), "{html}");
	}

	#[test]
	fn an_unpressed_toggle_says_so() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "Pin", pressed: false, svg {} } }
		}
		let html = render(app);
		assert!(html.contains("aria-pressed=false"), "{html}");
		assert!(!html.contains("bg-active"), "{html}");
	}

	#[test]
	fn leaves_aria_pressed_off_when_it_is_not_a_toggle() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "Pin", svg {} } }
		}
		let html = render(app);
		assert!(!html.contains("aria-pressed"), "{html}");
	}

	/// The Dioxus stand-in for the TS ref test: with a tooltip there is still
	/// one button, and it is the icon button, not a trigger wrapped around it.
	#[test]
	fn the_tooltip_anchors_on_the_same_button() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "B", tooltip: rsx! { "tip" }, svg {} } }
		}
		let html = render(app);
		assert_eq!(html.matches("<button").count(), 1, "{html}");
		assert!(html.contains("data-slot=\"icon-button\""), "{html}");
		assert!(!html.contains("tooltip-trigger"), "{html}");
	}

	#[test]
	fn shows_its_tooltip_on_focus() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "Copy", tooltip: rsx! { "Скопировать ссылку" }, svg {} } }
		}
		assert!(!render(app).contains("role=\"tooltip\""));
		let html = render_focused(app);
		assert!(html.contains("role=\"tooltip\""), "{html}");
		assert!(html.contains("Скопировать ссылку"), "{html}");
	}

	#[test]
	fn a_plain_icon_button_does_not_open_an_enclosing_tooltip() {
		fn app() -> Element {
			rsx! {
				Tooltip {
					IconButton { aria_label: "Inner", svg {} }
					TooltipContent { "outer-tip" }
				}
			}
		}
		let html = render_focused(app);
		assert!(!html.contains("outer-tip"), "{html}");
	}

	#[test]
	fn wears_the_square_button_face() {
		let classes = |variant: IconButtonVariant, size: Size| cn!(button_classes(&variant.button_variant(), size, true, None, ""), ICON_BUTTON_BASE);
		let md = classes(IconButtonVariant::Ghost, Size::Md);
		assert!(md.contains("h-7") && md.contains("aspect-square") && md.contains("px-0"), "{md}");
		assert!(classes(IconButtonVariant::Ghost, Size::Sm).contains("h-6"));
		assert!(classes(IconButtonVariant::Ghost, Size::Touch).contains("size-11"));
		let secondary = classes(IconButtonVariant::Secondary, Size::Md);
		assert!(secondary.contains("bg-secondary") && secondary.contains("text-ink-mid"), "{secondary}");
		assert_eq!(IconButtonVariant::Secondary.button_variant(), ButtonVariant::Secondary);
	}

	#[test]
	fn class_override_wins() {
		fn app() -> Element {
			rsx! { IconButton { aria_label: "x", class: "text-primary-ink", svg {} } }
		}
		let html = render(app);
		assert!(html.contains("text-primary-ink"), "{html}");
		assert!(!html.contains(" text-ink-mid"), "{html}");
	}
}
