pub use ev_lib_classes::{ChatAttachmentState, ChatAttachmentType, ChatMessageRole};
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{
		CHAT_ATTACHMENT_BASE, CHAT_ATTACHMENT_BODY, CHAT_ATTACHMENT_ICON, CHAT_ATTACHMENT_META, CHAT_ATTACHMENT_NAME, CHAT_ATTACHMENT_PROGRESS, CHAT_ATTACHMENT_PROGRESS_FILL,
		CHAT_ATTACHMENT_ROW, CHAT_BUBBLE_BASE, CHAT_MARKER_DOT, CHAT_MARKER_RULE, CHAT_MARKER_SEPARATOR, CHAT_MARKER_SEPARATOR_LABEL, CHAT_MARKER_STATUS, CHAT_MARKER_TOOL,
		CHAT_MARKER_TOOL_BODY, CHAT_MARKER_TOOL_BORDER, CHAT_MARKER_TOOL_CHECK, CHAT_MARKER_TOOL_DETAIL, CHAT_MARKER_TOOL_ERROR_BORDER, CHAT_MARKER_TOOL_ICON, CHAT_MARKER_TOOL_META,
		CHAT_MARKER_TOOL_TITLE, CHAT_MESSAGE_AVATAR, CHAT_MESSAGE_AVATAR_TILE, CHAT_MESSAGE_BASE, CHAT_MESSAGE_BODY, CHAT_MESSAGE_BODY_ASSISTANT, CHAT_MESSAGE_BODY_USER,
		CHAT_MESSAGE_HEADER, CHAT_MESSAGE_NAME, CHAT_MESSAGE_TIME, CHAT_MESSAGE_USER, ChatBubbleVariant, ChatStatusState, ChatToolState, IconButton, Size, icon_button::Glyph,
	},
};

const SPARKLES: &str = "M9.9 4.4 11 8l3.6 1.1L11 10.2 9.9 13.8 8.8 10.2 5.2 9.1 8.8 8ZM18 14l.7 2.3L21 17l-2.3.7L18 20l-.7-2.3L15 17l2.3-.7Z";

/// One block of message text. Mirrors the TS `ChatBubble`.
///
/// `streaming` marks a bubble that still receives text, for the caret and the
/// dimmed ink in theme.css. The TS bubble also fades string children in word
/// by word through `ChatStreamText`; this port has no `ChatStreamText`, so the
/// text shows at once. A bubble that has streamed keeps `data-caret`, so the
/// caret fades out by transition instead of vanishing.
#[component]
pub fn ChatBubble(#[prop(optional)] variant: ChatBubbleVariant, #[prop(optional, into)] streaming: Signal<bool>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let caret = Memo::new(move |was: Option<&bool>| was.copied().unwrap_or(false) || streaming.get());
	let cls = cn!(CHAT_BUBBLE_BASE, variant.as_class(), class);
	view! {
		<div
			class=cls
			data-slot="chat-bubble"
			data-variant=variant.as_ref().to_owned()
			data-streaming=move || streaming.get().then_some("")
			data-caret=move || caret.get().then_some("")
		>
			{children()}
		</div>
	}
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
	#[prop(optional, into)] name: Option<ViewFn>,
	#[prop(optional, into)] time: Option<ViewFn>,
	#[prop(optional, into)] avatar: Option<ViewFn>,
	#[prop(optional)] compact: bool,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let user = role == ChatMessageRole::User;
	let cls = cn!(CHAT_MESSAGE_BASE, if user { CHAT_MESSAGE_USER } else { "" }, class);
	let body = cn!(CHAT_MESSAGE_BODY, if user { CHAT_MESSAGE_BODY_USER } else { CHAT_MESSAGE_BODY_ASSISTANT });
	let header = !compact && (name.is_some() || time.is_some());
	view! {
		<div class=cls data-slot="chat-message" data-role=role.as_ref().to_owned() data-enter="rise">
			{(!user && !compact)
				.then(|| {
					view! {
						<div class=CHAT_MESSAGE_AVATAR>
							{match avatar {
								Some(avatar) => avatar.run(),
								None => {
									view! {
										<span class=CHAT_MESSAGE_AVATAR_TILE>
											<Glyph class="size-4">
												<path d=SPARKLES />
											</Glyph>
										</span>
									}
										.into_any()
								}
							}}
						</div>
					}
				})}
			<div class=body>
				{header
					.then(|| {
						view! {
							<div class=CHAT_MESSAGE_HEADER>
								{name.map(|n| view! { <span class=CHAT_MESSAGE_NAME>{n.run()}</span> })}
								{time.map(|t| view! { <span class=CHAT_MESSAGE_TIME>{t.run()}</span> })}
							</div>
						}
					})} {children()}
			</div>
		</div>
	}
}

