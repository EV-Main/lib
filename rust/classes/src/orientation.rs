/// The axis a component lays out along; rendered as `data-orientation`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Orientation {
	#[default]
	Horizontal,
	Vertical,
}
