use ev_lib_classes::{Accent, ButtonVariant, Size, cn};

use super::button_classes;

/// One CTA in a `StatusScreen`'s action row.
#[derive(Clone, Debug, PartialEq)]
pub struct StatusLinkData {
	pub label: String,
	pub href: String,
	pub variant: ButtonVariant,
	pub leading_arrow: bool,
}

/// A status CTA is a `Button` at the page's accent: the canonical button string
/// for that variant and rung, then the mono/uppercase treatment the error pages
/// wear.
///
/// Public because a host that renders its own action into `StatusScreen`'s slot
/// — a retry button wired to a framework's `reset`, say — has to be able to
/// match the CTAs beside it.
pub fn status_cta_class(accent: Accent, variant: ButtonVariant) -> String {
	cn!(button_classes(&variant, Size::Lg, false, Some(accent), ""), "font-mono text-xs uppercase tracking-widest")
}
