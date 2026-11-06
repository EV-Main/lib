use ev_lib_classes::NAVIGATION_MENU_TRIGGER_STYLE;

/// Canonical class for a navigation-menu trigger, the mirror of the TS
/// `navigationMenuTriggerStyle` helper so callers can style a plain link the
/// same way without rendering a `NavigationMenuTrigger`.
pub fn navigation_menu_trigger_style() -> &'static str {
	NAVIGATION_MENU_TRIGGER_STYLE
}
