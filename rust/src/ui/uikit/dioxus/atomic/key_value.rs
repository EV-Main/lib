use dioxus::{
	dioxus_core::{DynamicNode, TemplateNode},
	prelude::*,
};
pub use ev_lib_classes::KeyValueLabels;

use crate::{
	cn,
	ui::uikit::dioxus::{KEY_VALUE_BASE, KEY_VALUE_COPY, KEY_VALUE_LABEL, KEY_VALUE_ROW, KEY_VALUE_STATUS, KEY_VALUE_VALUE},
};

/// A small label over its value, with an optional copy button. Mirrors the TS
/// `KeyValue`.
///
/// `copy_text` adds the button. A press hands the text to `copy` when it is
/// set. Without `copy`, the `wasm` feature writes it to the clipboard through
/// `write_clipboard`; any other build has no clipboard to reach, so the
/// button does nothing there. The check mark and the "Copied" announcement
/// clear after two seconds only under `wasm`, which has the timer for it.
#[component]
pub fn KeyValue(
	label: Element,
	value: Element,
	/// Adds a copy button for this text.
	copy_text: Option<String>,
	/// Copies in place of the kit, for a renderer or a clipboard of the
	/// caller's own. The kit announces the copy as soon as it returns.
	copy: Option<EventHandler<String>>,
	#[props(default)] labels: KeyValueLabels,
	#[props(default)] class: String,
) -> Element {
	let copied = use_signal(|| false);
	// Counts copies, so a timer clears only the announcement of the copy that
	// started it, as the TS `clearTimeout` does.
	let generation = use_signal(|| 0u32);
	let copy_label = labels.copy.clone().unwrap_or_else(|| match plain_text_of(&label) {
		Some(text) => format!("Copy {text}"),
		None => String::from("Copy"),
	});
	let copied_label = labels.copied;
	let has_copy = copy_text.is_some();
	let on_press = move |_| {
		let Some(text) = copy_text.clone() else { return };
		let done = move |ok: bool| {
			if ok {
				announce(copied, generation);
			}
		};
		match copy {
			Some(handler) => {
				handler.call(text);
				done(true);
			}
			None => kit_copy(&text, done),
		}
	};
	let cls = cn!(KEY_VALUE_BASE, class);
	rsx! {
		div { class: cls, "data-slot": "key-value",
			span { class: KEY_VALUE_LABEL, "data-slot": "key-value-label", {label} }
			div { class: KEY_VALUE_ROW,
				span { class: KEY_VALUE_VALUE, "data-slot": "key-value-value", {value} }
				if has_copy {
					button {
						r#type: "button",
						class: KEY_VALUE_COPY,
						"data-slot": "key-value-copy",
						"aria-label": copy_label,
						onclick: on_press,
						if copied() {
							CheckIcon {}
						} else {
							CopyIcon {}
						}
					}
				}
			}
			span { role: "status", "aria-live": "polite", class: KEY_VALUE_STATUS,
				if copied() {
					{copied_label}
				}
			}
		}
	}
}

