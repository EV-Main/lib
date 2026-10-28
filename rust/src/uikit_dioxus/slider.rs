use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{self, Drag, Track},
	uikit_dioxus::{SLIDER_RANGE, SLIDER_ROOT, SLIDER_THUMB, SLIDER_TRACK, SliderOrientation, primitives::use_controllable},
};

#[component]
pub fn Slider(
	#[props(default)] class: String,
	value: Option<f64>,
	#[props(default)] default_value: f64,
	on_value_change: Option<EventHandler<f64>>,
	#[props(default = 0.0)] min: f64,
	#[props(default = 100.0)] max: f64,
	#[props(default = 1.0)] step: f64,
	#[props(default)] orientation: SliderOrientation,
	#[props(default)] disabled: bool,
	aria_label: Option<String>,
) -> Element {
	let state = use_controllable(value, default_value, on_value_change);
	let slider = uikit::Slider { min, max, step, orientation };
	let current = slider.clamp(state.get());
	let ori: &str = orientation.as_ref();

	// Track geometry, measured on each press, lets a pointer drag map cursor
	// position → value (the TS port reads `getBoundingClientRect`; we use Dioxus'
	// renderer-agnostic `get_client_rect`).
	let mut track = use_signal(|| Option::<std::rc::Rc<MountedData>>::None);
	let mut drag = use_signal(Drag::default);

	let on_key = move |e: KeyboardEvent| {
		if disabled {
			return;
		}
		if let Some(next) = slider.key(&(&e).into(), current) {
			e.prevent_default();
			state.set(next);
		}
	};

	let axis = move |e: &PointerEvent| match orientation {
		SliderOrientation::Horizontal => e.client_coordinates().x,
		SliderOrientation::Vertical => e.client_coordinates().y,
	};

	// Pointer handling lives on the root, not the track: the thumb sits above the
	// track (absolute), so grabbing it must still start a drag.
	rsx! {
		span {
			class: cn!(SLIDER_ROOT, class),
			"data-slot": "slider",
			"data-orientation": ori,
			"data-disabled": disabled.then_some(true),
			onpointerdown: move |e: PointerEvent| async move {
				if disabled {
					return;
				}
				let Some(t) = track() else { return };
				let Ok(rect) = t.get_client_rect().await else { return };
				let measured = match orientation {
					SliderOrientation::Horizontal => Track { origin: rect.origin.x, size: rect.size.width },
					SliderOrientation::Vertical => Track { origin: rect.origin.y, size: rect.size.height },
				};
				drag.write().press(measured);
				// Pointer capture (the trick Radix uses): route every later move/up to
				// the track so a drag keeps tracking once the cursor leaves the thin
				// ~16px-tall hit area. Web-only; a no-op on other renderers.
				#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
				if let Some(el) = t.downcast::<web_sys::Element>() {
					// best-effort: a stale pointer_id just leaves the drag in its
					// pre-capture, cursor-must-stay-over mode; it never panics.
					let _ = el.set_pointer_capture(e.pointer_id());
				}
				state.set(slider.value_at(axis(&e), measured));
			},
			onpointermove: move |e: PointerEvent| {
				if let Some(measured) = drag.write().moved(disabled, !e.held_buttons().is_empty()) {
					state.set(slider.value_at(axis(&e), measured));
				}
			},
			onpointerup: move |_| drag.write().release(),
			// Without pointer capture (any non-web renderer) a release off the root
			// never reaches `onpointerup`; leaving ends the drag instead. Under
			// capture the pointer counts as over the track until release, so this
			// does not cut a web drag short.
			onpointerleave: move |_| drag.write().release(),
			onpointercancel: move |_| drag.write().release(),
			span {
				class: SLIDER_TRACK,
				"data-slot": "slider-track",
				"data-orientation": ori,
				onmounted: move |e: MountedEvent| track.set(Some(e.data())),
				span {
					class: SLIDER_RANGE,
					"data-slot": "slider-range",
					"data-orientation": ori,
					style: slider.range_style(current),
				}
			}
			span {
				class: SLIDER_THUMB,
				"data-slot": "slider-thumb",
				"data-orientation": ori,
				style: slider.thumb_style(current),
				role: "slider",
				// The name belongs on the focusable `role="slider"` node: on the
				// role-less root it is prohibited and never announced.
				"aria-label": aria_label,
				tabindex: if disabled { "-1" } else { "0" },
				"aria-valuenow": current,
				"aria-valuemin": min,
				"aria-valuemax": max,
				"aria-orientation": ori,
				"aria-disabled": disabled.then_some(true),
				onkeydown: on_key,
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::test_util::render;

	#[test]
	fn renders_slots_role_and_aria() {
		fn app() -> Element {
			rsx! {
				Slider { default_value: 25.0 }
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"slider\""), "{html}");
		assert!(html.contains("data-slot=\"slider-track\""), "{html}");
		assert!(html.contains("data-slot=\"slider-range\""), "{html}");
		assert!(html.contains("data-slot=\"slider-thumb\""), "{html}");
		assert!(html.contains("role=\"slider\""), "{html}");
		assert!(html.contains("aria-valuenow=25"), "{html}");
		assert!(html.contains("aria-valuemin=0"), "{html}");
		assert!(html.contains("aria-valuemax=100"), "{html}");
		assert!(html.contains("aria-orientation=\"horizontal\""), "{html}");
	}

	#[test]
	fn aria_label_names_the_thumb_not_the_root() {
		fn app() -> Element {
			rsx! {
				Slider { aria_label: "Volume", default_value: 50.0 }
			}
		}
		let html = render(app);
		let root_at = html.find("data-slot=\"slider\"").expect("root rendered");
		let root = &html[html[..root_at].rfind('<').expect("opening tag")..root_at + html[root_at..].find('>').expect("closing bracket")];
		assert!(!root.contains("aria-label"), "the role-less root must not carry the name: {root}");
		let thumb_at = html.find("data-slot=\"slider-thumb\"").expect("thumb rendered");
		let thumb = &html[thumb_at..thumb_at + html[thumb_at..].find('>').expect("closing bracket")];
		assert!(thumb.contains("role=\"slider\""), "{thumb}");
		assert!(thumb.contains("aria-label=\"Volume\""), "{thumb}");
	}

	#[test]
	fn range_and_thumb_reflect_percent() {
		fn app() -> Element {
			rsx! {
				Slider { default_value: 25.0, min: 0.0, max: 100.0 }
			}
		}
		let html = render(app);
		assert!(html.contains("width: 25%"), "{html}");
		assert!(html.contains("left: 25%"), "{html}");
	}

	#[test]
	fn vertical_uses_height_and_bottom() {
		fn app() -> Element {
			rsx! {
				Slider { default_value: 40.0, orientation: SliderOrientation::Vertical }
			}
		}
		let html = render(app);
		assert!(html.contains("data-orientation=\"vertical\""), "{html}");
		assert!(html.contains("height: 40%"), "{html}");
		assert!(html.contains("bottom: 40%"), "{html}");
	}

	#[test]
	fn controlled_value_clamps_to_range() {
		fn app() -> Element {
			rsx! {
				Slider { value: 150.0, min: 0.0, max: 100.0 }
			}
		}
		let html = render(app);
		assert!(html.contains("aria-valuenow=100"), "{html}");
	}

	#[test]
	fn class_override_merges() {
		fn app() -> Element {
			rsx! {
				Slider { default_value: 0.0, class: "w-40" }
			}
		}
		let html = render(app);
		assert!(html.contains("w-40"), "{html}");
		// the root's standalone `w-full` is dropped by the override; the track's
		// `data-[orientation=horizontal]:w-full` is a different utility and stays.
		assert!(!html.contains("flex w-full"), "override should drop root w-full: {html}");
	}
}