/// What a [`ChatMarker`] draws. Rust only: the TS marker is a union keyed on
/// `type`, and each kind carries its own fields here.
#[derive(Clone)]
pub enum ChatMarkerKind {
	/// Progress: "Generating response…", "Thought for 6s".
	Status(ChatStatusState),
	/// A tool call.
	Tool {
		state: ChatToolState,
		/// The tool's name.
		title: ViewFn,
		detail: Option<ViewFn>,
		/// The right-hand note: "Running…", "0.4s", "Failed · 1.2s".
		meta: Option<ViewFn>,
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
pub fn ChatMarker(kind: ChatMarkerKind, #[prop(optional, into)] class: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
	let children = move || children.map(|c| c());
	match kind {
		ChatMarkerKind::Separator => view! {
			<div class=cn!(CHAT_MARKER_SEPARATOR, class) data-slot="chat-marker" data-type="separator">
				<span class=CHAT_MARKER_RULE></span>
				<span class=CHAT_MARKER_SEPARATOR_LABEL>{children()}</span>
				<span class=CHAT_MARKER_RULE></span>
			</div>
		}
		.into_any(),
		ChatMarkerKind::Status(state) => {
			let running = state == ChatStatusState::Running;
			view! {
				<div
					class=cn!(CHAT_MARKER_STATUS, class)
					data-slot="chat-marker"
					data-type="status"
					data-state=state.as_ref().to_owned()
					role=running.then_some("status")
				>
					<span aria-hidden="true" class=cn!(CHAT_MARKER_DOT, state.as_class())></span>
					{children()}
				</div>
			}
			.into_any()
		}
		ChatMarkerKind::Tool { state, title, detail, meta } => {
			let error = state == ChatToolState::Error;
			let cls = cn!(CHAT_MARKER_TOOL, if error { CHAT_MARKER_TOOL_ERROR_BORDER } else { CHAT_MARKER_TOOL_BORDER }, class);
			let meta_cls = cn!(CHAT_MARKER_TOOL_META, state.as_class());
			view! {
				<div
					class=cls
					data-slot="chat-marker"
					data-type="tool"
					data-state=state.as_ref().to_owned()
				>
					<Glyph class=CHAT_MARKER_TOOL_ICON>
						{if error {
							view! {
								<circle cx="12" cy="12" r="10" />
								<path d="M12 8v4M12 16h.01" />
							}
								.into_any()
						} else {
							view! { <path d=SPARKLES /> }.into_any()
						}}
					</Glyph>
					<div class=CHAT_MARKER_TOOL_BODY>
						<span class=CHAT_MARKER_TOOL_TITLE>{title.run()}</span>
						{detail.map(|d| view! { <span class=CHAT_MARKER_TOOL_DETAIL>{d.run()}</span> })}
						{children()}
					</div>
					{meta.map(|m| view! { <span class=meta_cls>{m.run()}</span> })}
					{(state == ChatToolState::Done)
						.then(|| {
							view! {
								<Glyph class=CHAT_MARKER_TOOL_CHECK>
									<path d="M20 6 9 17l-5-5" />
								</Glyph>
							}
						})}
				</div>
			}
			.into_any()
		}
	}
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
	#[prop(optional)] r#type: ChatAttachmentType,
	#[prop(optional)] state: ChatAttachmentState,
	#[prop(into)] name: String,
	#[prop(optional, into)] meta: Option<ViewFn>,
	#[prop(optional, into)] progress: Signal<f64>,
	#[prop(optional)] on_remove: Option<Callback<()>>,
	#[prop(optional, into)] remove_label: Option<String>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let pct = Memo::new(move |_| progress.get().round().clamp(0.0, 100.0) as u8);
	let uploading = state == ChatAttachmentState::Uploading;
	let meta = match meta {
		Some(meta) => Some(meta.run()),
		None => uploading.then(|| (move || format!("Uploading… {}%", pct.get())).into_any()),
	};
	let remove_label = remove_label.unwrap_or_else(|| format!("Remove {name}"));
	view! {
		<div
			class=cn!(CHAT_ATTACHMENT_BASE, class)
			data-slot="chat-attachment"
			data-type=r#type.as_ref().to_owned()
			data-state=state.as_ref().to_owned()
		>
			<div class=CHAT_ATTACHMENT_ROW>
				<span class=CHAT_ATTACHMENT_ICON>
					<Glyph class="size-4">
						{match r#type {
							ChatAttachmentType::Image => {
								view! {
									<rect width="18" height="18" x="3" y="3" rx="2" />
									<circle cx="9" cy="9" r="2" />
									<path d="m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" />
								}
									.into_any()
							}
							ChatAttachmentType::File => {
								view! {
									<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z" />
									<path d="M14 2v4a2 2 0 0 0 2 2h4" />
								}
									.into_any()
							}
						}}
					</Glyph>
				</span>
				<div class=CHAT_ATTACHMENT_BODY>
					<span class=CHAT_ATTACHMENT_NAME>{name.clone()}</span>
					<span class=CHAT_ATTACHMENT_META>{meta}</span>
				</div>
				{on_remove
					.map(|remove| {
						view! {
							<IconButton
								aria_label=remove_label
								size=Size::Sm
								on_click=Callback::new(move |_| remove.run(()))
							>
								<Glyph class="size-4">
									<path d="M18 6 6 18M6 6l12 12" />
								</Glyph>
							</IconButton>
						}
					})}
			</div>
			{uploading
				.then(|| {
					view! {
						<div
							class=CHAT_ATTACHMENT_PROGRESS
							role="progressbar"
							aria-label=name
							aria-valuemin="0"
							aria-valuemax="100"
							aria-valuenow=move || pct.get().to_string()
						>
							<div
								class=CHAT_ATTACHMENT_PROGRESS_FILL
								style=move || format!("width: {}%", pct.get())
							></div>
						</div>
					}
				})}
		</div>
	}
}
