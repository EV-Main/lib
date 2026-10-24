use tailwind_fuse::{AsTailwindClass, TwVariant};

/// 6 px. The dot is decorative, so the label beside it carries the meaning.
pub const STATUS_DOT_BASE: &str = "size-1.5 shrink-0 rounded-full";

pub const STATUS_ITEM_BASE: &str = "inline-flex h-5 items-center gap-1.5 px-2 text-caption text-ink-mid";

/// Tabular figures, so a changing latency does not shift the row.
pub const STATUS_ITEM_VALUE: &str = "tabular-nums text-ink";

/// The dot fill per tone. Mirrors the TS `StatusTone`.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum StatusTone {
	#[tw(class = "bg-positive")]
	Positive,
	#[tw(default, class = "bg-ink-soft")]
	Neutral,
	#[tw(class = "bg-accent-warn")]
	Warn,
	#[tw(class = "bg-accent-error")]
	Error,
	#[tw(class = "bg-ink-soft/50")]
	Muted,
}
