use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{SCROLL_AREA_THUMB, SCROLL_AREA_VIEWPORT, SCROLLBAR_BASE, ScrollBarOrientation, Slot},
};

// Native `overflow` scrolling; `ScrollBar` is static decoration kept for class parity.

#[component]
pub fn ScrollArea(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!("relative", class);
	view! {
		<div class=cls data-slot=Slot::ScrollArea.as_str()>
			<div data-slot=Slot::ScrollAreaViewport.as_str() class=SCROLL_AREA_VIEWPORT>
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
			data-slot=Slot::ScrollAreaScrollbar.as_str()
			data-orientation=orientation.as_ref().to_owned()
			class=cls
		>
			<div data-slot=Slot::ScrollAreaThumb.as_str() class=SCROLL_AREA_THUMB></div>
		</div>
	}
}
