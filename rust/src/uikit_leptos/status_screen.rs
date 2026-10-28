//! The shared 404 / 403 / 500 status surface, so every app hosting the kit shows
//! the same error pages. The mark is [`Logo`](crate::uikit_leptos::Logo)'s, i.e.
//! the consumer's.
//!
//! [`StatusScreen`] is the generic shell; [`NotFound`] / [`Forbidden`] /
//! [`ServerError`] are the ready-made pages with their copy baked in — a host
//! renders those with its own hrefs. Links render as a plain `<a>` (a full
//! document load, which is what you want off an error page).

use leptos::prelude::*;

pub use crate::uikit::{StatusLinkData, status_cta_class};
use crate::{
	cn,
	uikit_leptos::{Accent, ButtonVariant, Logo, accent_text_class},
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
		<section class="relative flex min-h-screen flex-col items-center justify-center overflow-hidden bg-background px-6 py-32 text-center">
			<div
				aria-hidden="true"
				class=cn!(
					"pointer-events-none absolute inset-0 opacity-[0.07] [background:radial-gradient(55%_45%_at_50%_30%,currentColor,transparent_70%)]",
					accent_text
				)
			></div>
			<div class="relative z-10 flex w-full max-w-2xl flex-col items-center">
				<Logo class=cn!("mb-7 h-10 w-auto", accent_text) />
				<p class=cn!(
					"mb-6 font-mono text-[11px] uppercase tracking-[0.34em]", accent_text
				)>{eyebrow}</p>
				<p class=cn!(
					"font-serif text-[110px] font-medium leading-[0.9] sm:text-[180px]", accent_text
				)>{code}</p>
				<h1 class="mt-4 font-serif text-3xl font-light leading-tight text-ink sm:text-5xl">
					{headline_lead} <span class=cn!("font-serif italic", accent_text)>{headline_accent}</span>
					{headline_tail}
				</h1>
				<p class="mx-auto mt-5 max-w-md text-sm leading-relaxed text-ink/60 sm:text-base">
					{subtext}
				</p>
				<div class="mt-9 flex flex-col items-center gap-3 sm:flex-row">
					{children.map(|c| c())} {links}
				</div>
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

// lucide `arrow-left`, inlined so the kit keeps its zero-icon-dep footprint.
fn arrow_left_icon() -> impl IntoView {
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			class="mr-2 h-4 w-4"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			<path d="m12 19-7-7 7-7" />
			<path d="M19 12H5" />
		</svg>
	}
}
