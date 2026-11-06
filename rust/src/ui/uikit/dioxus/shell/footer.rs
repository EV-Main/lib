use dioxus::prelude::*;
use ev_lib_classes::{
	FOOTER_BRAND, FOOTER_COPYRIGHT, FOOTER_DESCRIPTION, FOOTER_GRID, FOOTER_HEADING, FOOTER_LEGAL_LINK, FOOTER_LEGAL_LINKS, FOOTER_LEGAL_SEPARATOR, FOOTER_LOCKUP, FOOTER_LOCKUP_TEXT,
	FOOTER_NAV_COLUMN, FOOTER_NAV_LINK, FOOTER_NAV_LIST, FOOTER_NEWSLETTER_BLURB, FOOTER_NEWSLETTER_COLUMN, FOOTER_OFFICE_NAME, FOOTER_OFFICES_LIST, FOOTER_ROOT, FOOTER_TAGLINE,
	FOOTER_VERSION, FOOTER_WIDE_COLUMN, copy,
};
pub use ev_lib_classes::{FooterLink, FooterLinkGroup, FooterOffice};

use crate::ui::uikit::dioxus::Container;

/// A 12-col footer grid — brand 3 | sitemap groups 2 each | offices 3 |
/// newsletter 2 — with the legal links and copyright line. On mobile the sitemap
/// columns sit side by side. Every string is the caller's; the `offices` and
/// `newsletter` columns are omitted when empty. `children` render right after the
/// `<footer>` tag — the slot for app-side extras like a build-version easter egg.
#[component]
pub fn Footer(
	brand: String,
	description: String,
	copyright: String,
	nav: Vec<FooterLinkGroup>,
	/// The mark beside the brand name. Sized by the caller.
	mark: Option<Element>,
	tagline: Option<String>,
	#[props(default)] offices: Vec<FooterOffice>,
	#[props(default = copy::FOOTER_OFFICES.to_owned())] offices_heading: String,
	#[props(default)] legal_links: Vec<FooterLink>,
	newsletter: Option<Element>,
	#[props(default = copy::FOOTER_NEWSLETTER.to_owned())] newsletter_heading: String,
	#[props(default)] newsletter_blurb: String,
	version: Option<String>,
	commit_href: Option<String>,
	children: Element,
) -> Element {
	rsx! {
		footer { class: FOOTER_ROOT, "data-slot": "footer",
			{children}
			Container {
				div { class: FOOTER_GRID,
					div { class: FOOTER_WIDE_COLUMN,
						div { class: FOOTER_LOCKUP,
							{mark}
							div { class: FOOTER_LOCKUP_TEXT,
								span { class: FOOTER_BRAND, {brand} }
								if let Some(tagline) = tagline {
									span { class: FOOTER_TAGLINE, {tagline} }
								}
							}
						}
						p { class: FOOTER_DESCRIPTION, {description} }
						div { class: FOOTER_LEGAL_LINKS,
							for (i , link) in legal_links.iter().enumerate() {
								if i > 0 {
									span { class: FOOTER_LEGAL_SEPARATOR, "|" }
								}
								a { href: link.href.clone(), class: FOOTER_LEGAL_LINK, {link.label.clone()} }
							}
						}
					}

					for group in nav.iter() {
						nav {
							key: "{group.heading}",
							aria_label: "Footer {group.heading} links",
							class: FOOTER_NAV_COLUMN,
							h4 { class: FOOTER_HEADING, {group.heading.clone()} }
							ul { class: FOOTER_NAV_LIST,
								for link in group.links.iter() {
									li { key: "{link.href}",
										a {
											href: link.href.clone(),
											class: FOOTER_NAV_LINK,
											{link.label.clone()}
										}
									}
								}
							}
						}
					}

					if !offices.is_empty() {
						div { class: FOOTER_WIDE_COLUMN,
							h4 { class: FOOTER_HEADING, {offices_heading} }
							ul { class: FOOTER_OFFICES_LIST,
								for office in offices.iter() {
									li { key: "{office.name}",
										strong { class: FOOTER_OFFICE_NAME, {office.name.clone()} }
										{office.address.clone()}
									}
								}
							}
						}
					}

					if newsletter.is_some() {
						div { class: FOOTER_NEWSLETTER_COLUMN,
							h4 { class: FOOTER_HEADING, {newsletter_heading} }
							// What the source's `<Tier tier="alt"><Text>` emits: the alt body
							// size + the info variant, then the mb-4 the caller adds.
							p { class: FOOTER_NEWSLETTER_BLURB, {newsletter_blurb} }
							{newsletter}
						}
					}
				}

				div { class: FOOTER_COPYRIGHT,
					p {
						{copyright}
						if let Some(v) = version {
							" "
							a { href: commit_href, class: FOOTER_VERSION, {v} }
						}
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

	fn nav() -> Vec<FooterLinkGroup> {
		vec![FooterLinkGroup {
			heading: "Company".to_string(),
			links: vec![FooterLink {
				label: "Team".to_string(),
				href: "/team".to_string(),
			}],
		}]
	}

	#[test]
	fn renders_the_callers_copy_and_omits_the_empty_columns() {
		fn app() -> Element {
			rsx! {
				Footer {
					brand: "ACME",
					description: "We do a thing.",
					copyright: "© 2026 ACME",
					nav: nav(),
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"footer\""), "{html}");
		assert!(html.contains("ACME"), "{html}");
		assert!(html.contains("aria-label=\"Footer Company links\""), "{html}");
		assert!(html.contains("href=\"/team\""), "{html}");
		assert!(html.contains("© 2026 ACME"), "{html}");
		assert!(!html.contains("Offices"), "no offices column without offices: {html}");
		assert!(!html.contains("Newsletter"), "no newsletter column without the slot: {html}");
		assert!(!html.contains("text-ink/30"), "no version link without version: {html}");
	}

	#[test]
	fn offices_and_newsletter_columns_render_with_their_data() {
		fn app() -> Element {
			rsx! {
				Footer {
					brand: "ACME",
					description: "We do a thing.",
					copyright: "© 2026 ACME",
					nav: nav(),
					offices: vec![
						FooterOffice {
							name: "Head Office".to_string(),
							address: "1 Main St".to_string(),
						},
					],
					newsletter_blurb: "Subscribe.",
					newsletter: rsx! {
						form { input {} }
					},
				}
			}
		}
		let html = render(app);
		assert!(html.contains("Head Office"), "{html}");
		assert!(html.contains("1 Main St"), "{html}");
		assert!(html.contains("Newsletter"), "{html}");
		assert!(html.contains("Subscribe."), "{html}");
		assert!(html.contains("<form"), "{html}");
	}

	#[test]
	fn version_link_renders_when_given() {
		fn app() -> Element {
			rsx! {
				Footer {
					brand: "ACME",
					description: "We do a thing.",
					copyright: "© 2026 ACME",
					nav: nav(),
					version: "v1.2.3",
					commit_href: "https://example.com/commit/abc",
				}
			}
		}
		let html = render(app);
		assert!(html.contains("v1.2.3"), "{html}");
		assert!(html.contains("https://example.com/commit/abc"), "{html}");
		assert!(html.contains("text-ink/30"), "{html}");
	}
}
