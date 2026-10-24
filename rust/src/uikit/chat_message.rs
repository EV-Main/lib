use std::{cell::Cell, rc::Rc};

use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		CHAT_ATTACHMENT_BASE, CHAT_ATTACHMENT_BODY, CHAT_ATTACHMENT_ICON, CHAT_ATTACHMENT_META, CHAT_ATTACHMENT_NAME, CHAT_ATTACHMENT_PROGRESS, CHAT_ATTACHMENT_PROGRESS_FILL,
		CHAT_ATTACHMENT_ROW, CHAT_BUBBLE_BASE, CHAT_MARKER_DOT, CHAT_MARKER_RULE, CHAT_MARKER_SEPARATOR, CHAT_MARKER_SEPARATOR_LABEL, CHAT_MARKER_STATUS, CHAT_MARKER_TOOL,
		CHAT_MARKER_TOOL_BODY, CHAT_MARKER_TOOL_BORDER, CHAT_MARKER_TOOL_CHECK, CHAT_MARKER_TOOL_DETAIL, CHAT_MARKER_TOOL_ERROR_BORDER, CHAT_MARKER_TOOL_ICON, CHAT_MARKER_TOOL_META,
		CHAT_MARKER_TOOL_TITLE, CHAT_MESSAGE_AVATAR, CHAT_MESSAGE_AVATAR_TILE, CHAT_MESSAGE_BASE, CHAT_MESSAGE_BODY, CHAT_MESSAGE_BODY_ASSISTANT, CHAT_MESSAGE_BODY_USER,
		CHAT_MESSAGE_HEADER, CHAT_MESSAGE_NAME, CHAT_MESSAGE_TIME, CHAT_MESSAGE_USER, ChatBubbleVariant, ChatStatusState, ChatToolState, IconButton, Size,
	},
};

/// One block of message text. Mirrors the TS `ChatBubble`.
///
/// `streaming` marks a bubble that still receives text, for the caret and the
/// dimmed ink in theme.css. The TS bubble also fades string children in word
/// by word through `ChatStreamText`; this port has no `ChatStreamText`, so the
/// text shows at once. A bubble that has streamed keeps `data-caret`, so the
/// caret fades out by transition instead of vanishing.
#[component]
pub fn ChatBubble(#[props(default)] variant: ChatBubbleVariant, #[props(default)] streaming: bool, #[props(default)] class: String, children: Element) -> Element {
	// A plain cell, not a signal: it is written during render, and the render
	// that writes it already paints the result.
	let live = use_hook(|| Rc::new(Cell::new(false)));
	if streaming {
		live.set(true);
	}
	let cls = cn!(CHAT_BUBBLE_BASE, variant.as_class(), class);
	rsx! {
		div {
			class: cls,
			"data-slot": "chat-bubble",
			"data-variant": variant.as_ref(),
			"data-streaming": streaming.then_some(""),
			"data-caret": live.get().then_some(""),
			{children}
		}
	}
}

/// Who spoke a [`ChatMessage`]. Rust only: the TS `role` is an inline union.
#[derive(Clone, Copy, Debug, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatMessageRole {
	Assistant,
	User,
}

/// A turn: who spoke, when, then the bubbles and markers. Mirrors the TS
/// `ChatMessage`. theme.css fades it in and rises it once on mount
/// (`data-enter="rise"`).
///
/// `avatar` replaces the default sparkle tile of an assistant turn; a user turn
/// has none. `compact` drops the name row and the avatar, for a narrow dock.
#[component]
pub fn ChatMessage(
	role: ChatMessageRole,
	name: Option<Element>,
	time: Option<Element>,
	avatar: Option<Element>,
	#[props(default)] compact: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let user = role == ChatMessageRole::User;
	let cls = cn!(CHAT_MESSAGE_BASE, if user { CHAT_MESSAGE_USER } else { "" }, class);
	let body = cn!(CHAT_MESSAGE_BODY, if user { CHAT_MESSAGE_BODY_USER } else { CHAT_MESSAGE_BODY_ASSISTANT });
	let header = !compact && (name.is_some() || time.is_some());
	rsx! {
		div { class: cls, "data-slot": "chat-message", "data-role": role.as_ref(), "data-enter": "rise",
			if !user && !compact {
				div { class: CHAT_MESSAGE_AVATAR,
					if let Some(avatar) = avatar {
						{avatar}
					} else {
						span { class: CHAT_MESSAGE_AVATAR_TILE,
							Glyph { class: "size-4", path { d: SPARKLES } }
						}
					}
				}
			}
			div { class: body,
				if header {
					div { class: CHAT_MESSAGE_HEADER,
						if let Some(name) = name {
							span { class: CHAT_MESSAGE_NAME, {name} }
						}
						if let Some(time) = time {
							span { class: CHAT_MESSAGE_TIME, {time} }
						}
					}
				}
				{children}
			}
		}
	}
}

