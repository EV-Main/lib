use tailwind_fuse::{AsTailwindClass, TwVariant};

/// The kit's one control scale. A step sets `--control-box` and `--control-pad`
/// to its rung of the `--control-h-*` and `--control-inset-*` ramps (theme.css)
/// and the text size that goes with it; a component reads them on whatever axis
/// it sizes and draws its glyph off the text, so a step nobody else has is added here rather than as an enum of its own.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Size {
	#[tw(class = "[--control-box:var(--control-h-xs)] [--control-pad:var(--control-inset-xs)] text-xs")]
	Xs,
	#[tw(class = "[--control-box:var(--control-h-sm)] [--control-pad:var(--control-inset-sm)] text-sm")]
	Sm,
	#[tw(default, class = "[--control-box:var(--control-h-md)] [--control-pad:var(--control-inset-md)] text-sm")]
	Md,
	#[tw(class = "[--control-box:var(--control-h-lg)] [--control-pad:var(--control-inset-lg)] text-base")]
	Lg,
	#[tw(class = "[--control-box:var(--control-h-xl)] [--control-pad:var(--control-inset-xl)] text-[length:var(--control-text)]")]
	Xl,
	/// The 44 px pointer-target floor (WCAG 2.5.5) for touch-first controls. Named
	/// for its purpose rather than its magnitude because the floor is the contract.
	#[tw(class = "[--control-box:var(--control-h-touch)] [--control-pad:var(--control-inset-touch)] text-base")]
	Touch,
}
