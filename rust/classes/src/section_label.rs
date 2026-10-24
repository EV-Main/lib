use tailwind_fuse::{AsTailwindClass, TwVariant};

/// Plain body-s text: no case change and no tracking, so a label reads as a
/// quiet caption, not as a shouted heading.
pub const SECTION_LABEL_BASE: &str = "text-body-s";

/// The label ink per tone. Mirrors the TS `SectionLabelTone`.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum SectionLabelTone {
	/// Labels a group or a section. The padding lines it up with the rows of a nav.
	#[tw(default, class = "px-2 py-1 text-ink-soft")]
	Muted,
	/// Announces a hero figure.
	#[tw(class = "font-medium text-primary-ink")]
	Accent,
}
