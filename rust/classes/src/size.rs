/// The kit's one control scale. Every sized component reads it, so a component
/// that needs a step nobody else has adds it here rather than growing an enum of
/// its own.
#[derive(Clone, Copy, Default, PartialEq, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Size {
	/// 20 px
	Xs,
	/// 24 px
	Sm,
	/// 28 px
	#[default]
	Md,
	/// 32 px
	Lg,
	Xl,
	/// The 44 px pointer-target floor (WCAG 2.5.5) for touch-first controls. Named
	/// for its purpose rather than its magnitude because the floor is the contract:
	/// it sits between `Lg` and `Xl`, and a field already clears it at `Lg`.
	Touch,
}

impl Size {
	/// Tailwind magnitude; the component picks the axis (h-/size-/min-w-).
	pub fn scale(&self) -> u8 {
		match self {
			Size::Xs => 5,
			Size::Sm => 6,
			Size::Md => 7,
			Size::Lg => 8,
			Size::Xl => 12,
			Size::Touch => 11,
		}
	}
}
