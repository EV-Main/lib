use ev_lib_classes::{SECTION_DESCRIPTOR, SECTION_DESCRIPTOR_SUMMARY};
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{FIELD_DESCRIPTION, FIELD_TITLE},
};

#[component]
pub fn SectionDescriptor(
	#[prop(into)] title: String,
	#[prop(default = true)] icon: bool,
	#[prop(optional)] collapsible: bool,
	#[prop(optional)] default_open: bool,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let cls = cn!(SECTION_DESCRIPTOR, class);
	let glyph = icon.then(info_glyph);
	if collapsible {
		let summary_cls = cn!(FIELD_TITLE, SECTION_DESCRIPTOR_SUMMARY);
		let body_cls = cn!(FIELD_DESCRIPTION, "mt-2");
		view! {
			<details
				class=cls
				data-slot="section-descriptor"
				data-collapsible="true"
				open=default_open.then_some("true")
			>
				<summary class=summary_cls data-slot="section-descriptor-title">
					{glyph}
					<span>{title}</span>
				</summary>
				<p class=body_cls data-slot="section-descriptor-body">
					{children()}
				</p>
			</details>
		}
		.into_any()
	} else {
		view! {
			<section class=cls data-slot="section-descriptor">
				<div class=FIELD_TITLE data-slot="section-descriptor-title">
					{glyph}
					<span>{title}</span>
				</div>
				<p class=FIELD_DESCRIPTION data-slot="section-descriptor-body">
					{children()}
				</p>
			</section>
		}
		.into_any()
	}
}

// lucide `info`, inlined per the kit's no-lucide-dep icon convention.
fn info_glyph() -> impl IntoView {
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			class="size-4 shrink-0 text-accent-debug"
			aria-hidden="true"
		>
			<circle cx="12" cy="12" r="10" />
			<path d="M12 16v-4" />
			<path d="M12 8h.01" />
		</svg>
	}
}
