use ev_lib_classes::{Accent, BUTTON_BASE, ButtonVariant, Size, button_accent_class, button_size_class, cn};

/// Fuses the base, variant, size and accent classes with a caller override, last
/// wins. Mirrors the TS `buttonVariants` helper so consumers (e.g. pagination)
/// can reuse the same canonical class string without rendering a `Button`.
///
/// The accent fuses after the variant so it recolours whatever the variant set.
pub fn button_classes(variant: &ButtonVariant, size: Size, icon: bool, accent: Option<Accent>, class: &str) -> String {
	cn!(
		BUTTON_BASE,
		variant.as_class(),
		button_size_class(size, icon),
		accent.map_or("", |a| button_accent_class(a, variant)),
		class
	)
}
