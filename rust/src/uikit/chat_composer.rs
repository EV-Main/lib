use std::rc::Rc;

use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		CHAT_COMPOSER_ATTACHMENTS, CHAT_COMPOSER_BASE, CHAT_COMPOSER_FIELD, CHAT_COMPOSER_HINT, CHAT_COMPOSER_IDLE, CHAT_COMPOSER_SPACER, CHAT_COMPOSER_STREAMING, CHAT_COMPOSER_TOOLBAR,
		DropdownMenu, DropdownMenuContent, DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuTrigger, IconButton, IconButtonVariant, MODEL_CHIP_BASE, Size, Textarea,
		chat_message::Glyph, primitives::use_controllable,
	},
};

/// One choice of a [`ModelChip`]. Mirrors the TS `ModelOption`.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelOption {
	pub id: String,
	pub label: String,
}

/// The small bordered button naming the model; it opens a radio menu of the
/// options. Mirrors the TS `ModelChip`.
///
/// The trigger is named "{aria_label}: {current label}", and falls back to the
/// raw `value` when no option has that id.
#[component]
pub fn ModelChip(
	models: Vec<ModelOption>,
	value: String,
	on_value_change: EventHandler<String>,
	#[props(default = String::from("Model"))] aria_label: String,
	#[props(default)] class: String,
) -> Element {
	let shown = models.iter().find(|m| m.id == value).map_or_else(|| value.clone(), |m| m.label.clone());
	rsx! {
		DropdownMenu {
			DropdownMenuTrigger { aria_label: format!("{aria_label}: {shown}"), class: cn!(MODEL_CHIP_BASE, class),
				{shown.clone()}
				Glyph { class: "size-4", path { d: "m6 9 6 6 6-6" } }
			}
			DropdownMenuContent {
				DropdownMenuRadioGroup { value: Some(value), on_value_change: move |id| on_value_change.call(id),
					for m in models {
						DropdownMenuRadioItem { key: "{m.id}", value: m.id, "{m.label}" }
					}
				}
			}
		}
	}
}

/// The composer's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChatComposerLabels {
	/// Names the message field; "Message".
	pub field: Option<String>,
	/// The send button; "Send".
	pub send: Option<String>,
	/// The stop button while streaming; "Stop".
	pub stop: Option<String>,
	/// The attach button; "Attach file".
	pub attach: Option<String>,
}

