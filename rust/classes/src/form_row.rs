/// Stacks on a phone; label and control sit side by side from `sm` up.
pub const FORM_ROW_BASE: &str = "flex flex-col gap-1.5 px-4 py-3 sm:flex-row sm:gap-6";

/// The hairline under a row. The last row of a group drops it.
pub const FORM_ROW_DIVIDER: &str = "border-b border-border-subtle";

/// A fixed width from `sm` up, so the controls line up down the page.
pub const FORM_ROW_LABEL_COLUMN: &str = "flex min-w-0 flex-col gap-0.5 sm:w-52 sm:shrink-0";

pub const FORM_ROW_LABEL: &str = "text-body text-ink-mid";

pub const FORM_ROW_DESCRIPTION: &str = "text-caption text-ink-soft";

pub const FORM_ROW_CONTROL: &str = "flex min-w-0 flex-1 flex-col gap-1 text-body text-ink";

pub const FORM_ROW_HINT: &str = "text-caption text-ink-soft";
