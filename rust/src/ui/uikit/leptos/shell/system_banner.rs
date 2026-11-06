use leptos::prelude::*;

pub use crate::ui::uikit::core::SystemBannerLabels;
use crate::{
	cn,
	ui::uikit::leptos::{
		SYSTEM_BANNER_ACTION, SYSTEM_BANNER_BASE, SYSTEM_BANNER_BODY_AFTER_TITLE, SYSTEM_BANNER_DISMISS, SYSTEM_BANNER_GLYPH, SYSTEM_BANNER_ICON, SYSTEM_BANNER_MESSAGE, SYSTEM_BANNER_TITLE,
		SystemBannerTone, system_banner_tone_class,
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
	#[prop(optional)] tone: SystemBannerTone,
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
	let cls = cn!(SYSTEM_BANNER_BASE, system_banner_tone_class(&tone), class);
	let glyph = match icon {
		Some(icon) => icon.map(|i| i.run()),
		None if tone == SystemBannerTone::Warn => Some(view! { <TriangleAlertIcon /> }.into_any()),
		None => Some(view! { <InfoIcon /> }.into_any()),
	};
	let icon_cls = cn!(SYSTEM_BANNER_ICON, tone.as_class());
	let has_message = children.is_some();
	let body_cls = title.as_ref().map(|_| SYSTEM_BANNER_BODY_AFTER_TITLE);
	let dismiss_label = labels.dismiss.unwrap_or_else(|| String::from("Dismiss"));
	view! {
		<div class=cls data-slot="system-banner" data-tone=tone.as_ref().to_owned() role="status">
			{glyph.map(|g| view! { <span class=icon_cls>{g}</span> })}
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
fn Glyph(paths: &'static [&'static str], circle: bool) -> impl IntoView {
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
			{circle.then(|| view! { <circle cx="12" cy="12" r="10" /> })}
			{paths.iter().map(|d| view! { <path d=*d /> }).collect_view()}
		</svg>
	}
}

#[component]
fn TriangleAlertIcon() -> impl IntoView {
	view! {
		<Glyph
			paths=&[
				"m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3",
				"M12 9v4",
				"M12 17h.01",
			]
			circle=false
		/>
	}
}

#[component]
fn InfoIcon() -> impl IntoView {
	view! { <Glyph paths=&["M12 16v-4", "M12 8h.01"] circle=true /> }
}

#[component]
fn CloseIcon() -> impl IntoView {
	view! { <Glyph paths=&["M18 6 6 18", "m6 6 12 12"] circle=false /> }
}
