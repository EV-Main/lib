use tailwind_fuse::{AsTailwindClass, TwVariant};

/// The kit's one control scale. A step sets `--control-box` to its rung of the
/// `--control-h-*` ramp (theme.css) and the text size that goes with it; a
/// component reads the box on whatever axis it sizes and draws its glyph off the
/// text, so a step nobody else has is added here rather than as an enum of its own.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Size {
	#[tw(class = "[--control-box:var(--control-h-xs)] text-xs")]
	Xs,
	#[tw(class = "[--control-box:var(--control-h-sm)] text-sm")]
	Sm,
	#[tw(default, class = "[--control-box:var(--control-h-md)] text-sm")]
	Md,
	#[tw(class = "[--control-box:var(--control-h-lg)] text-base")]
	Lg,
	#[tw(class = "[--control-box:var(--control-h-xl)] text-base")]
	Xl,
	/// The 44 px pointer-target floor (WCAG 2.5.5) for touch-first controls. Named
	/// for its purpose rather than its magnitude because the floor is the contract.
	#[tw(class = "[--control-box:var(--control-h-touch)] text-base")]
	Touch,
}