/// What a [`ChatMarker`] draws. Rust only: the TS marker is a union keyed on
/// `type`, and each kind carries its own fields here.
#[derive(Clone, PartialEq)]
pub enum ChatMarkerKind {
	/// Progress: "Generating response…", "Thought for 6s".
	Status(ChatStatusState),
	/// A tool call.
	Tool {
		state: ChatToolState,
		/// The tool's name.
		title: Element,
		detail: Option<Element>,
		/// The right-hand note: "Running…", "0.4s", "Failed · 1.2s".
		meta: Option<Element>,
	},
	/// A day break; the children are its label.
	Separator,
}

/// The quiet lines between messages: progress, a tool call, or a day break.
/// Mirrors the TS `ChatMarker`.
///
/// Only a running status is a live region, so a finished one stays silent. A
/// separator has no `separator` role, because that role hides its label from
/// assistive tech.
#[component]
pub fn ChatMarker(kind: ChatMarkerKind, #[props(default)] class: String, children: Element) -> Element {
	match kind {
		ChatMarkerKind::Separator => rsx! {
			div { class: cn!(CHAT_MARKER_SEPARATOR, class), "data-slot": "chat-marker", "data-type": "separator",
				span { class: CHAT_MARKER_RULE }
				span { class: CHAT_MARKER_SEPARATOR_LABEL, {children} }
				span { class: CHAT_MARKER_RULE }
			}
		},
		ChatMarkerKind::Status(state) => {
			let running = state == ChatStatusState::Running;
			rsx! {
				div {
					class: cn!(CHAT_MARKER_STATUS, class),
					"data-slot": "chat-marker",
					"data-type": "status",
					"data-state": state.as_ref(),
					role: running.then_some("status"),
					span { "aria-hidden": "true", class: cn!(CHAT_MARKER_DOT, state.as_class()) }
					{children}
				}
			}
		}
		ChatMarkerKind::Tool { state, title, detail, meta } => {
			let error = state == ChatToolState::Error;
			let cls = cn!(CHAT_MARKER_TOOL, if error { CHAT_MARKER_TOOL_ERROR_BORDER } else { CHAT_MARKER_TOOL_BORDER }, class);
			let meta_cls = cn!(CHAT_MARKER_TOOL_META, state.as_class());
			rsx! {
				div { class: cls, "data-slot": "chat-marker", "data-type": "tool", "data-state": state.as_ref(),
					Glyph { class: CHAT_MARKER_TOOL_ICON,
						if error {
							circle { cx: "12", cy: "12", r: "10" }
							path { d: "M12 8v4M12 16h.01" }
						} else {
							path { d: SPARKLES }
						}
					}
					div { class: CHAT_MARKER_TOOL_BODY,
						span { class: CHAT_MARKER_TOOL_TITLE, {title} }
						if let Some(detail) = detail {
							span { class: CHAT_MARKER_TOOL_DETAIL, {detail} }
						}
						{children}
					}
					if let Some(meta) = meta {
						span { class: meta_cls, {meta} }
					}
					if state == ChatToolState::Done {
						Glyph { class: CHAT_MARKER_TOOL_CHECK, path { d: "M20 6 9 17l-5-5" } }
					}
				}
			}
		}
	}
}

/// What a [`ChatAttachment`] holds; it picks the glyph. Rust only: the TS
/// `type` is an inline union.
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatAttachmentType {
	#[default]
	File,
	Image,
}

/// Whether a [`ChatAttachment`] is still uploading. Rust only, like
/// [`ChatAttachmentType`].
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatAttachmentState {
	#[default]
	Ready,
	Uploading,
}

