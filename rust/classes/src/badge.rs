use tailwind_fuse::{AsTailwindClass, TwVariant};

use crate::focus::filled_focus_ring;

/// A 20 px chip. The fill and the edge come from the `--chip-*` tokens, so a
/// theme chooses between an outline (Dark, Light) and a tinted pill (Oldschool)
/// without the class table knowing.
///
/// A badge only takes focus as a link; it then wears the offset ring.
pub const BADGE_BASE: &str = concat!(
	"inline-flex h-5 items-center justify-center rounded-[var(--chip-radius)] border px-1.5 text-caption \
	 font-medium w-fit whitespace-nowrap shrink-0 gap-1 overflow-hidden \
	 [&>svg]:size-3 [&>svg]:pointer-events-none transition-colors outline-none \
	 aria-invalid:border-accent-error ",
	filled_focus_ring!()
);

/// The kit's one severity vocabulary (docs/spec/variants.md) plus `Primary`:
/// Figma's accent, negative and warning map to `Primary`, `Error` and `Warn`.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum BadgeVariant {
	#[tw(default, class = "border-chip-neutral-border bg-chip-neutral-bg text-ink-mid [a&]:hover:bg-active")]
	Neutral,
	#[tw(class = "border-chip-primary-border bg-chip-primary-bg text-primary-ink [a&]:hover:bg-hover")]
	Primary,
	#[tw(class = "border-chip-positive-border bg-chip-positive-bg text-positive-ink [a&]:hover:bg-hover")]
	Positive,
	#[tw(class = "border-chip-info-border bg-chip-info-bg text-accent-info-ink [a&]:hover:bg-hover")]
	Info,
	#[tw(class = "border-chip-warn-border bg-chip-warn-bg text-accent-warn-ink [a&]:hover:bg-hover")]
	Warn,
	#[tw(class = "border-chip-error-border bg-chip-error-bg text-accent-error-ink [a&]:hover:bg-hover")]
	Error,
}
