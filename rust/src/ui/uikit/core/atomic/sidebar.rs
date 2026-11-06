use ev_lib_classes::SidebarCollapsible;

/// The width variables a `SidebarProvider` sets for its parts.
pub(crate) const SIDEBAR_STYLE: &str = "--sidebar-width: 16rem; --sidebar-width-icon: 3rem;";

/// `data-collapsible`: how the sidebar collapsed, empty while it is open.
pub(crate) fn sidebar_collapsible_data(collapsible: SidebarCollapsible, open: bool) -> &'static str {
	if open { "" } else { collapsible.into() }
}

/// `data-state` of a sidebar that is `open` or not.
pub(crate) fn sidebar_state(open: bool) -> &'static str {
	if open { "expanded" } else { "collapsed" }
}
