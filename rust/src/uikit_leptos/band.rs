//! Full-bleed page bands and the type vocabulary that goes in them.
//!
//! A page built from these writes no spacing or type-scale class of its own: the
//! rhythm is `--band-py`, the gutter `--page-px` and the headline scale
//! `--display-scale`, so a global retune is one edit in the consumer's tokens.

use leptos::prelude::*;
use tailwind_fuse::AsTailwindClass;

use crate::{
	cn,
	uikit_leptos::{CHECK, DISPLAY_BASE, EYEBROW, LEDE, PROSE, Polarity, SECTION_BASE, SECTION_HEAD, SECTION_PY, SECTION_PY_TIGHT, STAT, STAT_FIGURE, STAT_LABEL, Surface},
};

/// A full-bleed horizontal band. `polarity` sets the token scope on the element,
/// so everything inside resolves correctly on either side. Omitted, the band sets
/// no scope and inherits its parent's: a default of `Light` would flip every band
/// of a dark-first brand. `id` is set when the band is a scroll anchor; `tight` is
/// the shorter vertical rhythm.
#[component]
pub fn Section(
	#[prop(optional)] polarity: Option<Polarity>,
	#[prop(optional)] surface: Surface,
	#[prop(optional, into)] id: Option<String>,
	#[prop(optional)] tight: bool,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	// conflict-free by construction (gutter, scope, plane, rhythm), so only the caller override is merged
	let base = format!(
		"{SECTION_BASE} {} {} {}",
		polarity.as_ref().map_or("", AsTailwindClass::as_class),
		surface.as_class(),
		if tight { SECTION_PY_TIGHT } else { SECTION_PY }
	);
	let cls = cn!(base, class);
	view! {
		<section id=id class=cls data-slot="section">
			{children()}
		</section>
	}
}

/// The label above a headline, in the interactive role's colour.
#[component]
pub fn Eyebrow(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(EYEBROW, class);
	view! {
		<p class=cls data-slot="eyebrow">
			{children()}
		</p>
	}
}

/// The headline. One scale for every band on the site, so a retune is one number.
#[component]
pub fn Display(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(DISPLAY_BASE, class);
	view! {
		<h2 class=cls data-slot="display">
			{children()}
		</h2>
	}
}

/// Eyebrow → headline → lede, the head a band shares.
#[component]
pub fn SectionHead(#[prop(into)] eyebrow: String, #[prop(into)] title: String, #[prop(optional, into)] lede: Option<String>, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SECTION_HEAD, class);
	view! {
		<div class=cls data-slot="section-head">
			<Eyebrow>{eyebrow}</Eyebrow>
			<Display>{title}</Display>
			{lede.map(|lede| view! { <p class=LEDE>{lede}</p> })}
		</div>
	}
}

/// Body copy at the reading measure.
#[component]
pub fn Prose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(PROSE, class);
	view! {
		<p class=cls data-slot="prose">
			{children()}
		</p>
	}
}

/// A figure with its label beside it.
#[component]
pub fn Stat(#[prop(into)] figure: String, #[prop(into)] label: String, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(STAT, class);
	view! {
		<span class=cls data-slot="stat">
			<span class=STAT_FIGURE>{figure}</span>
			<span class=STAT_LABEL>{label}</span>
		</span>
	}
}

/// The affirmative tick. Positive is a valence, not a significance rung — see
/// the kit's `docs/spec/accents.md`.
#[component]
pub fn Check(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(CHECK, class);
	view! {
		<span class=cls data-slot="check" aria-hidden="true">
			"✓"
		</span>
	}
}
