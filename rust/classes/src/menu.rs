/// Neutral vs destructive item styling, shared by every menu's items.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr, strum::IntoStaticStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum MenuItemVariant {
	#[default]
	Neutral,
	Destructive,
}
