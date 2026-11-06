use crate::ui::uikit::core::KeyInput;

/// The `keyCode` every browser reports for a key that an IME takes.
const IME_KEY_CODE: u32 = 229;

/// One choice of a `ModelChip`. Mirrors the TS `ModelOption`.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelOption {
	pub id: String,
	pub label: String,
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
