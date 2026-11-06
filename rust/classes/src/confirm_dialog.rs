use tailwind_fuse::{AsTailwindClass, TwVariant};

pub const CONFIRM_DIALOG_CONTENT: &str = "max-w-105 gap-4 bg-popover p-5";

pub const CONFIRM_DIALOG_HEADER: &str = "flex-row gap-3 text-left";

pub const CONFIRM_DIALOG_ICON: &str = "flex size-8 shrink-0 items-center justify-center rounded-(--control-radius)";

pub const CONFIRM_DIALOG_TEXT: &str = "flex min-w-0 flex-col gap-1";

pub const CONFIRM_DIALOG_TITLE: &str = "text-h3 font-medium text-ink";

pub const CONFIRM_DIALOG_DESCRIPTION: &str = "text-body text-ink-mid";

pub const CONFIRM_DIALOG_ERROR: &str = "text-body text-accent-error";

pub const CONFIRM_DIALOG_FOOTER: &str = "flex-row justify-end gap-2";

/// Keyed on `aria-disabled`, not `disabled`: a disabled button that holds
/// focus drops it to `<body>`, and Tab then leaves the dialog.
pub const CONFIRM_DIALOG_PENDING: &str = "aria-disabled:cursor-not-allowed aria-disabled:opacity-40";

pub const CONFIRM_DIALOG_SPINNER: &str = "size-3.5";

/// The glyph tile per tone. Mirrors the TS `ConfirmDialogTone`; the confirm
/// button follows the tone too (`danger` or `solid`).
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ConfirmDialogTone {
	#[tw(default, class = "bg-muted text-ink-mid")]
	Default,
	#[tw(class = "bg-chip-error-bg text-accent-error")]
	Danger,
}

/// The `ConfirmDialog`'s own strings. English by default; a caller translates by overriding fields.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(serde::Serialize), serde(rename_all = "camelCase"))]
pub struct ConfirmDialogLabels {
	/// The cancel button.
	pub cancel: String,
	/// Shown when the confirm action fails without a message.
	pub error: String,
}
impl Default for ConfirmDialogLabels {
	fn default() -> Self {
		Self {
			cancel: "Cancel".to_owned(),
			error: "Something went wrong".to_owned(),
		}
	}
}
