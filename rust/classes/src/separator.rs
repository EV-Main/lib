use crate::Orientation;

pub const SEPARATOR_BASE: &str = "bg-border shrink-0";

pub fn separator_orientation_class(orientation: Orientation) -> &'static str {
	match orientation {
		Orientation::Horizontal => "data-[orientation=horizontal]:h-px data-[orientation=horizontal]:w-full",
		Orientation::Vertical => "data-[orientation=vertical]:h-full data-[orientation=vertical]:w-px",
	}
}
