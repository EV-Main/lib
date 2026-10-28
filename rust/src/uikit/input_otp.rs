/// One `InputOTP`: a single hidden input holds the code; the slots render it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct InputOtp {
	pub max_length: usize,
}
impl InputOtp {
	/// What the hidden input's text becomes the code as: cut to `max_length`.
	pub(crate) fn accept(&self, typed: &str) -> String {
		typed.chars().take(self.max_length).collect()
	}

	/// What the slot at `index` shows of `value`.
	pub(crate) fn slot(&self, value: &str, focused: bool, index: usize) -> OtpSlot {
		let char = value.chars().nth(index);
		// Where the next character lands, the mirror of the TS
		// `focused ? Math.min(value.length, maxLength - 1) : -1`.
		let active = focused && index == value.chars().count().min(self.max_length.saturating_sub(1));
		OtpSlot {
			char,
			active,
			caret: active && char.is_none(),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct OtpSlot {
	pub char: Option<char>,
	/// Carries the ring: the slot the next character lands in.
	pub active: bool,
	/// The fake caret, on an active slot still empty.
	pub caret: bool,
}

#[cfg(test)]
mod tests {
	use super::*;

	const FOUR: InputOtp = InputOtp { max_length: 4 };

	fn active(value: &str, focused: bool) -> Option<usize> {
		(0..FOUR.max_length).find(|i| FOUR.slot(value, focused, *i).active)
	}

	#[test]
	fn the_active_slot_follows_the_typed_value_and_clamps_to_the_last() {
		assert_eq!(active("", true), Some(0));
		assert_eq!(active("12", true), Some(2), "the ring follows the caret");
		assert_eq!(active("1234", true), Some(3));
		assert_eq!(active("12", false), None, "nothing is active while unfocused");
	}

	#[test]
	fn only_an_empty_active_slot_shows_the_caret() {
		assert!(FOUR.slot("12", true, 2).caret);
		assert!(!FOUR.slot("1234", true, 3).caret, "a filled slot has no caret");
		assert_eq!(FOUR.slot("12", true, 1).char, Some('2'));
	}

	#[test]
	fn typing_past_max_length_is_cut() {
		assert_eq!(FOUR.accept("123456"), "1234");
	}
}
