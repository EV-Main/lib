/// A footer sitemap link.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(ts_rs::TS))]
pub struct FooterLink {
	pub label: String,
	pub href: String,
}

/// A footer sitemap column: a heading over its links. Each renders `lg:col-span-2`.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(ts_rs::TS))]
pub struct FooterLinkGroup {
	pub heading: String,
	#[cfg_attr(feature = "ts_gen", ts(type = "readonly FooterLink[]"))]
	pub links: Vec<FooterLink>,
}

/// An office entry in the footer's offices column.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(ts_rs::TS))]
pub struct FooterOffice {
	pub name: String,
	pub address: String,
}
pub const FOOTER_ROOT: &str = "bg-background border-t border-ink/10 py-16";
pub const FOOTER_GRID: &str = "grid grid-cols-2 gap-x-8 gap-y-8 lg:grid-cols-12 mb-12";
pub const FOOTER_WIDE_COLUMN: &str = "col-span-2 lg:col-span-3";
pub const FOOTER_LOCKUP: &str = "flex items-center gap-3 mb-6";
pub const FOOTER_LOCKUP_TEXT: &str = "flex flex-col";
pub const FOOTER_BRAND: &str = "font-serif font-bold text-base tracking-wider text-ink";
pub const FOOTER_TAGLINE: &str = "text-[8px] font-mono tracking-[0.3em] text-primary-ink uppercase";
pub const FOOTER_DESCRIPTION: &str = "text-ink/40 text-xs font-light max-w-sm leading-relaxed mb-6";
pub const FOOTER_LEGAL_LINKS: &str = "flex gap-4 text-xs font-mono text-primary-ink";
pub const FOOTER_LEGAL_SEPARATOR: &str = "text-ink/20";
pub const FOOTER_LEGAL_LINK: &str = "hover:underline";
pub const FOOTER_NAV_COLUMN: &str = "lg:col-span-2";
pub const FOOTER_HEADING: &str = "font-mono text-xs text-ink uppercase tracking-widest mb-6";
pub const FOOTER_NAV_LIST: &str = "space-y-3";
pub const FOOTER_NAV_LINK: &str = "text-xs font-light text-ink/70 hover:text-primary-ink transition-colors";
pub const FOOTER_OFFICES_LIST: &str = "space-y-4 text-xs text-ink/70 font-light leading-relaxed";
pub const FOOTER_OFFICE_NAME: &str = "text-ink block font-mono text-[10px] uppercase tracking-wider mb-1";
pub const FOOTER_NEWSLETTER_COLUMN: &str = "col-span-2 lg:col-span-2";
pub const FOOTER_NEWSLETTER_BLURB: &str = "text-sm sm:text-xs font-light leading-relaxed text-ink/70 mb-4";
pub const FOOTER_COPYRIGHT: &str = "border-t border-ink/10 pt-8 text-[10px] font-mono text-ink/40";
pub const FOOTER_VERSION: &str = "text-ink/30";
