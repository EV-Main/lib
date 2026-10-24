use tailwind_fuse::{AsTailwindClass, TwVariant};

pub const HEADER_BALANCE_BASE: &str = "inline-flex h-8 items-center gap-2 rounded-(--control-radius) border border-border bg-secondary pl-2.5 pr-2 text-body-s";

/// Added only when the balance is a button: a static figure must not look pressable.
pub const HEADER_BALANCE_INTERACTIVE: &str = "cursor-pointer outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring";

pub const HEADER_BALANCE_LABEL: &str = "text-ink-soft";

/// The TS `Settled` wrapper around the figure. The Dioxus port has no
/// `Settled`, so only the TS side reads it.
pub const HEADER_BALANCE_SETTLED: &str = "inline-flex";

pub const HEADER_BALANCE_SKELETON: &str = "h-3 w-12";

pub const HEADER_BALANCE_FIGURE: &str = "font-medium tabular-nums";

pub const HEADER_BALANCE_CHEVRON: &str = "size-4 text-ink-soft";

/// The figure's ink. `Warn` is the low-funds signal.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum HeaderBalanceTone {
	#[tw(default, class = "text-ink")]
	Default,
	#[tw(class = "text-accent-warn-ink")]
	Warn,
}

/// The dot is absolutely placed over the glyph, so the button is its frame.
pub const NOTIFICATION_BELL_TRIGGER: &str = "relative";

pub const NOTIFICATION_DOT: &str = "absolute right-1 top-1 size-1.5 rounded-full bg-accent-error";

pub const NOTIFICATION_BELL_CONTENT: &str = "w-80 p-1";

pub const NOTIFICATION_BELL_EMPTY: &str = "px-2 py-3 text-center text-body text-ink-soft";

pub const NOTIFICATION_BELL_LIST: &str = "flex max-h-80 flex-col gap-0.5 overflow-y-auto";

pub const NOTIFICATION_BELL_ITEM: &str =
	"flex w-full items-start gap-2 rounded-(--control-radius) px-2 py-1.5 text-left outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring";

pub const NOTIFICATION_BELL_ITEM_DOT: &str = "mt-1.5 size-1.5 shrink-0 rounded-full";

pub const NOTIFICATION_BELL_ITEM_DOT_UNREAD: &str = "bg-primary";

/// Transparent, not absent: read and unread titles stay on one left edge.
pub const NOTIFICATION_BELL_ITEM_DOT_READ: &str = "bg-transparent";

pub const NOTIFICATION_BELL_ITEM_BODY: &str = "flex min-w-0 flex-1 flex-col gap-0.5";

pub const NOTIFICATION_BELL_ITEM_TITLE: &str = "text-body text-ink";

pub const NOTIFICATION_BELL_ITEM_TITLE_UNREAD: &str = "font-medium";

pub const NOTIFICATION_BELL_ITEM_DESCRIPTION: &str = "text-body-s text-ink-mid";

pub const NOTIFICATION_BELL_ITEM_TIME: &str = "shrink-0 text-caption text-ink-soft";

pub const NOTIFICATION_BELL_FOOTER: &str = "border-t border-border-subtle p-1 pt-1.5";

pub const HEADER_SEARCH_BASE: &str = "flex h-8 w-55 items-center gap-2 rounded-(--control-radius) border border-border bg-muted pl-2.5 pr-1.5 text-left text-body text-ink-mid outline-none hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring";

/// `min-w-0` with `truncate`: a long placeholder ends in an ellipsis and the
/// shortcut hint stays visible.
pub const HEADER_SEARCH_PLACEHOLDER: &str = "min-w-0 flex-1 truncate";
