/// The width variables a `SidebarProvider` sets for its parts.
pub(crate) const SIDEBAR_STYLE: &str = "--sidebar-width: 16rem; --sidebar-width-icon: 3rem;";

#[derive(Clone, Debug, Default, Eq, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SidebarSide {
	#[default]
	Left,
	Right,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SidebarVariant {
	#[default]
	Sidebar,
	Floating,
	Inset,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SidebarCollapsible {
	#[default]
	Offcanvas,
	Icon,
	None,
}
impl SidebarCollapsible {
	/// `data-collapsible`: how the sidebar collapsed, empty while it is open.
	pub(crate) fn data(&self, open: bool) -> &str {
		if open { "" } else { self.as_ref() }
	}
}

/// `data-state` of a sidebar that is `open` or not.
pub(crate) fn sidebar_state(open: bool) -> &'static str {
	if open { "expanded" } else { "collapsed" }
}
