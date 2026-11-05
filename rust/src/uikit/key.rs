/// A keydown as the core reads it: the `key` the user's layout produced (the DOM
/// `KeyboardEvent.key` spelling, e.g. `"ArrowDown"`, `"a"`), its modifiers, and
/// whether an IME composition is under way. Each port converts its own event.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct KeyInput {
	pub key: String,
	pub ctrl: bool,
	pub shift: bool,
	pub alt: bool,
	pub meta: bool,
	pub composing: bool,
}

#[cfg(test)]
impl From<&str> for KeyInput {
	fn from(key: &str) -> Self {
		Self {
			key: key.to_owned(),
			..Self::default()
		}
	}
}
