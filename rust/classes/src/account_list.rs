pub const ACCOUNT_LIST_BASE: &str = "flex flex-col gap-2";

pub const ACCOUNT_LIST_ITEM_BASE: &str = "flex items-center gap-3 rounded-(--control-radius) border p-2.5 pr-3 transition-colors";

pub const ACCOUNT_LIST_ITEM_CHECKED: &str = "border-primary-border bg-selected";

pub const ACCOUNT_LIST_ITEM_UNCHECKED: &str = "border-border-subtle bg-muted hover:bg-hover";

pub const ACCOUNT_LIST_ITEM_DISABLED: &str = "opacity-40";

/// `min-w-0` lets a long name truncate instead of pushing the actions out of
/// the row.
pub const ACCOUNT_LIST_ITEM_RADIO: &str = "flex min-w-0 flex-1 items-center gap-3 rounded-(--control-radius) text-left outline-none focus-visible:ring-2 focus-visible:ring-ring";

pub const ACCOUNT_LIST_ITEM_AVATAR: &str = "size-14 shrink-0 rounded-(--control-radius)";

pub const ACCOUNT_LIST_ITEM_AVATAR_FALLBACK: &str = "font-display text-h2 font-light text-ink";

pub const ACCOUNT_LIST_ITEM_TEXT: &str = "flex min-w-0 flex-1 flex-col gap-0.5";

pub const ACCOUNT_LIST_ITEM_NAME: &str = "truncate text-body-l text-ink";

pub const ACCOUNT_LIST_ITEM_HANDLE: &str = "truncate text-body text-ink-mid";

pub const ACCOUNT_LIST_ITEM_BADGES: &str = "flex shrink-0 items-center gap-1.5";

pub const ACCOUNT_LIST_ITEM_FIGURE: &str = "shrink-0 font-display text-figure-m font-light tabular-nums text-ink";

pub const ACCOUNT_LIST_ITEM_CHECK: &str = "flex size-4 shrink-0 text-primary-ink";

/// Hidden, not removed: the mark keeps its width, so the figure does not
/// shift when the selection moves.
pub const ACCOUNT_LIST_ITEM_CHECK_HIDDEN: &str = "invisible";

pub const ACCOUNT_LIST_ITEM_ACTIONS: &str = "flex shrink-0 items-center gap-1";
