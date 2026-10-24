use tailwind_fuse::{AsTailwindClass, TwVariant};

/// Centres a 1408 px measure; the rail goes beside the cards from `lg` up.
pub const SETTINGS_LAYOUT: &str = "mx-auto flex w-full max-w-352 flex-col gap-6 px-4 py-8 lg:flex-row lg:gap-8";

/// 256 px wide from `lg` up, on top of the cards below it.
pub const SETTINGS_NAV: &str = "shrink-0 lg:w-64";

pub const SETTINGS_MAIN: &str = "flex min-w-0 flex-1 flex-col gap-6";

pub const SETTINGS_SECTION_BASE: &str = "flex flex-col rounded-lg border bg-card";

pub const SETTINGS_SECTION_HEADER: &str = "flex items-start justify-between gap-4 p-4";

/// A flush body starts with its own hairline, so the header needs less room above it.
pub const SETTINGS_SECTION_HEADER_FLUSH: &str = "pb-3";

pub const SETTINGS_SECTION_HEADING: &str = "flex min-w-0 flex-col gap-0.5";

pub const SETTINGS_SECTION_TITLE: &str = "text-h3 font-medium text-ink";

pub const SETTINGS_SECTION_DESCRIPTION: &str = "text-body text-ink-mid";

pub const SETTINGS_SECTION_ACTION: &str = "shrink-0";

pub const SETTINGS_SECTION_BODY: &str = "flex flex-col";

/// Rows run edge to edge and bring their own hairlines.
pub const SETTINGS_SECTION_BODY_FLUSH: &str = "border-t border-border-subtle";

pub const SETTINGS_SECTION_BODY_PADDED: &str = "gap-4 px-4 pb-4";

/// Keeps label and control on one line at every width, control at the end.
/// `flex-row` must win over the form row's `flex-col`, so it goes after the base.
pub const SETTINGS_ROW_INLINE: &str = "flex-row items-center justify-between sm:gap-6 [&>[data-slot=form-row-control]]:flex-none [&>[data-slot=form-row-control]]:items-end";

/// The card edge per tone. Mirrors the TS `SettingsSectionProps["tone"]`.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum SettingsSectionTone {
	#[tw(default, class = "border-border")]
	Default,
	/// The destructive zone.
	#[tw(class = "border-accent-error")]
	Danger,
}
