use leptos::prelude::*;

pub use crate::uikit::{ChatComposerLabels, ModelOption};
use crate::{
	cn,
	uikit::{KeyInput, enter_sends},
	uikit_leptos::{
		CHAT_COMPOSER_ATTACHMENTS, CHAT_COMPOSER_BASE, CHAT_COMPOSER_FIELD, CHAT_COMPOSER_HINT, CHAT_COMPOSER_IDLE, CHAT_COMPOSER_SPACER, CHAT_COMPOSER_STREAMING, CHAT_COMPOSER_TOOLBAR,
		DropdownMenu, DropdownMenuContent, DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuTrigger, IconButton, IconButtonVariant, MODEL_CHIP_BASE, Size, Textarea,
		icon_button::Glyph, primitives::use_controllable,
	},
};

/// The small bordered button naming the model; it opens a radio menu of the
/// options. Mirrors the TS `ModelChip`.
///
/// The trigger is named "{aria_label}: {current label}", and falls back to the
/// raw `value` when no option has that id.
#[component]
pub fn ModelChip(
	models: Vec<ModelOption>,
	#[prop(into)] value: Signal<String>,
	on_value_change: Callback<String>,
	#[prop(default = String::from("Model"), into)] aria_label: String,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let options = StoredValue::new(models.clone());
	let shown = Memo::new(move |_| {
		let value = value.get();
		options.with_value(|ms| ms.iter().find(|m| m.id == value).map_or(value.clone(), |m| m.label.clone()))
	});
	view! {
		<DropdownMenu>
			<DropdownMenuTrigger
				aria_label=Signal::derive(move || format!("{aria_label}: {}", shown.get()))
				class=cn!(MODEL_CHIP_BASE, class)
			>
				{move || shown.get()}
				<Glyph class="size-4">
					<path d="m6 9 6 6 6-6" />
				</Glyph>
			</DropdownMenuTrigger>
			<DropdownMenuContent>
				<DropdownMenuRadioGroup value=value on_value_change=on_value_change>
					{options
						.get_value()
						.into_iter()
						.map(|m| view! { <DropdownMenuRadioItem value=m.id>{m.label}</DropdownMenuRadioItem> })
						.collect_view()}
				</DropdownMenuRadioGroup>
			</DropdownMenuContent>
		</DropdownMenu>
	}
}

/// The message field with its toolbar. Mirrors the TS `ChatComposer`.
///
/// Enter or the send button calls `on_send` with the trimmed text, then clears
/// the field; Shift+Enter keeps the newline. An empty or whitespace-only
/// message is not sent, and `disabled` sends nothing. While `streaming`, the
/// send button becomes Stop, but Enter still queues a follow-up.
///
/// Enter that confirms an IME candidate never sends, as in TS.
///
/// The field grows with its text through the `Textarea` base's
/// `field-sizing-content`, up to `max-h-40`, as in TS; there is no JS sizing.
#[component]
pub fn ChatComposer(
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	on_send: Callback<String>,
	#[prop(optional, into)] streaming: Signal<bool>,
	#[prop(optional)] on_stop: Option<Callback<()>>,
	#[prop(default = String::from("Ask the assistant…"), into)] placeholder: String,
	/// Shows the attach button.
	#[prop(optional)]
	on_attach: Option<Callback<()>>,
	/// A row of `ChatAttachment`s above the field.
	#[prop(optional, into)]
	attachments: Option<ViewFn>,
	/// Model chips and other toolbar controls, after the attach button.
	#[prop(optional, into)]
	tools: Option<ViewFn>,
	/// A caption in the toolbar: "Enter to send · Shift+Enter for a new line".
	#[prop(optional, into)]
	hint: Option<ViewFn>,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional)] labels: ChatComposerLabels,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let text = use_controllable(value, default_value, on_value_change);
	let field = NodeRef::<leptos::html::Textarea>::new();
	let empty = Memo::new(move |_| text.get().trim().is_empty());
	// Reads the text at call time, so an Enter right after an input sends what
	// that input left, not the value of the last render.
	let send = move || {
		let current = text.get_untracked();
		let trimmed = current.trim();
		if trimmed.is_empty() || disabled.get_untracked() {
			return;
		}
		on_send.run(trimmed.to_string());
		text.set(String::new());
	};
	let ChatComposerLabels {
		field: field_label,
		send: send_label,
		stop: stop_label,
		attach: attach_label,
	} = labels;
	let cls = move || cn!(CHAT_COMPOSER_BASE, if streaming.get() { CHAT_COMPOSER_STREAMING } else { CHAT_COMPOSER_IDLE }, &class);
	view! {
		<div
			class=cls
			data-slot="chat-composer"
			data-state=move || if streaming.get() { "streaming" } else { "default" }
		>
			{attachments.map(|a| view! { <div class=CHAT_COMPOSER_ATTACHMENTS>{a.run()}</div> })}
			<Textarea
				rows=1
				value=Signal::derive(move || text.get())
				disabled=disabled
				placeholder=placeholder
				aria_label=field_label.unwrap_or_else(|| "Message".into())
				class=CHAT_COMPOSER_FIELD
				on_input=Callback::new(move |e| text.set(event_target_value(&e)))
				on_keydown=Callback::new(move |e: leptos::ev::KeyboardEvent| {
					if enter_sends(&KeyInput::from(&e), Some(e.key_code())) {
						e.prevent_default();
						send();
					}
				})
				node_ref=field
			/>
			<div class=CHAT_COMPOSER_TOOLBAR>
				{on_attach
					.map(|attach| {
						view! {
							<IconButton
								aria_label=attach_label.unwrap_or_else(|| "Attach file".into())
								size=Size::Sm
								disabled=disabled
								on_click=Callback::new(move |_| attach.run(()))
							>
								<Glyph class="size-4">
									<path d="M5 12h14M12 5v14" />
								</Glyph>
							</IconButton>
						}
					})} {tools.map(|t| t.run())} <span class=CHAT_COMPOSER_SPACER></span>
				{hint.map(|h| view! { <span class=CHAT_COMPOSER_HINT>{h.run()}</span> })}
				{
					let stop_label = stop_label.unwrap_or_else(|| "Stop".into());
					let send_label = send_label.unwrap_or_else(|| "Send".into());
					move || {
						if streaming.get() {
							view! {
								<IconButton
									variant=IconButtonVariant::Secondary
									aria_label=stop_label.clone()
									on_click=Callback::new(move |_| {
										if let Some(stop) = on_stop {
											stop.run(());
										}
										#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
										if let Some(el) = field.get_untracked() {
											let _ = el.focus();
										}
									})
								>
									<Glyph class="size-4">
										<rect width="12" height="12" x="6" y="6" rx="2" />
									</Glyph>
								</IconButton>
							}
								.into_any()
						} else {
							view! {
								<IconButton
									variant=IconButtonVariant::Secondary
									aria_label=send_label.clone()
									disabled=Signal::derive(move || disabled.get() || empty.get())
									on_click=Callback::new(move |_| send())
								>
									<Glyph class="size-4">
										<path d="m5 12 7-7 7 7M12 19V5" />
									</Glyph>
								</IconButton>
							}
								.into_any()
						}
					}
				}
			</div>
		</div>
	}
}
