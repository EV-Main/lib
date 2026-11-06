use tailwind_fuse::{AsTailwindClass, TwVariant};

pub const SECTION_LABEL_BASE: &str = "text-xs font-semibold uppercase tracking-widest";

/// The label ink per tone. Mirrors the TS `SectionLabelTone`.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum SectionLabelTone {
	/// Labels a group or a section.
	#[tw(default, class = "text-ink-soft")]
	Muted,
	/// Announces a hero figure.
	#[tw(class = "text-primary-ink")]
	Accent,
}
