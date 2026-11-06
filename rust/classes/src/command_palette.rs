/// Fused over the dialog content's own classes: it drops the palette into the top
/// sixth of the screen instead of centring it.
pub const COMMAND_PALETTE_CONTENT: &str = "top-1/6 max-w-[calc(100%-2rem)] translate-y-0 gap-0 overflow-hidden bg-popover p-0 sm:max-w-105";

pub const COMMAND_PALETTE_TITLE: &str = "sr-only";

pub const COMMAND_PALETTE_COMMAND: &str = "bg-transparent";

pub const COMMAND_PALETTE_LIST: &str = "max-h-72";

pub const COMMAND_PALETTE_LABEL: &str = "min-w-0 flex-1 truncate";

pub const COMMAND_PALETTE_HINTS: &str = "flex items-center gap-3 border-t border-border-subtle px-2 py-1 text-caption text-ink-soft";

pub const COMMAND_PALETTE_HINT: &str = "inline-flex items-center gap-1.5";

/// The `CommandPalette`'s own strings. English by default; a caller translates by overriding fields.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(serde::Serialize), serde(rename_all = "camelCase"))]
pub struct CommandPaletteLabels {
	/// The dialog's accessible title.
	pub title: String,
	/// The search field's placeholder.
	pub placeholder: String,
	/// Shown when nothing matches.
	pub empty: String,
	/// The arrow-keys hint.
	pub navigate: String,
	/// The Enter hint.
	pub select: String,
	/// The Escape hint.
	pub close: String,
}
impl Default for CommandPaletteLabels {
	fn default() -> Self {
		Self {
			title: "Command palette".to_owned(),
			placeholder: "Type a command or search…".to_owned(),
			empty: "No results.".to_owned(),
			navigate: "navigate".to_owned(),
			select: "select".to_owned(),
			close: "close".to_owned(),
		}
	}
}