/// The message field with its toolbar. Mirrors the TS `ChatComposer`.
///
/// Enter or the send button calls `on_send` with the trimmed text, then clears
/// the field; Shift+Enter keeps the newline. An empty or whitespace-only
/// message is not sent, and `disabled` sends nothing. While `streaming`, the
/// send button becomes Stop, but Enter still queues a follow-up.
///
/// Enter that confirms an IME candidate never sends: see [`enter_sends`].
///
/// The field grows with its text through the `Textarea` base's
/// `field-sizing-content`, up to `max-h-40`, as in TS; there is no JS sizing.
#[component]
pub fn ChatComposer(
	value: Option<String>,
	#[props(default)] default_value: String,
	on_value_change: Option<EventHandler<String>>,
	on_send: EventHandler<String>,
	#[props(default)] streaming: bool,
	on_stop: Option<EventHandler<()>>,
	#[props(default = String::from("Ask the assistant…"))] placeholder: String,
	/// Shows the attach button.
	on_attach: Option<EventHandler<()>>,
	/// A row of `ChatAttachment`s above the field.
	attachments: Option<Element>,
	/// Model chips and other toolbar controls, after the attach button.
	tools: Option<Element>,
	/// A caption in the toolbar: "Enter to send · Shift+Enter for a new line".
	hint: Option<Element>,
	#[props(default)] disabled: bool,
	#[props(default)] labels: ChatComposerLabels,
	#[props(default)] class: String,
) -> Element {
	let text = use_controllable(value, default_value, on_value_change);
	let mut field = use_signal(|| None::<Rc<MountedData>>);
	let empty = text.get().trim().is_empty();
	// Reads the text at call time, so an Enter right after an input sends what
	// that input left, not the value of the last render.
	let send = move || {
		let current = text.get();
		let trimmed = current.trim();
		if trimmed.is_empty() || disabled {
			return;
		}
		on_send.call(trimmed.to_string());
		text.set(String::new());
	};
	let cls = cn!(CHAT_COMPOSER_BASE, if streaming { CHAT_COMPOSER_STREAMING } else { CHAT_COMPOSER_IDLE }, class);
	rsx! {
		div { class: cls, "data-slot": "chat-composer", "data-state": if streaming { "streaming" } else { "default" },
			if let Some(attachments) = attachments {
				div { class: CHAT_COMPOSER_ATTACHMENTS, {attachments} }
			}
			Textarea {
				rows: 1,
				value: text.get(),
				disabled,
				placeholder,
				aria_label: labels.field.unwrap_or_else(|| "Message".into()),
				class: CHAT_COMPOSER_FIELD,
				oninput: move |e: FormEvent| text.set(e.value()),
				onkeydown: move |e: KeyboardEvent| {
					if enter_sends(&e.key(), e.modifiers().shift(), e.is_composing(), legacy_key_code(&e)) {
						e.prevent_default();
						send();
					}
				},
				onmounted: move |e: MountedEvent| field.set(Some(e.data())),
			}
			div { class: CHAT_COMPOSER_TOOLBAR,
				if let Some(attach) = on_attach {
					IconButton { aria_label: labels.attach.unwrap_or_else(|| "Attach file".into()), size: Size::Sm, disabled, onclick: move |_| attach.call(()),
						Glyph { class: "size-4", path { d: "M5 12h14M12 5v14" } }
					}
				}
				{tools}
				span { class: CHAT_COMPOSER_SPACER }
				if let Some(hint) = hint {
					span { class: CHAT_COMPOSER_HINT, {hint} }
				}
				if streaming {
					IconButton {
						variant: IconButtonVariant::Secondary,
						aria_label: labels.stop.unwrap_or_else(|| "Stop".into()),
						onclick: move |_| {
							if let Some(stop) = on_stop {
								stop.call(());
							}
							// Stop turns into Send, so focus would stay on a swapped-out button.
							if let Some(el) = field.peek().clone() {
								// Best-effort: a detached field just leaves focus where it was.
								spawn(async move {
									let _ = el.set_focus(true).await;
								});
							}
						},
						Glyph { class: "size-4", rect { width: "12", height: "12", x: "6", y: "6", rx: "2" } }
					}
				} else {
					IconButton {
						variant: IconButtonVariant::Secondary,
						aria_label: labels.send.unwrap_or_else(|| "Send".into()),
						disabled: disabled || empty,
						onclick: move |_| send(),
						Glyph { class: "size-4", path { d: "m5 12 7-7 7 7M12 19V5" } }
					}
				}
			}
		}
	}
}

/// The `keyCode` every browser reports for a key that an IME takes.
const IME_KEY_CODE: u32 = 229;

/// Whether a keydown sends the message: a plain Enter that no IME composition
/// owns. The Enter of an IME composition confirms the candidate. Safari reports
/// that Enter after `compositionend`, with `isComposing` already false but
/// `keyCode` 229, so both are checked, the same two checks as the TS composer.
fn enter_sends(key: &Key, shift: bool, composing: bool, key_code: Option<u32>) -> bool {
	*key == Key::Enter && !shift && !composing && key_code != Some(IME_KEY_CODE)
}

/// The legacy `keyCode`, which only the web renderer carries; `KeyboardData`
/// itself does not expose it.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn legacy_key_code(e: &KeyboardData) -> Option<u32> {
	e.downcast::<web_sys::KeyboardEvent>().map(web_sys::KeyboardEvent::key_code)
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
fn legacy_key_code(_: &KeyboardData) -> Option<u32> {
	None
}

