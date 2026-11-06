/// The `CommandPalette`'s own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandPaletteLabels {
	/// Names the dialog for assistive tech; "Command palette".
	pub title: Option<String>,
	/// The search field; "Type a command or search…".
	pub placeholder: Option<String>,
	/// Shown when the search matches nothing; "No results.".
	pub empty: Option<String>,
	/// The ↑↓ hint; "navigate".
	pub navigate: Option<String>,
	/// The ↵ hint; "select".
	pub select: Option<String>,
	/// The esc hint; "close".
	pub close: Option<String>,
}

/// What one `keydown` means to the ⌘K / Ctrl+K shortcut.
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ShortcutHit {
	/// Not ⌘K / Ctrl+K: the key goes on as usual.
	Miss,
	/// ⌘K / Ctrl+K held down: the browser must still not take it, but the
	/// palette must not toggle again.
	Held,
	/// ⌘K / Ctrl+K pressed: take it from the browser and fire.
	Fire,
}

/// The parts of a `keydown` the shortcut reads. The layout's `key` is not one
/// of them: on a Russian layout the K key reports `key == "л"` and the same
/// `code`.
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
pub(crate) struct ShortcutKey<'a> {
	pub code: &'a str,
	pub ctrl: bool,
	pub meta: bool,
	pub alt: bool,
	pub shift: bool,
	pub repeat: bool,
}

#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
pub(crate) fn shortcut_hit(k: &ShortcutKey<'_>) -> ShortcutHit {
	if !(k.meta || k.ctrl) || k.alt || k.shift || k.code != "KeyK" {
		return ShortcutHit::Miss;
	}
	if k.repeat { ShortcutHit::Held } else { ShortcutHit::Fire }
}

#[cfg(test)]
mod tests {
	use super::*;

	fn key(code: &str) -> ShortcutKey<'_> {
		ShortcutKey {
			code,
			ctrl: false,
			meta: false,
			alt: false,
			shift: false,
			repeat: false,
		}
	}

	#[test]
	fn the_shortcut_fires_on_ctrl_k_and_cmd_k_on_any_layout() {
		assert_eq!(shortcut_hit(&ShortcutKey { ctrl: true, ..key("KeyK") }), ShortcutHit::Fire);
		// A Russian layout: `key` is "л", `code` stays "KeyK".
		assert_eq!(shortcut_hit(&ShortcutKey { meta: true, ..key("KeyK") }), ShortcutHit::Fire);
	}

	#[test]
	fn the_shortcut_ignores_a_held_key_but_still_keeps_it_from_the_browser() {
		assert_eq!(
			shortcut_hit(&ShortcutKey {
				ctrl: true,
				repeat: true,
				..key("KeyK")
			}),
			ShortcutHit::Held
		);
	}

	#[test]
	fn the_shortcut_ignores_a_bare_k_other_chords_and_other_keys() {
		assert_eq!(shortcut_hit(&key("KeyK")), ShortcutHit::Miss);
		assert_eq!(
			shortcut_hit(&ShortcutKey {
				ctrl: true,
				shift: true,
				..key("KeyK")
			}),
			ShortcutHit::Miss
		);
		assert_eq!(
			shortcut_hit(&ShortcutKey {
				meta: true,
				alt: true,
				..key("KeyK")
			}),
			ShortcutHit::Miss
		);
		// A Dvorak "k" sits on another physical key.
		assert_eq!(shortcut_hit(&ShortcutKey { ctrl: true, ..key("KeyV") }), ShortcutHit::Miss);
	}
}
