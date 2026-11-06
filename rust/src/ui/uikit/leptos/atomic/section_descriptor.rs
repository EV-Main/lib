use ev_lib_classes::{Icon, SECTION_DESCRIPTOR, SECTION_DESCRIPTOR_SUMMARY};
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{FIELD_DESCRIPTION, FIELD_TITLE, Slot, primitives::icon_shapes},
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
				data-slot=Slot::SectionDescriptor.as_str()
				data-collapsible="true"
				open=default_open.then_some("true")
			>
				<summary class=summary_cls data-slot=Slot::SectionDescriptorTitle.as_str()>
					{glyph}
					<span>{title}</span>
				</summary>
				<p class=body_cls data-slot=Slot::SectionDescriptorBody.as_str()>
					{children()}
				</p>
			</details>
		}
		.into_any()
	} else {
		view! {
			<section class=cls data-slot=Slot::SectionDescriptor.as_str()>
				<div class=FIELD_TITLE data-slot=Slot::SectionDescriptorTitle.as_str()>
					{glyph}
					<span>{title}</span>
				</div>
				<p class=FIELD_DESCRIPTION data-slot=Slot::SectionDescriptorBody.as_str()>
					{children()}
				</p>
			</section>
		}
		.into_any()
	}
}

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
			{icon_shapes(Icon::Info)}
		</svg>
	}
}
