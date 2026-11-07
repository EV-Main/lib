use ev_lib_classes::{Size, TOGGLE_BASE, ToggleVariant};

use crate::cn;

/// Fuses the base, variant and size classes with a caller override, last wins.
/// Mirrors the TS `toggleVariants` helper so `toggle-group` can reuse the same
/// canonical class string.
pub fn toggle_classes(variant: &ToggleVariant, size: Size, class: &str) -> String {
	cn!(TOGGLE_BASE, variant.as_class(), size.as_class(), class)
}
