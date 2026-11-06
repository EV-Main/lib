use ev_lib_classes::{SIDEBAR_WIDTH, SIDEBAR_WIDTH_ICON, SidebarCollapsible};

/// The width variables a `SidebarProvider` sets for its parts.
pub(crate) fn sidebar_style() -> String {
	format!("--sidebar-width: {SIDEBAR_WIDTH}; --sidebar-width-icon: {SIDEBAR_WIDTH_ICON};")
}

/// `data-collapsible`: how the sidebar collapsed, empty while it is open.
pub(crate) fn sidebar_collapsible_data(collapsible: SidebarCollapsible, open: bool) -> &'static str {
	if open { "" } else { collapsible.into() }
}

/// `data-state` of a sidebar that is `open` or not.
pub(crate) fn sidebar_state(open: bool) -> &'static str {
	if open { "expanded" } else { "collapsed" }
}
