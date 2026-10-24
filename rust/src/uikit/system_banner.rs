use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		SYSTEM_BANNER_ACTION, SYSTEM_BANNER_BASE, SYSTEM_BANNER_BODY_AFTER_TITLE, SYSTEM_BANNER_DISMISS, SYSTEM_BANNER_GLYPH, SYSTEM_BANNER_ICON, SYSTEM_BANNER_MESSAGE, SYSTEM_BANNER_TITLE,
		SystemBannerTone, primitives::has_content,
	},
};

/// The banner's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SystemBannerLabels {
	/// `aria-label` of the close button; "Dismiss".
	pub dismiss: Option<String>,
}

/// A slim full-width strip above the page. Mirrors the TS `SystemBanner`.
/// It only presents: where its state comes from, and whether a dismissal
/// sticks, are the caller's.
///
/// `icon` replaces the tone's glyph. The TS `icon={null}` drops the glyph; here
/// an empty `icon: rsx! {}` does the same, because a Dioxus prop cannot tell
/// "unset" from "null" without a nested `Option`.
#[component]
pub fn SystemBanner(
	#[props(default)] tone: SystemBannerTone,
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
	let cls = cn!(SYSTEM_BANNER_BASE, class);
	let glyph = match icon {
		Some(icon) => icon,
		None if tone == SystemBannerTone::Warn => rsx! { TriangleAlertIcon {} },
		None => rsx! { InfoIcon {} },
	};
	let icon_cls = cn!(SYSTEM_BANNER_ICON, tone.as_class());
	let has_message = has_content(&children);
	let body_cls = title.as_ref().map(|_| SYSTEM_BANNER_BODY_AFTER_TITLE);
	let dismiss_label = labels.dismiss.unwrap_or_else(|| String::from("Dismiss"));
	rsx! {
		div { class: cls, "data-slot": "system-banner", "data-tone": tone.as_ref(), role: "status",
			if has_content(&glyph) {
				span { class: icon_cls, {glyph} }
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
fn Glyph(paths: &'static [&'static str], circle: bool) -> Element {
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
			if circle {
				circle { cx: "12", cy: "12", r: "10" }
			}
			for d in paths {
				path { d: *d }
			}
		}
	}
}

#[component]
fn TriangleAlertIcon() -> Element {
	rsx! {
		Glyph { paths: &["m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3", "M12 9v4", "M12 17h.01"], circle: false }
	}
}

#[component]
fn InfoIcon() -> Element {
	rsx! {
		Glyph { paths: &["M12 16v-4", "M12 8h.01"], circle: true }
	}
}

#[component]
fn CloseIcon() -> Element {
	rsx! {
		Glyph { paths: &["M18 6 6 18", "m6 6 12 12"], circle: false }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::{render, render_after_click};

	/// The opening tag of the glyph's wrapper, so a test can check the tone ink
	/// lands on the icon and nowhere else.
	fn icon_wrapper(html: &str) -> &str {
		let at = html.find("<svg data-testid=\"glyph\"").expect("glyph");
		let open = html[..at].rfind("<span").expect("wrapper");
		&html[open..at]
	}

	#[test]
	fn keeps_the_strip_on_the_card_fill_and_colours_only_the_icon() {
		fn app() -> Element {
			rsx! {
				SystemBanner { tone: SystemBannerTone::Warn, icon: rsx! { svg { "data-testid": "glyph" } }, "Работы сегодня ночью" }
			}
		}
		let html = render(app);
		let root = &html[..html.find('>').expect("root tag")];
		assert!(root.contains("role=\"status\""), "{html}");
		assert!(root.contains("bg-card") && root.contains("border-b"), "{html}");
		assert!(!html.contains("bg-accent-warn/10"), "{html}");
		assert!(root.contains("data-tone=\"warn\""), "{html}");
		assert!(icon_wrapper(&html).contains("text-accent-warn-ink"), "{html}");
		assert!(!root.contains("text-accent-warn-ink"), "{html}");
	}

	#[test]
	fn tints_the_icon_per_tone() {
		fn neutral() -> Element {
			rsx! { SystemBanner { tone: SystemBannerTone::Neutral, icon: rsx! { svg { "data-testid": "glyph" } }, "Hello" } }
		}
		fn primary() -> Element {
			rsx! { SystemBanner { tone: SystemBannerTone::Primary, icon: rsx! { svg { "data-testid": "glyph" } }, "Hello" } }
		}
		fn info() -> Element {
			rsx! { SystemBanner { tone: SystemBannerTone::Info, icon: rsx! { svg { "data-testid": "glyph" } }, "Hello" } }
		}
		for (app, cls) in [(neutral as fn() -> Element, "text-ink-soft"), (primary, "text-primary-ink"), (info, "text-accent-info-ink")] {
			let html = render(app);
			assert!(icon_wrapper(&html).contains(cls), "{cls}: {html}");
		}
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
	fn defaults_to_info_with_the_info_glyph() {
		fn app() -> Element {
			rsx! { SystemBanner { "Hello" } }
		}
		let html = render(app);
		assert!(html.contains("data-tone=\"info\""), "{html}");
		assert!(html.contains("<circle"), "{html}");
		assert!(html.contains("text-accent-info-ink"), "{html}");
	}

	#[test]
	fn warns_with_the_triangle_glyph() {
		fn app() -> Element {
			rsx! { SystemBanner { tone: SystemBannerTone::Warn, "Только чтение" } }
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
				SystemBanner { on_dismiss: |_| {}, labels: SystemBannerLabels { dismiss: Some("Закрыть".into()) }, "Hello" }
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Закрыть\""), "{html}");
		assert!(!html.contains("Dismiss"), "{html}");
	}
}
