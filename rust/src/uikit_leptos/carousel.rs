use std::sync::atomic::{AtomicUsize, Ordering};

use leptos::prelude::*;

use crate::{
	cn, uikit,
	uikit_leptos::{
		Button, ButtonVariant, CAROUSEL_CONTENT_TRACK, CAROUSEL_CONTENT_TRACK_HORIZONTAL, CAROUSEL_CONTENT_TRACK_VERTICAL, CAROUSEL_CONTENT_VIEWPORT, CAROUSEL_EDGE_FADE_NEXT,
		CAROUSEL_EDGE_FADE_PREV, CAROUSEL_ITEM, CAROUSEL_ITEM_HORIZONTAL, CAROUSEL_ITEM_VERTICAL, CAROUSEL_NAV, CAROUSEL_NEXT_HORIZONTAL, CAROUSEL_NEXT_VERTICAL,
		CAROUSEL_PREVIOUS_HORIZONTAL, CAROUSEL_PREVIOUS_VERTICAL, CarouselOrientation, Size,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn Carousel(
	#[prop(optional)] orientation: CarouselOrientation,
	#[prop(optional, into)] index: MaybeProp<usize>,
	#[prop(optional)] default_index: usize,
	#[prop(optional)] on_index_change: Option<Callback<usize>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let index = use_controllable(index, default_index, on_index_change);
	let ctx = CarouselContext {
		orientation,
		index,
		slides: RwSignal::new(uikit::Carousel::default()),
	};
	provide_context(ctx);
	let cls = cn!("relative", class);
	view! {
		<div
			class=cls
			role="region"
			aria-roledescription="carousel"
			tabindex="0"
			data-slot="carousel"
			on:keydown=move |e| {
				if let Some(to) = ctx.slides.with_untracked(|s| s.key(&(&e).into(), index.get_untracked()))
				{
					e.prevent_default();
					if let Some(to) = to {
						index.set(to);
					}
				}
			}
		>
			{children()}
		</div>
	}
}

#[component]
pub fn CarouselContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<CarouselContext>();
	// drag/momentum: omitted vs embla — see README Limitations
	let track = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_CONTENT_TRACK_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_CONTENT_TRACK_VERTICAL,
	};
	let cls = cn!(CAROUSEL_CONTENT_TRACK, track, class);
	view! {
		<div class=CAROUSEL_CONTENT_VIEWPORT data-slot="carousel-content">
			<div
				class=cls
				style=move || format!("transform: {}", ctx.orientation.transform(ctx.index.get()))
			>
				{children()}
			</div>
		</div>
	}
}

#[component]
pub fn CarouselItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	static NEXT: AtomicUsize = AtomicUsize::new(0);
	let ctx = expect_context::<CarouselContext>();
	// Registered as it renders, so the nav is already right in server-rendered markup.
	let id = NEXT.fetch_add(1, Ordering::Relaxed);
	ctx.slides.update(|s| s.register(id));
	on_cleanup(move || ctx.slides.update(|s| s.unregister(id)));
	let pad = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_ITEM_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_ITEM_VERTICAL,
	};
	let cls = cn!(CAROUSEL_ITEM, pad, class);
	view! {
		<div role="group" aria-roledescription="slide" data-slot="carousel-item" class=cls>
			{children()}
		</div>
	}
}

#[component]
pub fn CarouselPrevious(#[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = expect_context::<CarouselContext>();
	let pos = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_PREVIOUS_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_PREVIOUS_VERTICAL,
	};
	let cls = cn!(CAROUSEL_NAV, pos, class);
	view! {
		<Button
			variant=ButtonVariant::Outline
			size=Size::Md
			icon=true
			class=cls
			disabled=Signal::derive(move || !ctx.can_scroll_prev())
			on_click=Callback::new(move |_| ctx.scroll_prev())
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="m15 18-6-6 6-6"></path>
			</svg>
			<span class="sr-only">"Previous slide"</span>
		</Button>
	}
}

#[component]
pub fn CarouselNext(#[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = expect_context::<CarouselContext>();
	let pos = match ctx.orientation {
		CarouselOrientation::Horizontal => CAROUSEL_NEXT_HORIZONTAL,
		CarouselOrientation::Vertical => CAROUSEL_NEXT_VERTICAL,
	};
	let cls = cn!(CAROUSEL_NAV, pos, class);
	view! {
		<Button
			variant=ButtonVariant::Outline
			size=Size::Md
			icon=true
			class=cls
			disabled=Signal::derive(move || !ctx.can_scroll_next())
			on_click=Callback::new(move |_| ctx.scroll_next())
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				aria-hidden="true"
			>
				<path d="m9 18 6-6-6-6"></path>
			</svg>
			<span class="sr-only">"Next slide"</span>
		</Button>
	}
}

/// Netflix-style edge scrims that dissolve the current slide into the surface
/// colour, signalling adjacent slides without revealing them. Each side shows
/// only when there is somewhere to scroll. Drop inside a [`Carousel`].
#[component]
pub fn CarouselEdgeFade(#[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = expect_context::<CarouselContext>();
	let class = StoredValue::new(class);
	let shown = |on: bool| if on { "opacity-100" } else { "opacity-0" };
	view! {
		<div
			class=move || {
				class.with_value(|c| cn!(CAROUSEL_EDGE_FADE_PREV, shown(ctx.can_scroll_prev()), c))
			}
			aria-hidden="true"
		></div>
		<div
			class=move || {
				class.with_value(|c| cn!(CAROUSEL_EDGE_FADE_NEXT, shown(ctx.can_scroll_next()), c))
			}
			aria-hidden="true"
		></div>
	}
}

#[derive(Clone, Copy)]
struct CarouselContext {
	orientation: CarouselOrientation,
	index: Controllable<usize>,
	slides: RwSignal<uikit::Carousel>,
}

impl CarouselContext {
	fn can_scroll_prev(&self) -> bool {
		self.slides.with(|s| s.can_prev(self.index.get()))
	}

	fn can_scroll_next(&self) -> bool {
		self.slides.with(|s| s.can_next(self.index.get()))
	}

	fn scroll_prev(&self) {
		if let Some(to) = self.slides.with_untracked(|s| s.prev(self.index.get_untracked())) {
			self.index.set(to);
		}
	}

	fn scroll_next(&self) {
		if let Some(to) = self.slides.with_untracked(|s| s.next(self.index.get_untracked())) {
			self.index.set(to);
		}
	}
}
