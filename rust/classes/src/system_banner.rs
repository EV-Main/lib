use tailwind_fuse::{AsTailwindClass, TwVariant};

/// The strip stays on the card fill for every tone: a platform notice must not
/// shout louder than the page, so only the glyph carries the tone. The right
/// padding keeps the text clear of the absolutely placed close button.
pub const SYSTEM_BANNER_BASE: &str = "relative flex min-h-8 items-center justify-center gap-2 border-b border-border bg-card py-1.5 pl-4 pr-10 text-center text-body text-ink";

pub const SYSTEM_BANNER_ICON: &str = "flex shrink-0";

pub const SYSTEM_BANNER_GLYPH: &str = "size-4";

/// `min-w-0` lets a long message wrap instead of pushing the action out.
pub const SYSTEM_BANNER_MESSAGE: &str = "min-w-0";

pub const SYSTEM_BANNER_TITLE: &str = "font-medium";

/// The message steps back to the mid ink only when a bold title leads it.
pub const SYSTEM_BANNER_BODY_AFTER_TITLE: &str = "text-ink-mid";

pub const SYSTEM_BANNER_ACTION: &str = "shrink-0";

/// Its own face, not an `IconButton`: it is pinned to the strip's right edge
/// whatever the centred content does.
pub const SYSTEM_BANNER_DISMISS: &str = "absolute right-3 top-1/2 flex size-6 -translate-y-1/2 items-center justify-center rounded-(--control-radius) text-ink-mid outline-none hover:bg-hover hover:text-ink focus-visible:ring-2 focus-visible:ring-ring";

/// The glyph ink per tone. Mirrors the TS `SystemBannerTone`.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum SystemBannerTone {
	#[tw(class = "text-ink-soft")]
	Neutral,
	#[tw(class = "text-primary-ink")]
	Primary,
	/// An announcement.
	#[tw(default, class = "text-accent-info-ink")]
	Info,
	/// A platform state: maintenance, read-only.
	#[tw(class = "text-accent-warn-ink")]
	Warn,
}
