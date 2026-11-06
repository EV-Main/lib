use leptos::prelude::*;

pub use ev_lib_classes::KeyValueLabels;
use crate::{
	cn,
	ui::uikit::leptos::{KEY_VALUE_BASE, KEY_VALUE_COPY, KEY_VALUE_LABEL, KEY_VALUE_ROW, KEY_VALUE_STATUS, KEY_VALUE_VALUE, icon_button::Glyph},
};

/// A [`KeyValue`] label: text, or any view. Text is what the copy button's
/// default name ("Copy {label}") is built from — the TS `typeof label ===
/// "string"`, which a Leptos view cannot answer once built.
#[derive(Clone)]
pub enum KeyValueLabel {
	Text(String),
	View(ViewFn),
}
impl From<&str> for KeyValueLabel {
	fn from(text: &str) -> Self {
		Self::Text(text.to_owned())
	}
}
impl From<String> for KeyValueLabel {
	fn from(text: String) -> Self {
		Self::Text(text)
	}
}
impl From<ViewFn> for KeyValueLabel {
	fn from(view: ViewFn) -> Self {
		Self::View(view)
	}
}

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
	#[prop(into)] label: KeyValueLabel,
	#[prop(into)] value: ViewFn,
	/// Adds a copy button for this text.
	#[prop(optional, into)]
	copy_text: Option<String>,
	/// Copies in place of the kit, for a renderer or a clipboard of the
	/// caller's own. The kit announces the copy as soon as it returns.
	#[prop(optional)]
	copy: Option<Callback<String>>,
	#[prop(optional)] labels: KeyValueLabels,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let copied = RwSignal::new(false);
	// Counts copies, so a timer clears only the announcement of the copy that
	// started it, as the TS `clearTimeout` does.
	let generation = RwSignal::new(0u32);
	let copy_label = labels.copy.clone().unwrap_or_else(|| match &label {
		KeyValueLabel::Text(text) => format!("Copy {text}"),
		KeyValueLabel::View(_) => String::from("Copy"),
	});
	let copied_label = labels.copied;
	let label = match label {
		KeyValueLabel::Text(text) => text.into_any(),
		KeyValueLabel::View(view) => view.run(),
	};
	let button = copy_text.map(|text| {
		let on_press = move |_| {
			let done = move |ok: bool| {
				if ok {
					announce(copied, generation);
				}
			};
			match copy {
				Some(handler) => {
					handler.run(text.clone());
					done(true);
				}
				None => kit_copy(&text, done),
			}
		};
		view! {
			<button
				type="button"
				class=KEY_VALUE_COPY
				data-slot="key-value-copy"
				aria-label=copy_label
				on:click=on_press
			>
				{move || {
					if copied.get() {
						view! {
							<Glyph class="size-4">
								<path d="M20 6 9 17l-5-5" />
							</Glyph>
						}
							.into_any()
					} else {
						view! {
							<Glyph class="size-4">
								<rect width="14" height="14" x="8" y="8" rx="2" />
								<path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" />
							</Glyph>
						}
							.into_any()
					}
				}}
			</button>
		}
	});
	let cls = cn!(KEY_VALUE_BASE, class);
	view! {
		<div class=cls data-slot="key-value">
			<span class=KEY_VALUE_LABEL data-slot="key-value-label">
				{label}
			</span>
			<div class=KEY_VALUE_ROW>
				<span class=KEY_VALUE_VALUE data-slot="key-value-value">
					{value.run()}
				</span>
				{button}
			</div>
			<span role="status" aria-live="polite" class=KEY_VALUE_STATUS>
				{move || copied.get().then(|| copied_label.clone())}
			</span>
		</div>
	}
}

fn announce(copied: RwSignal<bool>, generation: RwSignal<u32>) {
	// `try_`: under `wasm` this also runs from a promise or a timer, which can
	// fire after the component is gone and its signals with it.
	copied.try_set(true);
	let Some(started) = generation.try_update(|g| {
		*g = g.wrapping_add(1);
		*g
	}) else {
		return;
	};
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	set_timeout(
		move || {
			if generation.try_get_untracked() == Some(started) {
				copied.try_set(false);
			}
		},
		std::time::Duration::from_millis(crate::ui::uikit::host::COPIED_MS as u64),
	);
	// No timer outside `wasm`, so nothing waits on this copy's number.
	#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
	let _ = started;
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn kit_copy(text: &str, done: impl FnOnce(bool) + 'static) {
	crate::ui::uikit::host::write_clipboard(text, done);
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
fn kit_copy(_: &str, _: impl FnOnce(bool) + 'static) {}
