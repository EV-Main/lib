/// A footer sitemap link.
#[derive(Clone, Debug, PartialEq)]
pub struct FooterLink {
	pub label: String,
	pub href: String,
}

/// A footer sitemap column: a heading over its links. Each renders `lg:col-span-2`.
#[derive(Clone, Debug, PartialEq)]
pub struct FooterLinkGroup {
	pub heading: String,
	pub links: Vec<FooterLink>,
}

/// An office entry in the footer's offices column.
#[derive(Clone, Debug, PartialEq)]
pub struct FooterOffice {
	pub name: String,
	pub address: String,
}