/// A card for a file in a message or in the composer. Mirrors the TS
/// `ChatAttachment`.
///
/// `progress` runs from 0 to 100 and is clamped to that range. While uploading,
/// `meta` defaults to "Uploading… N%"; that default is English, as in TS, so a
/// translated UI passes its own `meta`. `on_remove` shows the remove button,
/// named `remove_label` or "Remove {name}".
#[component]
pub fn ChatAttachment(
	#[props(default)] r#type: ChatAttachmentType,
	#[props(default)] state: ChatAttachmentState,
	name: String,
	meta: Option<Element>,
	#[props(default)] progress: f64,
	on_remove: Option<EventHandler<()>>,
	remove_label: Option<String>,
	#[props(default)] class: String,
) -> Element {
	let pct = progress.round().clamp(0.0, 100.0) as u8;
	let uploading = state == ChatAttachmentState::Uploading;
	let meta = meta.or_else(|| uploading.then(|| rsx! { "Uploading… {pct}%" }));
	let remove_label = remove_label.unwrap_or_else(|| format!("Remove {name}"));
	rsx! {
		div {
			class: cn!(CHAT_ATTACHMENT_BASE, class),
			"data-slot": "chat-attachment",
			"data-type": r#type.as_ref(),
			"data-state": state.as_ref(),
			div { class: CHAT_ATTACHMENT_ROW,
				span { class: CHAT_ATTACHMENT_ICON,
					Glyph { class: "size-4",
						match r#type {
							ChatAttachmentType::Image => rsx! {
								rect { width: "18", height: "18", x: "3", y: "3", rx: "2" }
								circle { cx: "9", cy: "9", r: "2" }
								path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" }
							},
							ChatAttachmentType::File => rsx! {
								path { d: "M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" }
								path { d: "M14 2v4a2 2 0 0 0 2 2h4" }
							},
						}
					}
				}
				div { class: CHAT_ATTACHMENT_BODY,
					span { class: CHAT_ATTACHMENT_NAME, {name.clone()} }
					span { class: CHAT_ATTACHMENT_META, {meta} }
				}
				if let Some(remove) = on_remove {
					IconButton { aria_label: remove_label, size: Size::Sm, onclick: move |_| remove.call(()),
						Glyph { class: "size-4", path { d: "M18 6 6 18M6 6l12 12" } }
					}
				}
			}
			if uploading {
				div {
					class: CHAT_ATTACHMENT_PROGRESS,
					role: "progressbar",
					"aria-label": name,
					"aria-valuemin": "0",
					"aria-valuemax": "100",
					"aria-valuenow": "{pct}",
					div { class: CHAT_ATTACHMENT_PROGRESS_FILL, style: "width: {pct}%" }
				}
			}
		}
	}
}

const SPARKLES: &str = "M9.9 4.4 11 8l3.6 1.1L11 10.2 9.9 13.8 8.8 10.2 5.2 9.1 8.8 8ZM18 14l.7 2.3L21 17l-2.3.7L18 20l-.7-2.3L15 17l2.3-.7Z";