#[cfg(test)]
mod tests {
	use std::{
		cell::RefCell,
		sync::atomic::{AtomicUsize, Ordering},
	};

	use dioxus::html::{
		SerializedKeyboardData,
		input_data::keyboard_types::{Code, Location, Modifiers},
	};

	use super::*;
	use crate::uikit::test_util::{Harness, render};

	fn enter(modifiers: Modifiers, composing: bool) -> SerializedKeyboardData {
		SerializedKeyboardData::new(Key::Enter, Code::Enter, Location::Standard, false, modifiers, composing)
	}

	/// The textarea: the only element that listens for `input`.
	fn field(h: &Harness) -> dioxus::dioxus_core::ElementId {
		h.listening("input")[0]
	}

	/// The whole opening tag that holds `marker`, whatever the attribute order.
	fn opening_tag<'a>(html: &'a str, marker: &str) -> &'a str {
		let at = html.find(marker).unwrap_or_else(|| panic!("{marker} rendered: {html}"));
		let start = html[..at].rfind('<').expect("tag start");
		let end = at + html[at..].find('>').expect("tag end");
		&html[start..end]
	}

	thread_local! {
		static SENT: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
	}
	fn sent() -> Vec<String> {
		SENT.with_borrow(Clone::clone)
	}
	fn record(text: String) {
		SENT.with_borrow_mut(|s| s.push(text));
	}

	#[test]
	fn enter_sends_the_trimmed_text_and_clears_the_field() {
		fn app() -> Element {
			rsx! { ChatComposer { on_send: record } }
		}
		let mut h = Harness::new(app);
		let id = field(&h);
		h.input(id, "  привет, мир  ");
		assert!(h.html().contains("привет, мир"), "the input reached the field: {}", h.html());
		let default_kept = h.keyboard(id, enter(Modifiers::empty(), false));
		assert_eq!(sent(), ["привет, мир"]);
		assert!(!default_kept, "a sending Enter must not also insert a newline");
		assert!(!h.html().contains("привет"), "{}", h.html());
	}

	#[test]
	fn shift_enter_keeps_the_newline() {
		fn app() -> Element {
			rsx! { ChatComposer { on_send: record } }
		}
		let mut h = Harness::new(app);
		let id = field(&h);
		h.input(id, "первая строка");
		let default_kept = h.keyboard(id, enter(Modifiers::SHIFT, false));
		assert!(sent().is_empty(), "{:?}", sent());
		assert!(default_kept, "Shift+Enter must reach the textarea");
		assert!(h.html().contains("первая строка"), "{}", h.html());
	}

	#[test]
	fn enter_during_an_ime_composition_confirms_the_candidate_and_does_not_send() {
		fn app() -> Element {
			rsx! { ChatComposer { on_send: record } }
		}
		let mut h = Harness::new(app);
		let id = field(&h);
		h.input(id, "かな");
		let default_kept = h.keyboard(id, enter(Modifiers::empty(), true));
		assert!(sent().is_empty(), "{:?}", sent());
		assert!(default_kept, "the IME owns this Enter");
		assert!(h.html().contains("かな"), "the text stays: {}", h.html());
		// The same text sends once the composition is over, so the guard, not
		// the text, stopped the first Enter.
		h.keyboard(id, enter(Modifiers::empty(), false));
		assert_eq!(sent(), ["かな"]);
	}

	#[test]
	fn the_safari_enter_with_key_code_229_does_not_send() {
		// `SerializedKeyboardData` derives its keyCode from the key, so 229 only
		// reaches the guard through a browser; the decision itself is pure.
		assert!(!enter_sends(&Key::Enter, false, false, Some(IME_KEY_CODE)));
		assert!(enter_sends(&Key::Enter, false, false, Some(13)));
		assert!(enter_sends(&Key::Enter, false, false, None));
		assert!(!enter_sends(&Key::Enter, false, true, None));
		assert!(!enter_sends(&Key::Enter, true, false, None));
		assert!(!enter_sends(&Key::Process, false, false, Some(IME_KEY_CODE)));
		assert!(!enter_sends(&Key::Character("ы".into()), false, false, None));
	}

	#[test]
	fn names_the_field() {
		fn plain() -> Element {
			rsx! { ChatComposer { on_send: |_| {} } }
		}
		fn translated() -> Element {
			rsx! { ChatComposer { on_send: |_| {}, labels: ChatComposerLabels { field: Some("Сообщение".into()), send: Some("Отправить".into()), ..Default::default() } } }
		}
		let html = render(plain);
		let area = opening_tag(&html, "<textarea");
		assert!(area.contains("aria-label=\"Message\"") && area.contains("rows=\"1\""), "{area}");
		assert!(area.contains("placeholder=\"Ask the assistant…\""), "{area}");
		let html = render(translated);
		assert!(opening_tag(&html, "<textarea").contains("aria-label=\"Сообщение\""), "{html}");
		assert!(html.contains("aria-label=\"Отправить\""), "{html}");
	}

	#[test]
	fn the_field_reuses_the_textarea_without_its_box() {
		fn app() -> Element {
			rsx! { ChatComposer { on_send: |_| {} } }
		}
		let html = render(app);
		let area = opening_tag(&html, "<textarea");
		assert!(area.contains("data-slot=\"textarea\"") && area.contains("field-sizing-content"), "{area}");
		assert!(area.contains("min-h-6") && area.contains("max-h-40"), "{area}");
		assert!(!area.contains("min-h-16") && !area.contains("bg-muted"), "the composer's classes win: {area}");
	}

	#[test]
	fn an_empty_message_is_not_sent_and_the_send_button_is_disabled() {
		fn app() -> Element {
			rsx! { ChatComposer { on_send: record } }
		}
		let mut h = Harness::new(app);
		let id = field(&h);
		h.keyboard(id, enter(Modifiers::empty(), false));
		h.input(id, "   \n ");
		h.keyboard(id, enter(Modifiers::empty(), false));
		assert!(sent().is_empty(), "{:?}", sent());
		let html = h.html();
		assert!(opening_tag(&html, "aria-label=\"Send\"").contains(" disabled=true"), "{html}");
	}

	#[test]
	fn the_send_button_sends_once_there_is_text() {
		fn app() -> Element {
			rsx! { ChatComposer { on_send: record, default_value: "привет" } }
		}
		let mut h = Harness::new(app);
		let html = h.html();
		assert!(!opening_tag(&html, "aria-label=\"Send\"").contains(" disabled=true"), "{html}");
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 1, "only the send button: {html}");
		h.click(clicks[0]);
		assert_eq!(sent(), ["привет"]);
	}

	#[test]
	fn streaming_swaps_send_for_stop_and_enter_still_queues_a_follow_up() {
		static STOPS: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				ChatComposer { on_send: record, on_stop: |_| { STOPS.fetch_add(1, Ordering::SeqCst); }, streaming: true, default_value: "дальше" }
			}
		}
		let mut h = Harness::new(app);
		let html = h.html();
		assert!(!html.contains("aria-label=\"Send\""), "{html}");
		assert!(opening_tag(&html, "data-slot=\"chat-composer\"").contains("data-state=\"streaming\""), "{html}");
		assert!(opening_tag(&html, "data-slot=\"chat-composer\"").contains("border-border\""), "{html}");
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 1, "only the stop button: {html}");
		h.click(clicks[0]);
		assert_eq!(STOPS.load(Ordering::SeqCst), 1);
		let id = field(&h);
		h.keyboard(id, enter(Modifiers::empty(), false));
		assert_eq!(sent(), ["дальше"]);
	}

	#[test]
	fn shows_the_attach_button_the_attachments_and_the_tools() {
		static ATTACHES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				ChatComposer {
					on_send: |_| {},
					on_attach: |_| { ATTACHES.fetch_add(1, Ordering::SeqCst); },
					attachments: rsx! { span { "файл.pdf" } },
					tools: rsx! { span { "Быстрая" } },
					hint: rsx! { "Enter — отправить" },
				}
			}
		}
		let mut h = Harness::new(app);
		let html = h.html();
		assert!(html.contains("aria-label=\"Attach file\""), "{html}");
		assert!(html.contains("файл.pdf") && html.contains("Быстрая") && html.contains("Enter — отправить"), "{html}");
		// Every button: the send button with no text does nothing, so only the
		// attach button counts.
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 2, "attach and send: {html}");
		for id in clicks {
			h.click(id);
		}
		assert_eq!(ATTACHES.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn does_nothing_when_disabled() {
		fn app() -> Element {
			rsx! { ChatComposer { on_send: record, disabled: true, default_value: "х", on_attach: |_| {} } }
		}
		let mut h = Harness::new(app);
		let id = field(&h);
		h.keyboard(id, enter(Modifiers::empty(), false));
		assert!(sent().is_empty(), "{:?}", sent());
		let html = h.html();
		assert!(opening_tag(&html, "<textarea").contains(" disabled=true"), "{html}");
		assert!(opening_tag(&html, "aria-label=\"Attach file\"").contains(" disabled=true"), "{html}");
	}

	#[test]
	fn a_controlled_composer_reports_edits_and_the_clear() {
		thread_local! {
			static CHANGES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
		}
		fn app() -> Element {
			let mut value = use_signal(|| "ответ".to_string());
			rsx! {
				ChatComposer {
					value: value(),
					on_value_change: move |v: String| {
						CHANGES.with_borrow_mut(|c| c.push(v.clone()));
						value.set(v);
					},
					on_send: record,
				}
			}
		}
		let mut h = Harness::new(app);
		let id = field(&h);
		h.input(id, "ответ готов");
		h.keyboard(id, enter(Modifiers::empty(), false));
		assert_eq!(sent(), ["ответ готов"]);
		assert_eq!(CHANGES.with_borrow(Clone::clone), ["ответ готов", ""]);
	}

	fn models() -> Vec<ModelOption> {
		vec![
			ModelOption {
				id: "fast".into(),
				label: "Быстрая".into(),
			},
			ModelOption {
				id: "deep".into(),
				label: "Рассуждение".into(),
			},
		]
	}

	/// The opening tag of the menu item that shows `label`. The trigger shows
	/// the current label too, so the last occurrence is the item's.
	fn item_tag<'a>(html: &'a str, label: &str) -> &'a str {
		let at = html.rfind(label).unwrap_or_else(|| panic!("{label} rendered: {html}"));
		let role = html[..at].rfind("role=\"menuitemradio\"").expect("a menu item");
		let start = html[..role].rfind('<').expect("tag start");
		let end = role + html[role..].find('>').expect("tag end");
		&html[start..end]
	}

	#[test]
	fn the_model_chip_names_the_current_model_and_picks_another() {
		fn app() -> Element {
			rsx! { ModelChip { models: models(), value: "fast", on_value_change: record } }
		}
		let mut h = Harness::new(app);
		let html = h.html();
		assert!(opening_tag(&html, "aria-haspopup=\"menu\"").contains("aria-label=\"Model: Быстрая\""), "{html}");
		h.click(h.listening("click")[0]);
		let html = h.html();
		assert!(item_tag(&html, "Быстрая").contains("aria-checked=\"true\""), "{html}");
		assert!(item_tag(&html, "Рассуждение").contains("aria-checked=\"false\""), "{html}");
		// The menu items are the last click listeners created, in option order.
		let deep = *h.listening("click").last().expect("the menu is open");
		h.click(deep);
		assert_eq!(sent(), ["deep"]);
	}

	#[test]
	fn the_model_chip_falls_back_to_the_raw_value_and_takes_a_name() {
		fn app() -> Element {
			rsx! { ModelChip { models: models(), value: "custom-7", aria_label: "Модель", on_value_change: |_| {} } }
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Модель: custom-7\""), "{html}");
		assert!(html.contains(MODEL_CHIP_BASE), "{html}");
	}
}
