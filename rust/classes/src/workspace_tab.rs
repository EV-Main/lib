pub const WORKSPACE_TABS_BASE: &str = "flex items-center gap-1";

/// The ring is drawn on the whole pill, not on the inner tab button, so the
/// close button sits inside the focus outline with its tab.
pub const WORKSPACE_TAB_BASE: &str = "group inline-flex h-7 items-center rounded-(--control-radius) border border-transparent text-body-s font-medium text-ink-mid transition-colors \
                                      hover:bg-hover hover:text-ink has-[:focus-visible]:ring-2 has-[:focus-visible]:ring-ring";

/// The hover repeats the fill, or the generic hover would wash the open tab
/// back to the closed look.
pub const WORKSPACE_TAB_SELECTED: &str = "border-border-subtle bg-secondary text-ink hover:bg-secondary";

/// `min-w-0` lets a long title truncate instead of widening the row.
pub const WORKSPACE_TAB_TRIGGER: &str = "inline-flex h-full min-w-0 items-center gap-2 rounded-[inherit] pl-2 outline-none";

pub const WORKSPACE_TAB_LABEL: &str = "truncate";

pub const WORKSPACE_TAB_CLOSE: &str =
	"inline-flex size-6 items-center justify-center rounded-(--control-radius) text-ink-soft outline-none hover:bg-hover hover:text-ink focus-visible:ring-2 focus-visible:ring-ring";

/// Stands in for the close button on a tab that cannot close, so the title
/// keeps the same right inset.
pub const WORKSPACE_TAB_SPACER: &str = "w-2";
