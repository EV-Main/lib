use ev_lib_classes::Icon;
pub use ev_lib_classes::SystemBannerLabels;
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{
		SYSTEM_BANNER_ACTION, SYSTEM_BANNER_BASE, SYSTEM_BANNER_BODY_AFTER_TITLE, SYSTEM_BANNER_DISMISS, SYSTEM_BANNER_GLYPH, SYSTEM_BANNER_ICON, SYSTEM_BANNER_MESSAGE, SYSTEM_BANNER_TITLE,
		Tone, primitives::icon_shapes,
	},
};

/// A notice strip. Mirrors the TS `SystemBanner`.
/// It only presents: where its state comes from, and whether a dismissal
/// sticks, are the caller's.
///
/// `icon` replaces the tone's glyph, and `icon=None` drops it, the TS
/// `icon={null}`. Leptos cannot see whether a view renders anything, so where
/// the Dioxus twin reads emptiness off its children, this one reads presence:
/// no `icon` glyph for `None`, no message without children.
#[component]
pub fn SystemBanner(
	#[prop(optional)] tone: Tone,
	/// A bold lead before the message.
	#[prop(optional, into)]
	title: Option<ViewFn>,
	#[prop(optional)] icon: Option<Option<ViewFn>>,
	/// Shows a close button.
	#[prop(optional)]
	on_dismiss: Option<Callback<()>>,
	/// A link or button after the message.
	#[prop(optional, into)]
	action: Option<ViewFn>,
	#[prop(optional)] labels: SystemBannerLabels,
	#[prop(optional, into)] class: String,
	#[prop(optional)] children: Option<Children>,
) -> impl IntoView {
	let cls = cn!(SYSTEM_BANNER_BASE, tone.as_class(), class);
	let glyph = match icon {
		Some(icon) => icon.map(|i| i.run()),
		None if tone == Tone::Warn => Some(view! { <TriangleAlertIcon /> }.into_any()),
		None => Some(view! { <InfoIcon /> }.into_any()),
	};
	let has_message = children.is_some();
	let body_cls = title.as_ref().map(|_| SYSTEM_BANNER_BODY_AFTER_TITLE);
	let dismiss_label = labels.dismiss;
	view! {
		<div class=cls data-slot="system-banner" data-tone=tone.as_ref().to_owned() role="status">
			{glyph.map(|g| view! { <span class=SYSTEM_BANNER_ICON>{g}</span> })}
			<div class=SYSTEM_BANNER_MESSAGE>
				{title
					.map(|t| {
						view! {
							<span class=SYSTEM_BANNER_TITLE>{t.run()}</span>
							{has_message.then_some(" — ")}
						}
					})}
				{children
					.map(|c| match body_cls {
						Some(cls) => view! { <span class=cls>{c()}</span> }.into_any(),
						None => view! { <span>{c()}</span> }.into_any(),
					})}
			</div>
			{action.map(|a| view! { <span class=SYSTEM_BANNER_ACTION>{a.run()}</span> })}
			{on_dismiss
				.map(|on_dismiss| {
					view! {
						<button
							type="button"
							class=SYSTEM_BANNER_DISMISS
							aria-label=dismiss_label
							on:click=move |_| on_dismiss.run(())
						>
							<CloseIcon />
						</button>
					}
				})}
		</div>
	}
}

#[component]
fn Glyph(icon: Icon) -> impl IntoView {
	view! {
		<svg
			class=SYSTEM_BANNER_GLYPH
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			{icon_shapes(icon)}
		</svg>
	}
}

#[component]
fn TriangleAlertIcon() -> impl IntoView {
	view! { <Glyph icon=Icon::TriangleAlert /> }
}

#[component]
fn InfoIcon() -> impl IntoView {
	view! { <Glyph icon=Icon::Info /> }
}

#[component]
fn CloseIcon() -> impl IntoView {
	view! { <Glyph icon=Icon::Close /> }
}