/// The TS `UiIcon` frame: a 24-unit lucide glyph drawn in the current ink.
/// Shared with the composer, which draws the same frame.
#[component]
pub(super) fn Glyph(class: &'static str, children: Element) -> Element {
	rsx! {
		svg {
			class,
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

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicUsize, Ordering};

	use super::*;
	use crate::uikit::test_util::{Harness, render};

	/// The whole opening tag that holds `marker`, whatever the attribute order.
	fn opening_tag<'a>(html: &'a str, marker: &str) -> &'a str {
		let at = html.find(marker).unwrap_or_else(|| panic!("{marker} rendered: {html}"));
		let start = html[..at].rfind('<').expect("tag start");
		let end = at + html[at..].find('>').expect("tag end");
		&html[start..end]
	}

	#[test]
	fn user_and_muted_bubbles_are_boxed_and_the_assistant_one_is_open() {
		fn app() -> Element {
			rsx! {
				ChatBubble { variant: ChatBubbleVariant::User, "вопрос" }
				ChatBubble { variant: ChatBubbleVariant::Muted, "заметка" }
				ChatBubble { "ответ" }
			}
		}
		let html = render(app);
		let user = opening_tag(&html, "data-variant=\"user\"");
		assert!(user.contains(" border ") && user.contains("bg-secondary"), "{user}");
		assert!(opening_tag(&html, "data-variant=\"muted\"").contains("bg-muted"), "{html}");
		let assistant = opening_tag(&html, "data-variant=\"assistant\"");
		assert!(!assistant.contains("border"), "{assistant}");
		assert!(assistant.contains("whitespace-pre-wrap"), "{assistant}");
	}

	#[test]
	fn a_streaming_bubble_is_marked_for_the_caret() {
		fn app() -> Element {
			rsx! {
				ChatBubble { streaming: true, "печатаю" }
				ChatBubble { "готово" }
			}
		}
		let html = render(app);
		let live = opening_tag(&html, "печатаю");
		assert!(live.contains("data-streaming") && live.contains("data-caret"), "{live}");
		let done = opening_tag(&html, "готово");
		assert!(!done.contains("data-streaming") && !done.contains("data-caret"), "{done}");
		assert!(html.contains(">печатаю</div>"), "the text is not split into words: {html}");
	}

	#[test]
	fn a_bubble_keeps_its_caret_slot_after_streaming_ends() {
		fn app() -> Element {
			let mut streaming = use_signal(|| true);
			rsx! {
				button { onclick: move |_| streaming.set(false) }
				ChatBubble { streaming: streaming(), "текст" }
			}
		}
		let mut h = Harness::new(app);
		assert!(opening_tag(&h.html(), "текст").contains("data-streaming"));
		let click = h.listening("click")[0];
		h.click(click);
		let tag = h.html();
		let tag = opening_tag(&tag, "текст");
		assert!(!tag.contains("data-streaming"), "{tag}");
		assert!(tag.contains("data-caret"), "{tag}");
	}

	#[test]
	fn a_turn_rises_in_and_shows_who_and_when() {
		fn app() -> Element {
			rsx! {
				ChatMessage { role: ChatMessageRole::Assistant, name: rsx! { "Ассистент" }, time: rsx! { "13:04" },
					ChatBubble { "Привет" }
				}
			}
		}
		let html = render(app);
		let root = opening_tag(&html, "data-slot=\"chat-message\"");
		assert!(root.contains("data-enter=\"rise\"") && root.contains("data-role=\"assistant\""), "{root}");
		assert!(html.contains("Ассистент</span>") && html.contains("13:04</span>"), "{html}");
		assert!(html.contains(CHAT_MESSAGE_AVATAR_TILE) && html.contains("<svg"), "the default sparkle tile: {html}");
	}

	#[test]
	fn the_users_turn_aligns_to_the_end_and_has_no_avatar() {
		fn app() -> Element {
			rsx! {
				ChatMessage { role: ChatMessageRole::User, name: rsx! { "Вы" },
					ChatBubble { variant: ChatBubbleVariant::User, "Здравствуйте" }
				}
			}
		}
		let html = render(app);
		assert!(opening_tag(&html, "data-slot=\"chat-message\"").contains("justify-end"), "{html}");
		assert!(!html.contains("<svg"), "{html}");
		assert!(html.contains("Вы</span>"), "{html}");
	}

	#[test]
	fn compact_drops_the_header_and_the_avatar() {
		fn app() -> Element {
			rsx! {
				ChatMessage { role: ChatMessageRole::Assistant, name: rsx! { "Ассистент" }, compact: true,
					ChatBubble { "Привет" }
				}
			}
		}
		let html = render(app);
		assert!(!html.contains("Ассистент"), "{html}");
		assert!(!html.contains("<svg"), "{html}");
		assert!(html.contains("Привет"), "{html}");
	}

	#[test]
	fn a_custom_avatar_replaces_the_sparkle_tile() {
		fn app() -> Element {
			rsx! {
				ChatMessage { role: ChatMessageRole::Assistant, avatar: rsx! { span { "data-own": "" } }, ChatBubble { "Привет" } }
			}
		}
		let html = render(app);
		assert!(html.contains("data-own"), "{html}");
		assert!(!html.contains(CHAT_MESSAGE_AVATAR_TILE), "{html}");
	}

	#[test]
	fn a_running_status_is_announced_and_a_finished_one_is_silent() {
		fn running() -> Element {
			rsx! { ChatMarker { kind: ChatMarkerKind::Status(ChatStatusState::Running), "Генерирую ответ…" } }
		}
		fn done() -> Element {
			rsx! { ChatMarker { kind: ChatMarkerKind::Status(ChatStatusState::Done), "Думал 6 с" } }
		}
		let html = render(running);
		let tag = opening_tag(&html, "data-slot=\"chat-marker\"");
		assert!(tag.contains("role=\"status\"") && tag.contains("data-state=\"running\""), "{tag}");
		assert!(html.contains("motion-safe:animate-pulse") && html.contains("Генерирую ответ…"), "{html}");
		let html = render(done);
		assert!(!html.contains("role=\"status\""), "{html}");
		assert!(html.contains("bg-ink-soft") && !html.contains("animate-pulse"), "{html}");
	}

	#[test]
	fn a_tool_call_shows_its_detail_and_meta_and_flags_a_failure() {
		fn done() -> Element {
			rsx! {
				ChatMarker {
					kind: ChatMarkerKind::Tool {
						state: ChatToolState::Done,
						title: rsx! { "get_positions" },
						detail: Some(rsx! { "account=main" }),
						meta: Some(rsx! { "0,4 с" }),
					},
				}
			}
		}
		fn failed() -> Element {
			rsx! {
				ChatMarker { kind: ChatMarkerKind::Tool { state: ChatToolState::Error, title: rsx! { "get_positions" }, detail: None, meta: Some(rsx! { "Ошибка · 1,2 с" }) } }
			}
		}
		let html = render(done);
		assert!(html.contains("get_positions</span>") && html.contains("account=main</span>"), "{html}");
		assert!(opening_tag(&html, "0,4 с").contains("text-ink-soft"), "{html}");
		assert!(html.contains("M20 6 9 17l-5-5"), "a finished call shows the check: {html}");
		let html = render(failed);
		let root = opening_tag(&html, "data-slot=\"chat-marker\"");
		assert!(root.contains("data-state=\"error\"") && root.contains("border-chip-error-border"), "{root}");
		assert!(opening_tag(&html, "Ошибка · 1,2 с").contains("text-accent-error-ink"), "{html}");
		assert!(!html.contains("M20 6 9 17l-5-5"), "{html}");
	}

	#[test]
	fn a_separator_shows_its_label_without_the_separator_role() {
		fn app() -> Element {
			rsx! { ChatMarker { kind: ChatMarkerKind::Separator, "Сегодня" } }
		}
		let html = render(app);
		assert!(html.contains("Сегодня</span>"), "{html}");
		assert!(html.contains("data-type=\"separator\""), "{html}");
		assert!(!html.contains("role="), "{html}");
	}

	#[test]
	fn an_attachment_shows_name_and_meta_and_removes_through_a_named_button() {
		static REMOVED: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				ChatAttachment { name: "отчёт.pdf", meta: rsx! { "PDF · 1,2 МБ" }, on_remove: |_| { REMOVED.fetch_add(1, Ordering::SeqCst); } }
			}
		}
		let mut h = Harness::new(app);
		let html = h.html();
		assert!(html.contains("PDF · 1,2 МБ</span>"), "{html}");
		assert!(html.contains("aria-label=\"Remove отчёт.pdf\""), "{html}");
		let clicks = h.listening("click");
		assert_eq!(clicks.len(), 1, "{html}");
		h.click(clicks[0]);
		assert_eq!(REMOVED.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn an_attachment_without_on_remove_has_no_button() {
		fn app() -> Element {
			rsx! { ChatAttachment { name: "a.pdf", remove_label: "Убрать" } }
		}
		let html = render(app);
		assert!(!html.contains("<button"), "{html}");
	}

	#[test]
	fn upload_progress_is_a_named_progressbar_and_the_default_meta() {
		fn app() -> Element {
			rsx! { ChatAttachment { r#type: ChatAttachmentType::Image, state: ChatAttachmentState::Uploading, name: "график.png", progress: 62.0 } }
		}
		let html = render(app);
		let bar = opening_tag(&html, "role=\"progressbar\"");
		assert!(bar.contains("aria-label=\"график.png\"") && bar.contains("aria-valuenow=\"62\""), "{bar}");
		assert!(html.contains("Uploading… 62%</span>"), "{html}");
		assert!(html.contains("width: 62%"), "{html}");
		assert!(html.contains("data-type=\"image\""), "{html}");
	}

	#[test]
	fn progress_is_clamped_to_the_range() {
		fn over() -> Element {
			rsx! { ChatAttachment { state: ChatAttachmentState::Uploading, name: "а", progress: 140.0 } }
		}
		fn under() -> Element {
			rsx! { ChatAttachment { state: ChatAttachmentState::Uploading, name: "а", progress: -5.0 } }
		}
		assert!(render(over).contains("aria-valuenow=\"100\""));
		assert!(render(under).contains("aria-valuenow=\"0\""));
	}

	#[test]
	fn a_ready_attachment_has_no_progressbar_and_an_empty_meta() {
		fn app() -> Element {
			rsx! { ChatAttachment { name: "а.pdf", progress: 50.0 } }
		}
		let html = render(app);
		assert!(!html.contains("progressbar"), "{html}");
		assert!(!html.contains("Uploading"), "{html}");
	}
}
