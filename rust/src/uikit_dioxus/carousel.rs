use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{
	cn, uikit,
	uikit_dioxus::{
		ButtonVariant, CAROUSEL_CONTENT_TRACK, CAROUSEL_CONTENT_TRACK_HORIZONTAL, CAROUSEL_CONTENT_TRACK_VERTICAL, CAROUSEL_CONTENT_VIEWPORT, CAROUSEL_EDGE_FADE_NEXT,
		CAROUSEL_EDGE_FADE_PREV, CAROUSEL_ITEM, CAROUSEL_ITEM_HORIZONTAL, CAROUSEL_ITEM_VERTICAL, CAROUSEL_NAV, CAROUSEL_NEXT_HORIZONTAL, CAROUSEL_NEXT_VERTICAL,
		CAROUSEL_PREVIOUS_HORIZONTAL, CAROUSEL_PREVIOUS_VERTICAL, CarouselOrientation, Size,
		button::Button,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn Carousel(
	#[props(default)] orientation: CarouselOrientation,
	index: Option<usize>,
	#[props(default)] default_index: usize,
	on_index_change: Option<EventHandler<usize>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let index = use_controllable(index, default_index, on_index_change);
	let slides = use_signal(uikit::Carousel::default);
	let ctx = use_context_provider(|| CarouselContext { orientation, index, slides });
	let cls = cn!("relative", class);
	rsx! {
		div {
			class: cls,
			role: "region",
			"aria-roledescription": "carousel",
			tabindex: "0",
			"data-slot": "carousel",
			onkeydown: move |e| {
				if let Some(to) = ctx.slides.read().key(&(&e).into(), ctx.index.get()) {
					e.prevent_default();
					if let Some(to) = to {
						ctx.index.set(to);
					}
				}
			},
			{children}
		}
	}
}
#[component]
pub fn CarouselContent(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<CarouselContext>();
	let index = ctx.index.get();
	// drag/momentum: omitted vs embla — see README Limitations
	let transform = ctx.orientation.transform(index);
	let track = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_CONTENT_TRACK_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_CONTENT_TRACK_VERTICAL,
	};
	let cls = cn!(CAROUSEL_CONTENT_TRACK, track, class);
	rsx! {
		div { class: CAROUSEL_CONTENT_VIEWPORT, "data-slot": "carousel-content",
			div { class: cls, style: "transform: {transform}", {children} }
		}
	}
}
#[component]
pub fn CarouselItem(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<CarouselContext>();
	// Registered as it renders, so the nav after the slides is already right in
	// server-rendered markup.
	let id = use_hook(|| {
		static NEXT: AtomicUsize = AtomicUsize::new(0);
		let id = NEXT.fetch_add(1, Ordering::Relaxed);
		let mut slides = ctx.slides;
		slides.write().register(id);
		id
	});
	use_drop(move || {
		let mut slides = ctx.slides;
		slides.write().unregister(id);
	});
	let pad = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_ITEM_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_ITEM_VERTICAL,
	};
	let cls = cn!(CAROUSEL_ITEM, pad, class);
	rsx! {
		div {
			role: "group",
			"aria-roledescription": "slide",
			"data-slot": "carousel-item",
			class: cls,
			{children}
		}
	}
}
#[component]
pub fn CarouselPrevious(#[props(default)] class: String) -> Element {
	let ctx = use_context::<CarouselContext>();
	let pos = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_PREVIOUS_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_PREVIOUS_VERTICAL,
	};
	let cls = cn!(CAROUSEL_NAV, pos, class);
	rsx! {
		Button {
			variant: ButtonVariant::Outline,
			size: Size::Md,
			icon: true,
			class: cls,
			disabled: !ctx.can_scroll_prev(),
			onclick: move |_| ctx.scroll_prev(),
			svg {
				xmlns: "http://www.w3.org/2000/svg",
				view_box: "0 0 24 24",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "2",
				stroke_linecap: "round",
				stroke_linejoin: "round",
				"aria-hidden": "true",
				path { d: "m15 18-6-6 6-6" }
			}
			span { class: "sr-only", "Previous slide" }
		}
	}
}
#[component]
pub fn CarouselNext(#[props(default)] class: String) -> Element {
	let ctx = use_context::<CarouselContext>();
	let pos = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_NEXT_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_NEXT_VERTICAL,
	};
	let cls = cn!(CAROUSEL_NAV, pos, class);
	rsx! {
		Button {
			variant: ButtonVariant::Outline,
			size: Size::Md,
			icon: true,
			class: cls,
			disabled: !ctx.can_scroll_next(),
			onclick: move |_| ctx.scroll_next(),
			svg {
				xmlns: "http://www.w3.org/2000/svg",
				view_box: "0 0 24 24",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "2",
				stroke_linecap: "round",
				stroke_linejoin: "round",
				"aria-hidden": "true",
				path { d: "m9 18 6-6-6-6" }
			}
			span { class: "sr-only", "Next slide" }
		}
	}
}
/// Netflix-style edge scrims that dissolve the current slide into the surface
/// colour, signalling adjacent slides without revealing them. Each side shows
/// only when there is somewhere to scroll. Drop inside a [`Carousel`].
#[component]
pub fn CarouselEdgeFade(#[props(default)] class: String) -> Element {
	let ctx = use_context::<CarouselContext>();
	let prev = cn!(CAROUSEL_EDGE_FADE_PREV, if ctx.can_scroll_prev() { "opacity-100" } else { "opacity-0" }, class.clone());
	let next = cn!(CAROUSEL_EDGE_FADE_NEXT, if ctx.can_scroll_next() { "opacity-100" } else { "opacity-0" }, class);
	rsx! {
		div { class: prev, "aria-hidden": "true" }
		div { class: next, "aria-hidden": "true" }
	}
}
#[derive(Clone, Copy)]
struct CarouselContext {
	orientation: CarouselOrientation,
	index: Controllable<usize>,
	slides: Signal<uikit::Carousel>,
}

