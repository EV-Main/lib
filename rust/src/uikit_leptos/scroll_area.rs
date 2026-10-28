use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{SCROLL_AREA_THUMB, SCROLL_AREA_VIEWPORT, SCROLLBAR_BASE, ScrollBarOrientation},
};

// Native `overflow` scrolling; `ScrollBar` is static decoration kept for class parity.

#[component]
pub fn ScrollArea(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!("relative", class);
	view! {
		<div class=cls data-slot="scroll-area">
			<div data-slot="scroll-area-viewport" class=SCROLL_AREA_VIEWPORT>
				{children()}
			</div>
			<ScrollBar />
		</div>
	}
}

#[component]
pub fn ScrollBar(#[prop(optional)] orientation: ScrollBarOrientation, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(SCROLLBAR_BASE, orientation.as_class(), class);
	view! {
		<div
			data-slot="scroll-area-scrollbar"
			data-orientation=orientation.as_ref().to_owned()
			class=cls
		>
			<div data-slot="scroll-area-thumb" class=SCROLL_AREA_THUMB></div>
		</div>
	}
}
