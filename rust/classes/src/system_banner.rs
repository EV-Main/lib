use tailwind_fuse::{AsTailwindClass, TwVariant};

/// The tone's fill rides on it: [`system_banner_tone_class`].
pub const SYSTEM_BANNER_BASE: &str = "flex items-start gap-2.5 rounded-lg border px-4 py-2.5 text-sm";

pub const SYSTEM_BANNER_ICON: &str = "mt-0.5 flex shrink-0";

pub const SYSTEM_BANNER_GLYPH: &str = "size-4";

/// `min-w-0` lets a long message wrap instead of pushing the action out.
pub const SYSTEM_BANNER_MESSAGE: &str = "min-w-0 flex-1";

pub const SYSTEM_BANNER_TITLE: &str = "font-semibold";

/// The message steps back to the soft ink only when a bold title leads it.
pub const SYSTEM_BANNER_BODY_AFTER_TITLE: &str = "text-ink-soft";

pub const SYSTEM_BANNER_ACTION: &str = "shrink-0";

/// A 24 px hit area; the negative margins keep it from growing the strip.
pub const SYSTEM_BANNER_DISMISS: &str = "-my-0.5 -mr-1 flex size-6 shrink-0 items-center justify-center rounded-md opacity-70 outline-none \
                                         transition-opacity hover:opacity-100 focus-visible:ring-2 focus-visible:ring-ring";

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
	#[tw(class = "text-accent-warn")]
	Warn,
}

/// The strip's edge, fill and text per tone, fused over [`SYSTEM_BANNER_BASE`].
pub fn system_banner_tone_class(tone: &SystemBannerTone) -> &'static str {
	match tone {
		SystemBannerTone::Neutral | SystemBannerTone::Info => "border-border bg-card text-ink",
		SystemBannerTone::Primary => "border-primary-ink/40 bg-primary-ink/10 text-primary-ink",
		SystemBannerTone::Warn => "border-accent-warn/40 bg-accent-warn/10 text-accent-warn",
	}
}
