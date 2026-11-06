use crate::ui::uikit::core::KeyInput;

/// The `keyCode` every browser reports for a key that an IME takes.
const IME_KEY_CODE: u32 = 229;

/// Who spoke a `ChatMessage`. Rust only: the TS `role` is an inline union.
#[derive(Clone, Copy, Debug, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatMessageRole {
	Assistant,
	User,
}

/// What a `ChatAttachment` holds; it picks the glyph. Rust only: the TS
/// `type` is an inline union.
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatAttachmentType {
	#[default]
	File,
	Image,
}

/// Whether a `ChatAttachment` is still uploading. Rust only, like
/// [`ChatAttachmentType`].
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ChatAttachmentState {
	#[default]
	Ready,
	Uploading,
}

/// One choice of a `ModelChip`. Mirrors the TS `ModelOption`.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelOption {
	pub id: String,
	pub label: String,
}

/// The `ChatComposer`'s own strings; every one defaults to English.
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

/// Whether a keydown sends the message: a plain Enter that no IME composition
/// owns. The Enter of an IME composition confirms the candidate. Safari reports
/// that Enter after `compositionend`, with `isComposing` already false but
/// `keyCode` 229, so both are checked, the same two checks as the TS composer.
/// `key_code` is the legacy `keyCode`, where the renderer carries one.
pub(crate) fn enter_sends(input: &KeyInput, key_code: Option<u32>) -> bool {
	input.key == "Enter" && !input.shift && !input.composing && key_code != Some(IME_KEY_CODE)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_safari_enter_with_key_code_229_does_not_send() {
		let enter = KeyInput::from("Enter");
		assert!(!enter_sends(&enter, Some(IME_KEY_CODE)));
		assert!(enter_sends(&enter, Some(13)));
		assert!(enter_sends(&enter, None));
		assert!(!enter_sends(&KeyInput { composing: true, ..enter.clone() }, None));
		assert!(!enter_sends(&KeyInput { shift: true, ..enter }, None));
		assert!(!enter_sends(&KeyInput::from("Process"), Some(IME_KEY_CODE)));
		assert!(!enter_sends(&KeyInput::from("ы"), None));
	}
}
