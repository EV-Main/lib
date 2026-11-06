use ev_lib_classes::{SHEET_SIDE_BOTTOM, SHEET_SIDE_LEFT, SHEET_SIDE_RIGHT, SHEET_SIDE_TOP};

use super::Side;

/// Per-side slide-in/positioning classes, canonical with the TS `sheetSideClasses`.
pub(crate) fn sheet_side_class(side: Side) -> &'static str {
	match side {
		Side::Right => SHEET_SIDE_RIGHT,
		Side::Left => SHEET_SIDE_LEFT,
		Side::Top => SHEET_SIDE_TOP,
		Side::Bottom => SHEET_SIDE_BOTTOM,
	}
}