fn announce(mut copied: Signal<bool>, mut generation: Signal<u32>) {
	// `try_write`: under `wasm` this also runs from a promise or a timer, which
	// can fire after the component is gone and its signals with it.
	if let Ok(mut c) = copied.try_write() {
		*c = true;
	}
	let started = {
		let Ok(mut g) = generation.try_write() else { return };
		*g = g.wrapping_add(1);
		*g
	};
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	clear_later(copied, generation, started);
	// No timer outside `wasm`, so nothing waits on this copy's number.
	#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
	let _ = started;
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn clear_later(mut copied: Signal<bool>, generation: Signal<u32>, started: u32) {
	use web_sys::{js_sys::Function, wasm_bindgen::JsCast};

	let Some(window) = web_sys::window() else { return };
	let clear = web_sys::wasm_bindgen::closure::Closure::once_into_js(move || {
		if generation.try_read().is_ok_and(|g| *g == started)
			&& let Ok(mut c) = copied.try_write()
		{
			*c = false;
		}
	});
	// A failed `setTimeout` only leaves the check mark up; nothing to recover.
	let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(clear.unchecked_ref::<Function>(), ev_lib_classes::KEY_VALUE_COPIED_MS);
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn kit_copy(text: &str, done: impl FnOnce(bool) + 'static) {
	crate::ui::uikit::host::write_clipboard(text, done);
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
fn kit_copy(_: &str, _: impl FnOnce(bool) + 'static) {}

/// The TS `UiIcon` frame: its default `size-4` and the 1.75 stroke.
#[component]
fn Glyph(children: Element) -> Element {
	rsx! {
		svg {
			class: "size-4",
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.75",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			"aria-hidden": "true",
			{children}
		}
	}
}

#[component]
fn CopyIcon() -> Element {
	rsx! {
		Glyph {
			rect { width: "14", height: "14", x: "8", y: "8", rx: "2" }
			path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }
		}
	}
}

#[component]
fn CheckIcon() -> Element {
	rsx! {
		Glyph {
			path { d: "M20 6 9 17l-5-5" }
		}
	}
}

/// The text of `children` when they are nothing but text — the Rust side of
/// the TS `typeof node === "string"`, for a label a name is built from. Any
/// element or component makes it `None`.
fn plain_text_of(children: &Element) -> Option<String> {
	let node = children.as_ref().ok()?;
	let mut out = String::new();
	plain_vnode_text(node, &mut out).then_some(out)
}
fn plain_vnode_text(node: &VNode, out: &mut String) -> bool {
	node.template.roots.iter().all(|root| match root {
		TemplateNode::Element { .. } => false,
		TemplateNode::Text { text } => {
			out.push_str(text);
			true
		}
		TemplateNode::Dynamic { id } => match node.dynamic_nodes.get(*id) {
			Some(DynamicNode::Text(text)) => {
				out.push_str(&text.value);
				true
			}
			Some(DynamicNode::Fragment(nodes)) => nodes.iter().all(|n| plain_vnode_text(n, out)),
			Some(DynamicNode::Placeholder(_)) => true,
			Some(DynamicNode::Component(_)) | None => false,
		},
	})
}

#[cfg(test)]
mod tests {
	use std::sync::Mutex;

	use super::*;
	use crate::ui::uikit::dioxus::test_util::{render, render_after_click};

	#[test]
	fn shows_the_label_above_the_value_and_no_button_without_copy_text() {
		fn app() -> Element {
			rsx! {
				KeyValue { label: rsx! { "Номер счёта" }, value: rsx! { "acc_42" } }
			}
		}
		let html = render(app);
		let label = html.find("Номер счёта").unwrap_or_else(|| panic!("{html}"));
		let value = html.find("acc_42").unwrap_or_else(|| panic!("{html}"));
		assert!(label < value, "the label comes first: {html}");
		assert!(!html.contains("<button"), "{html}");
	}

	#[test]
	fn copies_through_the_callback_and_announces_it_politely() {
		static COPIED: Mutex<Vec<String>> = Mutex::new(Vec::new());
		fn app() -> Element {
			rsx! {
				KeyValue {
					label: rsx! { "Счёт" },
					value: rsx! { "счёт_42" },
					copy_text: "счёт_42",
					copy: |text: String| COPIED.lock().unwrap_or_else(|e| e.into_inner()).push(text),
				}
			}
		}
		let before = render(app);
		assert!(
			before.contains("<span role=\"status\" aria-live=\"polite\" class=\"sr-only\"></span>"),
			"empty before a copy: {before}"
		);
		assert!(before.contains("M4 16c-1.1"), "the copy glyph: {before}");

		let html = render_after_click(app);
		assert_eq!(*COPIED.lock().unwrap_or_else(|e| e.into_inner()), ["счёт_42"]);
		assert!(html.contains(">Copied</span>"), "{html}");
		assert!(html.contains("M20 6 9 17l-5-5"), "the check mark replaces the copy glyph: {html}");
	}

	#[test]
	fn takes_a_translated_announcement() {
		fn app() -> Element {
			rsx! {
				KeyValue {
					label: rsx! { "Счёт" },
					value: rsx! { "x" },
					copy_text: "x",
					copy: |_| {},
					labels: KeyValueLabels { copied: "Скопировано".into(), ..Default::default() },
				}
			}
		}
		let html = render_after_click(app);
		assert!(html.contains(">Скопировано</span>"), "{html}");
		assert!(!html.contains("Copied"), "{html}");
	}

	#[test]
	fn names_the_copy_button_after_a_text_label_and_falls_back_to_copy() {
		fn app() -> Element {
			let name = "Почта";
			rsx! {
				KeyValue { label: rsx! { "Электронная почта" }, value: rsx! { "a" }, copy_text: "a" }
				KeyValue { label: rsx! { {name} }, value: rsx! { "c" }, copy_text: "c" }
				KeyValue { label: rsx! { b { "Узел" } }, value: rsx! { "b" }, copy_text: "b" }
				KeyValue { label: rsx! { "Узел" }, value: rsx! { "d" }, copy_text: "d", labels: KeyValueLabels { copy: Some("Скопировать узел".into()), ..Default::default() } }
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Copy Электронная почта\""), "{html}");
		assert!(html.contains("aria-label=\"Copy Почта\""), "a dynamic text label still names it: {html}");
		assert!(html.contains("aria-label=\"Copy\""), "a label that is not text: {html}");
		assert!(html.contains("aria-label=\"Скопировать узел\""), "{html}");
	}

	#[test]
	fn without_wasm_or_a_callback_the_button_does_not_announce() {
		fn app() -> Element {
			rsx! {
				KeyValue { label: rsx! { "Счёт" }, value: rsx! { "x" }, copy_text: "x" }
			}
		}
		let html = render_after_click(app);
		assert!(!html.contains("Copied"), "nothing was copied, so nothing is announced: {html}");
	}
}
