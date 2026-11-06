use crate::{ButtonVariant, Size};

/// Layered over the ghost or outline button face: a lone glyph reads quieter
/// than a label, so it rests at the mid ink and lifts on hover.
pub const ICON_BUTTON_BASE: &str = "text-ink-mid hover:text-ink";

/// The toggled-on fill. The hover repeats it, or the ghost hover would wash a
/// pressed button back to the unpressed look.
pub const ICON_BUTTON_PRESSED: &str = "bg-active text-ink hover:bg-active";

/// The two button faces a lone glyph may wear; a filled call to action needs a
/// label to say what it commits to. Mirrors the TS `IconButtonVariant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum IconButtonVariant {
	Outline,
	#[default]
	Ghost,
}

impl IconButtonVariant {
	pub fn button_variant(self) -> ButtonVariant {
		match self {
			IconButtonVariant::Outline => ButtonVariant::Outline,
			IconButtonVariant::Ghost => ButtonVariant::Ghost,
		}
	}
}

/// The square's edge, fused over the button's `--control-box`: a lone glyph sits
/// two steps under its labelled sibling, so `md` is the `xs` rung and `sm` one step
/// under it. `touch` keeps the floor; the TS `IconButtonSize` narrows to these three.
pub fn icon_button_size_class(size: Size) -> &'static str {
	match size {
		Size::Sm => "[--control-box:calc(var(--control-h-xs)_-_var(--control-step))]",
		Size::Md => "[--control-box:var(--control-h-xs)]",
		Size::Xs | Size::Lg | Size::Xl | Size::Touch => "",
	}
}
