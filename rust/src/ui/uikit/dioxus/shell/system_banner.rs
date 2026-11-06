use dioxus::prelude::*;
use ev_lib_classes::Icon;
pub use ev_lib_classes::SystemBannerLabels;

use crate::{
	cn,
	ui::uikit::dioxus::{
		SYSTEM_BANNER_ACTION, SYSTEM_BANNER_BASE, SYSTEM_BANNER_BODY_AFTER_TITLE, SYSTEM_BANNER_DISMISS, SYSTEM_BANNER_GLYPH, SYSTEM_BANNER_ICON, SYSTEM_BANNER_MESSAGE, SYSTEM_BANNER_TITLE,
		Tone,
		primitives::{has_content, icon_shapes},
	},
};

/// A notice strip. Mirrors the TS `SystemBanner`.
/// It only presents: where its state comes from, and whether a dismissal
/// sticks, are the caller's.
///
/// `icon` replaces the tone's glyph. The TS `icon={null}` drops the glyph; here
/// an empty `icon: rsx! {}` does the same, because a Dioxus prop cannot tell
/// "unset" from "null" without a nested `Option`.
#[component]
pub fn SystemBanner(
	#[props(default)] tone: Tone,
	/// A bold lead before the message.
	title: Option<Element>,
	icon: Option<Element>,
	/// Shows a close button.
	on_dismiss: Option<EventHandler<()>>,
	/// A link or button after the message.
	action: Option<Element>,
	#[props(default)] labels: SystemBannerLabels,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let cls = cn!(SYSTEM_BANNER_BASE, tone.as_class(), class);
	let glyph = match icon {
		Some(icon) => icon,
		None if tone == Tone::Warn => rsx! { TriangleAlertIcon {} },
		None => rsx! { InfoIcon {} },
	};
	let has_message = has_content(&children);
	let body_cls = title.as_ref().map(|_| SYSTEM_BANNER_BODY_AFTER_TITLE);
	let dismiss_label = labels.dismiss;
	rsx! {
		div { class: cls, "data-slot": "system-banner", "data-tone": tone.as_ref(), role: "status",
			if has_content(&glyph) {
				span { class: SYSTEM_BANNER_ICON, {glyph} }
			}
			div { class: SYSTEM_BANNER_MESSAGE,
				if let Some(title) = title {
					span { class: SYSTEM_BANNER_TITLE, {title} }
					if has_message {
						" — "
					}
				}
				if has_message {
					span { class: body_cls, {children} }
				}
			}
			if let Some(action) = action {
				span { class: SYSTEM_BANNER_ACTION, {action} }
			}
			if let Some(on_dismiss) = on_dismiss {
				button { r#type: "button", class: SYSTEM_BANNER_DISMISS, "aria-label": dismiss_label, onclick: move |_| on_dismiss.call(()), CloseIcon {} }
			}
		}
	}
}

#[component]
fn Glyph(icon: Icon) -> Element {
	rsx! {
		svg {
			class: SYSTEM_BANNER_GLYPH,
			xmlns: "http://www.w3.org/2000/svg",
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "2",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			"aria-hidden": "true",
			{icon_shapes(icon)}
		}
	}
}

#[component]
fn TriangleAlertIcon() -> Element {
	rsx! {
		Glyph { icon: Icon::TriangleAlert }
	}
}

#[component]
fn InfoIcon() -> Element {
	rsx! {
		Glyph { icon: Icon::Info }
	}
}

#[component]
fn CloseIcon() -> Element {
	rsx! {
		Glyph { icon: Icon::Close }
	}
}

#[cfg(test)]
mod tests {
	use tailwind_fuse::AsTailwindClass;

	use super::*;
	use crate::ui::uikit::dioxus::test_util::{render, render_after_click};

	/// The opening tag of the glyph's wrapper, so a test can check the tone ink
	/// lands on the icon and nowhere else.
	fn icon_wrapper(html: &str) -> &str {
		let at = html.find("<svg data-testid=\"glyph\"").expect("glyph");
		let open = html[..at].rfind("<span").expect("wrapper");
		&html[open..at]
	}

	#[test]
	fn the_strip_wears_the_tone_and_the_icon_inherits_it() {
		fn warn() -> Element {
			rsx! {
				SystemBanner { tone: Tone::Warn, icon: rsx! { svg { "data-testid": "glyph" } }, "Работы сегодня ночью" }
			}
		}
		let html = render(warn);
		let root = &html[..html.find('>').expect("root tag")];
		assert!(root.contains("role=\"status\""), "{html}");
		assert!(root.contains("data-tone=\"warn\""), "{html}");
		assert!(root.contains(Tone::Warn.as_class()), "{html}");
		assert!(!icon_wrapper(&html).contains("text-"), "{html}");
	}

	#[test]
	fn renders_an_action_after_the_message() {
		fn app() -> Element {
			rsx! { SystemBanner { action: rsx! { a { href: "#d", "Подробнее" } }, "Hello" } }
		}
		let html = render(app);
		let message = html.find("Hello").expect("message");
		let link = html.find("<a href=\"#d\">Подробнее</a>").expect("action link");
		assert!(message < link, "{html}");
		assert!(html.contains(&format!("<span class=\"{SYSTEM_BANNER_ACTION}\"><a")), "{html}");
	}

	#[test]
	fn defaults_to_neutral_with_the_info_glyph() {
		fn app() -> Element {
			rsx! { SystemBanner { "Hello" } }
		}
		let html = render(app);
		assert!(html.contains("data-tone=\"neutral\""), "{html}");
		assert!(html.contains("<circle"), "{html}");
	}

	#[test]
	fn warns_with_the_triangle_glyph() {
		fn app() -> Element {
			rsx! { SystemBanner { tone: Tone::Warn, "Только чтение" } }
		}
		let html = render(app);
		assert!(html.contains("M12 9v4"), "{html}");
		assert!(!html.contains("<circle"), "{html}");
	}

	#[test]
	fn an_empty_icon_drops_the_glyph() {
		fn app() -> Element {
			rsx! { SystemBanner { icon: rsx! {}, "Hello" } }
		}
		let html = render(app);
		assert!(!html.contains("<svg"), "{html}");
		assert!(!html.contains(SYSTEM_BANNER_ICON), "{html}");
	}

	#[test]
	fn joins_title_and_message_and_mutes_the_message() {
		fn app() -> Element {
			rsx! { SystemBanner { title: rsx! { "Обслуживание" }, "с 02:00 до 04:00" } }
		}
		let html = render(app);
		assert!(
			html.contains(&format!(
				"<span class=\"{SYSTEM_BANNER_TITLE}\">Обслуживание</span> — <span class=\"{SYSTEM_BANNER_BODY_AFTER_TITLE}\">с 02:00 до 04:00</span>"
			)),
			"{html}"
		);
	}

	#[test]
	fn a_title_alone_has_no_dash() {
		fn app() -> Element {
			rsx! { SystemBanner { title: rsx! { "Обслуживание" } } }
		}
		let html = render(app);
		assert!(!html.contains(" — "), "{html}");
		assert!(!html.contains(SYSTEM_BANNER_BODY_AFTER_TITLE), "{html}");
	}

	#[test]
	fn a_message_alone_keeps_the_full_ink() {
		fn app() -> Element {
			rsx! { SystemBanner { "Hello" } }
		}
		let html = render(app);
		assert!(html.contains("<span>Hello</span>"), "{html}");
		assert!(!html.contains(SYSTEM_BANNER_BODY_AFTER_TITLE), "{html}");
	}

	#[test]
	fn no_close_button_without_on_dismiss() {
		fn app() -> Element {
			rsx! { SystemBanner { "Hello" } }
		}
		assert!(!render(app).contains("<button"));
	}

	#[test]
	fn the_close_button_is_named_and_fires_on_dismiss() {
		fn app() -> Element {
			let mut gone = use_signal(|| false);
			if gone() {
				return rsx! { p { "dismissed" } };
			}
			rsx! { SystemBanner { on_dismiss: move |_| gone.set(true), "Hello" } }
		}
		let html = render(app);
		assert!(html.contains("<button type=\"button\""), "{html}");
		assert!(html.contains("aria-label=\"Dismiss\""), "{html}");
		assert!(render_after_click(app).contains("dismissed"));
	}

	#[test]
	fn the_dismiss_label_is_overridable() {
		fn app() -> Element {
			rsx! {
				SystemBanner { on_dismiss: |_| {}, labels: SystemBannerLabels { dismiss: "Закрыть".into() }, "Hello" }
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Закрыть\""), "{html}");
		assert!(!html.contains("Dismiss"), "{html}");
	}
}
