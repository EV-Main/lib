//! The shared 404 / 403 / 500 status surface, so every app hosting the kit shows
//! the same error pages. The mark is [`Logo`](crate::uikit_leptos::Logo)'s, i.e.
//! the consumer's.
//!
//! [`StatusScreen`] is the generic shell; [`NotFound`] / [`Forbidden`] /
//! [`ServerError`] are the ready-made pages with their copy baked in — a host
//! renders those with its own hrefs. Links render as a plain `<a>` (a full
//! document load, which is what you want off an error page).

use ev_lib_classes::{
	Icon, STATUS_SCREEN_ACTION_ICON, STATUS_SCREEN_ACTIONS, STATUS_SCREEN_BODY, STATUS_SCREEN_CODE, STATUS_SCREEN_EYEBROW, STATUS_SCREEN_GLOW, STATUS_SCREEN_LOGO, STATUS_SCREEN_ROOT,
	STATUS_SCREEN_STACK, STATUS_SCREEN_TITLE, STATUS_SCREEN_TITLE_ACCENT,
};
use leptos::prelude::*;

pub use crate::ui::uikit::core::{StatusLinkData, status_cta_class};
use crate::{
	cn,
	ui::uikit::leptos::{Accent, ButtonVariant, Logo, accent_text_class, primitives::icon_shapes},
};

/// Shared skeleton for the 404 / 403 / 500 status pages: a centred hero with the
/// logo mark, a mono eyebrow, a giant Playfair code, a headline whose final clause
/// is an italic accent, supporting copy, and the CTAs. The accent threads through
/// all four marks. Actions render inline (from `links` data, plus an optional
/// `children` leading slot, e.g. the 500 client retry).
#[component]
pub fn StatusScreen(
	accent: Accent,
	#[prop(into)] eyebrow: String,
	#[prop(into)] code: String,
	#[prop(into)] headline_lead: String,
	#[prop(into)] headline_accent: String,
	#[prop(default = ".".to_string(), into)] headline_tail: String,
	#[prop(into)] subtext: String,
	#[prop(optional)] links: Vec<StatusLinkData>,
	/// Leading action slot, rendered before `links` (e.g. the 500 client retry).
	#[prop(optional)]
	children: Option<Children>,
) -> impl IntoView {
	let accent_text = accent_text_class(accent);
	let links = links
		.into_iter()
		.map(|link| {
			view! {
				<a href=link.href class=status_cta_class(accent, link.variant)>
					{link.leading_arrow.then(arrow_left_icon)}
					{link.label}
				</a>
			}
		})
		.collect_view();
	view! {
		<section class=STATUS_SCREEN_ROOT>
			<div aria-hidden="true" class=cn!(STATUS_SCREEN_GLOW,
					accent_text)></div>
			<div class=STATUS_SCREEN_STACK>
				<Logo class=cn!(STATUS_SCREEN_LOGO, accent_text) />
				<p class=cn!(STATUS_SCREEN_EYEBROW, accent_text)>{eyebrow}</p>
				<p class=cn!(STATUS_SCREEN_CODE, accent_text)>{code}</p>
				<h1 class=STATUS_SCREEN_TITLE>
					{headline_lead}
					<span class=cn!(STATUS_SCREEN_TITLE_ACCENT, accent_text)>{headline_accent}</span>
					{headline_tail}
				</h1>
				<p class=STATUS_SCREEN_BODY>{subtext}</p>
				<div class=STATUS_SCREEN_ACTIONS>{children.map(|c| c())} {links}</div>
			</div>
		</section>
	}
}

/// 404 — page not found.
#[component]
pub fn NotFound(#[prop(default = "/".to_string(), into)] home_href: String, #[prop(default = "/contact".to_string(), into)] contact_href: String) -> impl IntoView {
	view! {
		<StatusScreen
			accent=Accent::Debug
			eyebrow="Page not found"
			code="404"
			headline_lead="You've reached "
			headline_accent="open water"
			subtext="The page you're looking for has drifted off our coastline — moved, renamed, or never charted. Let's get you back to shore."
			links=vec![
				home("Back to home", home_href, ButtonVariant::Primary),
				contact("Contact the team", contact_href),
			]
		/>
	}
}

/// 403 — access forbidden.
#[component]
pub fn Forbidden(#[prop(default = "/".to_string(), into)] home_href: String, #[prop(default = "/contact".to_string(), into)] contact_href: String) -> impl IntoView {
	view! {
		<StatusScreen
			accent=Accent::Warn
			eyebrow="Access forbidden"
			code="403"
			headline_lead="This harbour is "
			headline_accent="private"
			subtext="You don't have the credentials to view this page. If you believe you should, our team can open the right doors."
			links=vec![
				home("Back to home", home_href, ButtonVariant::Primary),
				contact("Request access", contact_href),
			]
		/>
	}
}

/// 500 — server error. The "Try again" button runs `reset` or reloads the page.
#[component]
pub fn ServerError(#[prop(default = "/".to_string(), into)] home_href: String, #[prop(optional)] reset: Option<Callback<()>>) -> impl IntoView {
	view! {
		<StatusScreen
			accent=Accent::Error
			eyebrow="Server error"
			code="500"
			headline_lead="Our systems are "
			headline_accent="recalibrating"
			subtext="Something broke on our end — not yours. We've been alerted and are restoring service. Please try again in a moment."
			links=vec![home("Back to home", home_href, ButtonVariant::Outline)]
		>
			<button
				type="button"
				class=status_cta_class(Accent::Error, ButtonVariant::Primary)
				on:click=move |_| match reset {
					Some(cb) => cb.run(()),
					None => {
						window().location().reload().expect("reloading the current document cannot be refused")
					}
				}
			>
				"Try again"
			</button>
		</StatusScreen>
	}
}

fn home(label: &str, href: String, variant: ButtonVariant) -> StatusLinkData {
	StatusLinkData {
		label: label.to_string(),
		href,
		variant,
		leading_arrow: true,
	}
}

fn contact(label: &str, href: String) -> StatusLinkData {
	StatusLinkData {
		label: label.to_string(),
		href,
		variant: ButtonVariant::Outline,
		leading_arrow: false,
	}
}

fn arrow_left_icon() -> impl IntoView {
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class=STATUS_SCREEN_ACTION_ICON
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			{icon_shapes(Icon::ArrowLeft)}
		</svg>
	}
}
