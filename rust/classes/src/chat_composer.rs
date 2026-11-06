pub const MODEL_CHIP_BASE: &str = "inline-flex h-6.5 items-center gap-1 rounded-chip border border-border py-1 pl-2 pr-1.5 text-body-s text-ink-mid outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring";

pub const CHAT_COMPOSER_BASE: &str = "flex flex-col gap-2 rounded-panel border bg-card px-3 pb-2.5 pt-3 focus-within:ring-2 focus-within:ring-ring";

/// The edge softens while the assistant answers.
pub const CHAT_COMPOSER_STREAMING: &str = "border-border";

pub const CHAT_COMPOSER_IDLE: &str = "border-border-strong";

pub const CHAT_COMPOSER_ATTACHMENTS: &str = "flex flex-wrap gap-2";

/// Fused over the `Textarea` base: the field drops its own box and lets the
/// composer's border and ring stand for it. It grows with `field-sizing-content`
/// from the base up to `max-h-40`.
pub const CHAT_COMPOSER_FIELD: &str = "max-h-40 min-h-6 resize-none border-0 bg-transparent p-0 focus-visible:ring-0";

pub const CHAT_COMPOSER_TOOLBAR: &str = "flex items-center gap-1.5";

/// Pushes the hint and the send button to the toolbar end.
pub const CHAT_COMPOSER_SPACER: &str = "flex-1";

pub const CHAT_COMPOSER_HINT: &str = "hidden text-caption text-ink-soft sm:inline";

/// The `ChatComposer`'s own strings. English by default; a caller translates by overriding fields.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(serde::Serialize), serde(rename_all = "camelCase"))]
pub struct ChatComposerLabels {
	/// Names the message field.
	pub field: String,
	/// The send button.
	pub send: String,
	/// The stop button while streaming.
	pub stop: String,
	/// The attach button.
	pub attach: String,
}
impl Default for ChatComposerLabels {
	fn default() -> Self {
		Self {
			field: "Message".to_owned(),
			send: "Send".to_owned(),
			stop: "Stop".to_owned(),
			attach: "Attach file".to_owned(),
		}
	}
}
