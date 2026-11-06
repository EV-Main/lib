//! The shared 404 / 403 / 500 status surface, ported from site_conductor so every
//! app hosting the kit shows the same error pages. The mark is
//! [`Logo`](crate::uikit_dioxus::Logo)'s, i.e. the consumer's.
//!
//! [`StatusScreen`] is the generic shell; [`NotFound`] / [`Forbidden`] /
//! [`ServerError`] are the ready-made pages with their copy baked in — a host
//! renders those with its own hrefs. Links render as a plain `<a>` (a full
//! document load, which is what you want off an error page); the TS port's
//! `linkComponent` soft-nav affordance has no Dioxus equivalent (see the package
//! README's Rust↔TS notes), matching how site_conductor's `Header` and
//! [`Footer`](crate::uikit_dioxus::Footer) render their links.

use dioxus::prelude::*;
use ev_lib_classes::{
	Icon, STATUS_SCREEN_ACTION_ICON, STATUS_SCREEN_ACTIONS, STATUS_SCREEN_BODY, STATUS_SCREEN_CODE, STATUS_SCREEN_EYEBROW, STATUS_SCREEN_GLOW, STATUS_SCREEN_LOGO, STATUS_SCREEN_ROOT,
	STATUS_SCREEN_STACK, STATUS_SCREEN_TITLE, STATUS_SCREEN_TITLE_ACCENT, copy,
};

pub use crate::ui::uikit::core::{StatusLinkData, status_cta_class};
use crate::{
	cn,
	ui::uikit::dioxus::{Accent, ButtonVariant, Logo, accent_text_class, primitives::icon_shapes},
};

