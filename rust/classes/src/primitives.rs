use crate::Orientation;

/// Which edge of its anchor an overlay is placed against. Rendered as a
/// `data-side` attribute so CSS positions and animates the overlay; the kit
/// does not measure the viewport (the TS `useFloating` does).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Side {
	Top,
	Right,
	#[default]
	Bottom,
	Left,
}

/// Which arrow keys walk a roving-focus group. `Home`/`End` always do.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum RovingOrientation {
	Horizontal,
	#[default]
	Vertical,
	Both,
}
impl From<Orientation> for RovingOrientation {
	fn from(o: Orientation) -> Self {
		match o {
			Orientation::Horizontal => Self::Horizontal,
			Orientation::Vertical => Self::Vertical,
		}
	}
}
