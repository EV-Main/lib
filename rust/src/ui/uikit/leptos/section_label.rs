use leptos::prelude::*;

pub use crate::ui::uikit::core::SectionLabelAs;
use crate::{
	cn,
	ui::uikit::leptos::{SECTION_LABEL_BASE, SectionLabelTone},
};

/// The quiet label above a figure, a group or a section. Mirrors the TS
/// `SectionLabel` from `page-frame.tsx`.
#[component]
pub fn SectionLabel(#[prop(optional)] r#as: SectionLabelAs, #[prop(optional)] tone: SectionLabelTone, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(SECTION_LABEL_BASE, tone.as_class(), class);
	match r#as {
		SectionLabelAs::P => view! {
			<p class=cls data-slot="section-label">
				{children()}
			</p>
		}
		.into_any(),
		SectionLabelAs::Span => view! {
			<span class=cls data-slot="section-label">
				{children()}
			</span>
		}
		.into_any(),
		SectionLabelAs::Div => view! {
			<div class=cls data-slot="section-label">
				{children()}
			</div>
		}
		.into_any(),
		SectionLabelAs::H2 => view! {
			<h2 class=cls data-slot="section-label">
				{children()}
			</h2>
		}
		.into_any(),
		SectionLabelAs::H3 => view! {
			<h3 class=cls data-slot="section-label">
				{children()}
			</h3>
		}
		.into_any(),
		SectionLabelAs::H4 => view! {
			<h4 class=cls data-slot="section-label">
				{children()}
			</h4>
		}
		.into_any(),
	}
}
