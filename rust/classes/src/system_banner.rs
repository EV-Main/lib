/// Wears its [`Tone`](crate::Tone) whole: fill, edge and ink.
pub const SYSTEM_BANNER_BASE: &str = "flex items-start gap-2.5 rounded-panel border border-(color:--tone-edge) bg-(color:--tone-fill) px-4 py-2.5 text-sm text-(color:--tone-ink)";

pub const SYSTEM_BANNER_ICON: &str = "mt-0.5 flex shrink-0";

pub const SYSTEM_BANNER_GLYPH: &str = "size-4";

/// `min-w-0` lets a long message wrap instead of pushing the action out.
pub const SYSTEM_BANNER_MESSAGE: &str = "min-w-0 flex-1";

pub const SYSTEM_BANNER_TITLE: &str = "font-semibold";

/// The message steps back to the soft ink only when a bold title leads it.
pub const SYSTEM_BANNER_BODY_AFTER_TITLE: &str = "text-ink-soft";

pub const SYSTEM_BANNER_ACTION: &str = "shrink-0";

/// A 24 px hit area; the negative margins keep it from growing the strip.
pub const SYSTEM_BANNER_DISMISS: &str = "-my-0.5 -mr-1 flex size-6 shrink-0 items-center justify-center rounded-control opacity-70 outline-none \
                                         transition-opacity hover:opacity-100 focus-visible:ring-2 focus-visible:ring-ring";
