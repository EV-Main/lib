pub const MENU_RADIO_DOT: &str = "size-2 fill-current";
/// Neutral vs destructive item styling, shared by every menu's items.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr, strum::EnumIter, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum MenuItemVariant {
	#[default]
	Neutral,
	Destructive,
}
