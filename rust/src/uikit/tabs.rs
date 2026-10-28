use super::RovingOrientation;

/// Tab layout axis; also drives the roving-focus arrow keys.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum TabsOrientation {
	#[default]
	Horizontal,
	Vertical,
}
impl From<TabsOrientation> for RovingOrientation {
	fn from(o: TabsOrientation) -> Self {
		match o {
			TabsOrientation::Horizontal => Self::Horizontal,
			TabsOrientation::Vertical => Self::Vertical,
		}
	}
}