impl CarouselContext {
	fn can_scroll_prev(&self) -> bool {
		self.slides.read().can_prev(self.index.get())
	}

	fn can_scroll_next(&self) -> bool {
		self.slides.read().can_next(self.index.get())
	}

	fn scroll_prev(&self) {
		let to = self.slides.read().prev(self.index.get());
		if let Some(to) = to {
			self.index.set(to);
		}
	}

	fn scroll_next(&self) {
		let to = self.slides.read().next(self.index.get());
		if let Some(to) = to {
			self.index.set(to);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::test_util::render;

	#[test]
	fn carousel_renders_region_and_slots() {
		fn app() -> Element {
			rsx! {
				Carousel {
					CarouselContent {
						CarouselItem { "a" }
						CarouselItem { "b" }
					}
					CarouselPrevious {}
					CarouselNext {}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"carousel\""), "{html}");
		assert!(html.contains("aria-roledescription=\"carousel\""), "{html}");
		assert!(html.contains("data-slot=\"carousel-content\""), "{html}");
		assert!(html.contains("data-slot=\"carousel-item\""), "{html}");
		assert!(html.contains("basis-full"), "{html}");
	}

	#[test]
	fn item_uses_horizontal_padding_by_default() {
		fn app() -> Element {
			rsx! {
				Carousel {
					CarouselItem { "a" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("pl-4"), "{html}");
	}

	#[test]
	fn previous_disabled_at_start() {
		fn app() -> Element {
			rsx! {
				Carousel {
					CarouselPrevious {}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"button\""), "{html}");
		assert!(html.contains("disabled=true"), "prev should be disabled at index 0: {html}");
		assert!(html.contains("m15 18-6-6 6-6"), "{html}");
	}

	#[test]
	fn next_is_enabled_while_a_slide_follows() {
		fn app() -> Element {
			rsx! {
				Carousel {
					CarouselContent {
						CarouselItem { "a" }
						CarouselItem { "b" }
					}
					CarouselNext {}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("m9 18 6-6-6-6"), "{html}");
		assert!(!html.contains("disabled=true"), "two slides at index 0 can scroll on: {html}");
	}

	#[test]
	fn content_transform_translates_by_index() {
		fn app() -> Element {
			rsx! {
				Carousel { index: 1,
					CarouselContent {
						CarouselItem { "a" }
						CarouselItem { "b" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("translate3d(-100%, 0, 0)"), "{html}");
	}
}
