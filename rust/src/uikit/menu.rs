use ev_lib_classes::NAVIGATION_MENU_TRIGGER_STYLE;

/// Neutral vs destructive item styling; mirrors the TS `variant` union.
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum DropdownMenuItemVariant {
	#[default]
	Neutral,
	Destructive,
}

/// Neutral vs destructive item styling; mirrors the TS `variant` union.
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum ContextMenuItemVariant {
	#[default]
	Neutral,
	Destructive,
}

/// Item visual tone — the canonical superset mirrors the TS `variant` prop.
#[derive(Clone, Copy, Debug, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum MenubarItemVariant {
	#[default]
	Neutral,
	Destructive,
}

/// Canonical class for a navigation-menu trigger, the mirror of the TS
/// `navigationMenuTriggerStyle` helper so callers can style a plain link the
/// same way without rendering a `NavigationMenuTrigger`.
pub fn navigation_menu_trigger_style() -> &'static str {
	NAVIGATION_MENU_TRIGGER_STYLE
}
