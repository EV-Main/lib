use crate::ButtonVariant;

/// Layered over the ghost or secondary button face: a lone glyph reads quieter
/// than a label, so it rests at the mid ink and lifts on hover.
pub const ICON_BUTTON_BASE: &str = "text-ink-mid hover:text-ink";

/// The toggled-on fill. The hover repeats it, or the ghost hover would wash a
/// pressed button back to the unpressed look.
pub const ICON_BUTTON_PRESSED: &str = "bg-active text-ink hover:bg-active";

/// The two button faces a lone glyph may wear; the filled and outlined calls to
/// action need a label to say what they commit to. Mirrors the TS `IconButtonVariant`.
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum IconButtonVariant {
	Secondary,
	#[default]
	Ghost,
}

impl IconButtonVariant {
	pub fn button_variant(self) -> ButtonVariant {
		match self {
			IconButtonVariant::Secondary => ButtonVariant::Secondary,
			IconButtonVariant::Ghost => ButtonVariant::Ghost,
		}
	}
}
