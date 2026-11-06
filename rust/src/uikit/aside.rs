/// The `AsideHeader`'s own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AsideHeaderLabels {
	/// `aria-label` of the back button; "Back".
	pub back: Option<String>,
	/// `aria-label` of the close button; "Close".
	pub close: Option<String>,
}

/// How an `AsidePanel` takes its place. Mirrors the TS `AsidePanelMode`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AsidePanelMode {
	/// Sits in the layout and takes room from the content.
	#[default]
	Docked,
	/// Floats over a scrim on a `Sheet`.
	Overlay,
}

/// Where an `AsidePanel` gives up its wide form for a bottom `Drawer`.
/// The TS `collapseBelow` takes `"md" | false`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AsideCollapse {
	/// Below the `md` breakpoint (768 px).
	#[default]
	Md,
	/// At no width: for a framed preview or an embedded layout, where the
	/// viewport says nothing about the room.
	Never,
}