/// Shared skeleton for the 404 / 403 / 500 status pages: a centred hero with the
/// logo mark, a mono eyebrow, a giant Playfair code, a headline whose final clause
/// is an italic accent, supporting copy, and the CTAs. The accent threads through
/// all four marks. Actions render inline (from `links` data, plus an optional
/// `children` leading slot, e.g. the 500 client retry).
#[component]
pub fn StatusScreen(
	accent: Accent,
	eyebrow: String,
	code: String,
	headline_lead: String,
	headline_accent: String,
	#[props(default = ".".to_string())] headline_tail: String,
	subtext: String,
	#[props(default)] links: Vec<StatusLinkData>,
	/// Leading action slot, rendered before `links` (e.g. the 500 client retry).
	children: Element,
) -> Element {
	let accent_text = accent_text_class(accent);
	rsx! {
		section { class: STATUS_SCREEN_ROOT,
			div {
				aria_hidden: "true",
				class: cn!(
					STATUS_SCREEN_GLOW,
					accent_text
				),
			}
			div { class: STATUS_SCREEN_STACK,
				Logo { class: cn!(STATUS_SCREEN_LOGO, accent_text) }
				p { class: cn!(STATUS_SCREEN_EYEBROW, accent_text), {eyebrow} }
				p { class: cn!(STATUS_SCREEN_CODE, accent_text), {code} }
				h1 { class: STATUS_SCREEN_TITLE,
					{headline_lead}
					span { class: cn!(STATUS_SCREEN_TITLE_ACCENT, accent_text), {headline_accent} }
					{headline_tail}
				}
				p { class: STATUS_SCREEN_BODY, {subtext} }
				div { class: STATUS_SCREEN_ACTIONS,
					{children}
					for link in links.iter() {
						a {
							key: "{link.label}",
							href: link.href.clone(),
							class: status_cta_class(accent, link.variant),
							if link.leading_arrow {
								{arrow_left_icon()}
							}
							{link.label.clone()}
						}
					}
				}
			}
		}
	}
}
/// 404 — page not found.
#[component]
pub fn NotFound(#[props(default = "/".to_string())] home_href: String, #[props(default = "/contact".to_string())] contact_href: String) -> Element {
	rsx! {
		StatusScreen {
			accent: Accent::Debug,
			eyebrow: copy::NOT_FOUND_EYEBROW,
			code: "404",
			headline_lead: copy::NOT_FOUND_HEADLINE_LEAD,
			headline_accent: copy::NOT_FOUND_HEADLINE_ACCENT,
			subtext: copy::NOT_FOUND_SUBTEXT,
			links: vec![
				StatusLinkData {
					label: copy::STATUS_HOME.to_string(),
					href: home_href,
					variant: ButtonVariant::Primary,
					leading_arrow: true,
				},
				StatusLinkData {
					label: copy::NOT_FOUND_CONTACT.to_string(),
					href: contact_href,
					variant: ButtonVariant::Outline,
					leading_arrow: false,
				},
			],
		}
	}
}
/// 403 — access forbidden.
#[component]
pub fn Forbidden(#[props(default = "/".to_string())] home_href: String, #[props(default = "/contact".to_string())] contact_href: String) -> Element {
	rsx! {
		StatusScreen {
			accent: Accent::Warn,
			eyebrow: copy::FORBIDDEN_EYEBROW,
			code: "403",
			headline_lead: copy::FORBIDDEN_HEADLINE_LEAD,
			headline_accent: copy::FORBIDDEN_HEADLINE_ACCENT,
			subtext: copy::FORBIDDEN_SUBTEXT,
			links: vec![
				StatusLinkData {
					label: copy::STATUS_HOME.to_string(),
					href: home_href,
					variant: ButtonVariant::Primary,
					leading_arrow: true,
				},
				StatusLinkData {
					label: copy::FORBIDDEN_CONTACT.to_string(),
					href: contact_href,
					variant: ButtonVariant::Outline,
					leading_arrow: false,
				},
			],
		}
	}
}
/// 500 — server error. The "Try again" button runs `reset` or reloads the page.
#[component]
pub fn ServerError(#[props(default = "/".to_string())] home_href: String, reset: Option<EventHandler<()>>) -> Element {
	rsx! {
		StatusScreen {
			accent: Accent::Error,
			eyebrow: copy::SERVER_ERROR_EYEBROW,
			code: "500",
			headline_lead: copy::SERVER_ERROR_HEADLINE_LEAD,
			headline_accent: copy::SERVER_ERROR_HEADLINE_ACCENT,
			subtext: copy::SERVER_ERROR_SUBTEXT,
			links: vec![
				StatusLinkData {
					label: copy::STATUS_HOME.to_string(),
					href: home_href,
					variant: ButtonVariant::Outline,
					leading_arrow: true,
				},
			],
			button {
				r#type: "button",
				class: status_cta_class(Accent::Error, ButtonVariant::Primary),
				onclick: move |_| {
					if let Some(cb) = reset {
						cb.call(());
					} else {
						let _ = document::eval("window.location.reload();");
					}
				},
				{copy::SERVER_ERROR_RETRY}
			}
		}
	}
}
fn arrow_left_icon() -> Element {
	rsx! {
		svg {
			xmlns: "http://www.w3.org/2000/svg",
			class: STATUS_SCREEN_ACTION_ICON,
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "2",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			"aria-hidden": "true",
			{icon_shapes(Icon::ArrowLeft)}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

	#[test]
	fn status_screen_renders_code_eyebrow_headline_and_link() {
		fn app() -> Element {
			rsx! {
				StatusScreen {
					accent: Accent::Debug,
					eyebrow: "Page not found",
					code: "404",
					headline_lead: "You've reached ",
					headline_accent: "open water",
					subtext: "drifted off",
					links: vec![StatusLinkData {
						label: "Back to home".to_string(),
						href: "/".to_string(),
						variant: ButtonVariant::Primary,
						leading_arrow: true,
					}],
				}
			}
		}
		let html = render(app);
		assert!(html.contains("404"), "{html}");
		assert!(html.contains("Page not found"), "{html}");
		assert!(html.contains("open water"), "{html}");
		assert!(html.contains("<a"), "CTA is a plain anchor: {html}");
		assert!(html.contains("href=\"/\""), "{html}");
	}

	#[test]
	fn ready_made_pages_bake_in_code_and_hrefs() {
		fn not_found() -> Element {
			rsx! {
				NotFound { home_href: "/", contact_href: "/contact" }
			}
		}
		let html = render(not_found);
		assert!(html.contains("404"), "{html}");
		assert!(html.contains("Back to home"), "{html}");
		assert!(html.contains("href=\"/contact\""), "{html}");
		assert!(html.contains("Contact the team"), "{html}");

		fn forbidden() -> Element {
			rsx! {
				Forbidden {}
			}
		}
		let html = render(forbidden);
		assert!(html.contains("403"), "{html}");
		assert!(html.contains("Request access"), "{html}");
	}

	#[test]
	fn server_error_shows_500_and_retry_button() {
		fn app() -> Element {
			rsx! {
				ServerError {}
			}
		}
		let html = render(app);
		assert!(html.contains("500"), "{html}");
		assert!(html.contains("Try again"), "{html}");
		assert!(html.contains("<button"), "retry is a button: {html}");
	}
}
