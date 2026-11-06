pub const CONFIRM_DIALOG_CONTENT: &str = "max-w-105 gap-4 bg-popover p-5";

pub const CONFIRM_DIALOG_HEADER: &str = "flex-row gap-3 text-left";

/// The glyph tile wears the dialog's [`Tone`](crate::Tone): its fill and ink.
pub const CONFIRM_DIALOG_ICON: &str = "flex size-8 shrink-0 items-center justify-center rounded-control bg-(color:--tone-fill) text-(color:--tone-ink)";

pub const CONFIRM_DIALOG_TEXT: &str = "flex min-w-0 flex-col gap-1";

pub const CONFIRM_DIALOG_TITLE: &str = "text-h3 font-medium text-ink";

pub const CONFIRM_DIALOG_DESCRIPTION: &str = "text-body text-ink-mid";

pub const CONFIRM_DIALOG_ERROR: &str = "text-body text-accent-error";

pub const CONFIRM_DIALOG_FOOTER: &str = "flex-row justify-end gap-2";

/// Keyed on `aria-disabled`, not `disabled`: a disabled button that holds
/// focus drops it to `<body>`, and Tab then leaves the dialog.
pub const CONFIRM_DIALOG_PENDING: &str = "aria-disabled:cursor-not-allowed aria-disabled:opacity-40";

pub const CONFIRM_DIALOG_SPINNER: &str = "size-3.5";
