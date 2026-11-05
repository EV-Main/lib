use leptos::prelude::*;

use crate::{
	cn,
	uikit::{self, Drag},
	uikit_leptos::{SLIDER_RANGE, SLIDER_ROOT, SLIDER_THUMB, SLIDER_TRACK, SliderOrientation, primitives::use_controllable},
};

#[component]
pub fn Slider(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] value: MaybeProp<f64>,
	#[prop(optional)] default_value: f64,
	#[prop(optional)] on_value_change: Option<Callback<f64>>,
	#[prop(default = 0.0)] min: f64,
	#[prop(default = 100.0)] max: f64,
	#[prop(default = 1.0)] step: f64,
	#[prop(optional)] orientation: SliderOrientation,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] aria_label: Option<String>,
) -> impl IntoView {
	let state = use_controllable(value, default_value, on_value_change);
	let slider = uikit::Slider { min, max, step, orientation };
	let current = Memo::new(move |_| slider.clamp(state.get()));
	let ori: &'static str = orientation.into();
	let track = NodeRef::<leptos::html::Span>::new();
	let drag = StoredValue::new(Drag::default());
	let axis = move |e: &leptos::ev::PointerEvent| match orientation {
		SliderOrientation::Horizontal => f64::from(e.client_x()),
		SliderOrientation::Vertical => f64::from(e.client_y()),
	};
	let release = move |_| drag.update_value(Drag::release);

	// Pointer handling lives on the root, not the track: the thumb sits above the
	// track (absolute), so grabbing it must still start a drag.
	view! {
		<span
			class=cn!(SLIDER_ROOT, class)
			data-slot="slider"
			data-orientation=ori
			data-disabled=move || disabled.get().then_some("true")
			on:pointerdown=move |e| {
				if disabled.get_untracked() {
					return;
				}
				#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
				if let Some(el) = track.get_untracked() {
					let rect = el.get_bounding_client_rect();
					let measured = match orientation {
						SliderOrientation::Horizontal => {
							uikit::Track {
								origin: rect.left(),
								size: rect.width(),
							}
						}
						SliderOrientation::Vertical => {
							uikit::Track {
								origin: rect.top(),
								size: rect.height(),
							}
						}
					};
					drag.update_value(|d| d.press(measured));
					let _ = el.set_pointer_capture(e.pointer_id());
					state.set(slider.value_at(axis(&e), measured));
				}
				#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
				let _ = (e, track);
			}
			on:pointermove=move |e| {
				let mut moved = None;
				drag.update_value(|d| moved = d.moved(disabled.get_untracked(), e.buttons() != 0));
				if let Some(measured) = moved {
					state.set(slider.value_at(axis(&e), measured));
				}
			}
			on:pointerup=release
			on:pointerleave=release
			on:pointercancel=release
		>
			<span class=SLIDER_TRACK data-slot="slider-track" data-orientation=ori node_ref=track>
				<span
					class=SLIDER_RANGE
					data-slot="slider-range"
					data-orientation=ori
					style=move || slider.range_style(current.get())
				></span>
			</span>
			<span
				class=SLIDER_THUMB
				data-slot="slider-thumb"
				data-orientation=ori
				style=move || slider.thumb_style(current.get())
				role="slider"
				// The name belongs on the focusable `role="slider"` node: on the
				// role-less root it is prohibited and never announced.
				aria-label=aria_label
				tabindex=move || if disabled.get() { "-1" } else { "0" }
				aria-valuenow=move || current.get().to_string()
				aria-valuemin=min.to_string()
				aria-valuemax=max.to_string()
				aria-orientation=ori
				aria-disabled=move || disabled.get().then_some("true")
				on:keydown=move |e| {
					if disabled.get_untracked() {
						return;
					}
					if let Some(next) = slider.key(&(&e).into(), current.get_untracked()) {
						e.prevent_default();
						state.set(next);
					}
				}
			></span>
		</span>
	}
}
