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
