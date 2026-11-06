use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{
		SETTINGS_LAYOUT, SETTINGS_MAIN, SETTINGS_NAV, SETTINGS_ROW_INLINE, SETTINGS_SECTION_ACTION, SETTINGS_SECTION_BASE, SETTINGS_SECTION_BODY, SETTINGS_SECTION_BODY_FLUSH,
		SETTINGS_SECTION_BODY_PADDED, SETTINGS_SECTION_DESCRIPTION, SETTINGS_SECTION_HEADER, SETTINGS_SECTION_HEADER_FLUSH, SETTINGS_SECTION_HEADING, SETTINGS_SECTION_TITLE,
		SettingsSectionTone, form_row::FormRowFrame, primitives::use_stable_id,
	},
};

/// Rail on the left, cards in the main column. Mirrors the TS `SettingsLayout`.
/// The main column is a `div`, not a `<main>`: the app shell already renders
/// the page's `<main>`, and a page has only one.
#[component]
pub fn SettingsLayout(#[prop(into)] nav: ViewFn, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SETTINGS_LAYOUT, class);
	view! {
		<div class=cls data-slot="settings-layout">
			<div class=SETTINGS_NAV data-slot="settings-nav">
				{nav.run()}
			</div>
			<div class=SETTINGS_MAIN data-slot="settings-main">
				{children()}
			</div>
		</div>
	}
}

/// A settings card: title, a line saying what it is for, then the body.
/// Mirrors the TS `SettingsSection`. The title names the `<section>`, so the
/// card is a landmark region. `action` is a top-right control; `flush` lets
/// rows run edge to edge with their own hairlines.
///
/// Without children there is no body, so no flush hairline under the header.
/// Leptos cannot see whether children render anything, so an empty body is
/// told by their absence, where the Dioxus twin reads the vnode.
#[component]
pub fn SettingsSection(
	#[prop(into)] title: ViewFn,
	#[prop(optional, into)] description: Option<ViewFn>,
	#[prop(optional, into)] action: Option<ViewFn>,
	#[prop(optional)] tone: SettingsSectionTone,
	#[prop(optional)] flush: bool,
	#[prop(optional, into)] class: String,
	#[prop(optional)] children: Option<Children>,
) -> impl IntoView {
	let heading_id = use_stable_id("settings-section");
	let labelled_by = heading_id.clone();
	let cls = cn!(SETTINGS_SECTION_BASE, tone.as_class(), class);
	let header = cn!(SETTINGS_SECTION_HEADER, if flush { SETTINGS_SECTION_HEADER_FLUSH } else { "" });
	let body = cn!(SETTINGS_SECTION_BODY, if flush { SETTINGS_SECTION_BODY_FLUSH } else { SETTINGS_SECTION_BODY_PADDED });
	view! {
		<section
			class=cls
			data-slot="settings-section"
			data-tone=tone.as_ref().to_owned()
			aria-labelledby=labelled_by
		>
			<header class=header>
				<div class=SETTINGS_SECTION_HEADING>
					<h3 id=heading_id class=SETTINGS_SECTION_TITLE>
						{title.run()}
					</h3>
					{description.map(|d| view! { <p class=SETTINGS_SECTION_DESCRIPTION>{d.run()}</p> })}
				</div>
				{action.map(|a| view! { <div class=SETTINGS_SECTION_ACTION>{a.run()}</div> })}
			</header>
			{children.map(|c| view! { <div class=body>{c()}</div> })}
		</section>
	}
}

/// A form row for settings. Mirrors the TS `SettingsRow`. `inline` keeps label
/// and control on one line at every width, control at the end: for a switch or
/// a button.
#[component]
pub fn SettingsRow(
	#[prop(into)] label: ViewFn,
	#[prop(optional, into)] r#for: Option<String>,
	#[prop(optional, into)] description: Option<ViewFn>,
	#[prop(optional, into)] hint: Option<ViewFn>,
	#[prop(default = true)] divider: bool,
	#[prop(optional)] inline: bool,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let cls = cn!(if inline { SETTINGS_ROW_INLINE } else { "" }, class);
	view! {
		<FormRowFrame label r#for description hint divider inline=Some(inline) class=cls>
			{children()}
		</FormRowFrame>
	}
}
