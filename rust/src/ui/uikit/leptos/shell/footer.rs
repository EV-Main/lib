use ev_lib_classes::{
	FOOTER_BRAND, FOOTER_COPYRIGHT, FOOTER_DESCRIPTION, FOOTER_GRID, FOOTER_HEADING, FOOTER_LEGAL_LINK, FOOTER_LEGAL_LINKS, FOOTER_LEGAL_SEPARATOR, FOOTER_LOCKUP, FOOTER_LOCKUP_TEXT,
	FOOTER_NAV_COLUMN, FOOTER_NAV_LINK, FOOTER_NAV_LIST, FOOTER_NEWSLETTER_BLURB, FOOTER_NEWSLETTER_COLUMN, FOOTER_OFFICE_NAME, FOOTER_OFFICES_LIST, FOOTER_ROOT, FOOTER_TAGLINE,
	FOOTER_VERSION, FOOTER_WIDE_COLUMN, copy,
};
pub use ev_lib_classes::{FooterLink, FooterLinkGroup, FooterOffice};
use leptos::prelude::*;

use crate::ui::uikit::leptos::{Container, Slot};

/// A 12-col footer grid — brand 3 | sitemap groups 2 each | offices 3 |
/// newsletter 2 — with the legal links and copyright line. On mobile the sitemap
/// columns sit side by side. Every string is the caller's; the `offices` and
/// `newsletter` columns are omitted when empty. `children` render right after the
/// `<footer>` tag — the slot for app-side extras like a build-version easter egg.
#[component]
pub fn Footer(
	#[prop(into)] brand: String,
	#[prop(into)] description: String,
	#[prop(into)] copyright: String,
	nav: Vec<FooterLinkGroup>,
	/// The mark beside the brand name. Sized by the caller.
	#[prop(optional, into)]
	mark: Option<ViewFn>,
	#[prop(optional, into)] tagline: Option<String>,
	#[prop(optional)] offices: Vec<FooterOffice>,
	#[prop(default = copy::FOOTER_OFFICES.to_owned(), into)] offices_heading: String,
	#[prop(optional)] legal_links: Vec<FooterLink>,
	#[prop(optional, into)] newsletter: Option<ViewFn>,
	#[prop(default = copy::FOOTER_NEWSLETTER.to_owned(), into)] newsletter_heading: String,
	#[prop(optional, into)] newsletter_blurb: String,
	#[prop(optional, into)] version: Option<String>,
	#[prop(optional, into)] commit_href: Option<String>,
	#[prop(optional)] children: Option<Children>,
) -> impl IntoView {
	let legal = legal_links
		.into_iter()
		.enumerate()
		.map(|(i, link)| {
			view! {
				{(i > 0).then(|| view! { <span class=FOOTER_LEGAL_SEPARATOR>"|"</span> })}
				<a href=link.href class=FOOTER_LEGAL_LINK>
					{link.label}
				</a>
			}
		})
		.collect_view();
	let groups = nav
		.into_iter()
		.map(|group| {
			let label = format!("Footer {} links", group.heading);
			let links = group
				.links
				.into_iter()
				.map(|link| {
					view! {
						<li>
							<a href=link.href class=FOOTER_NAV_LINK>
								{link.label}
							</a>
						</li>
					}
				})
				.collect_view();
			view! {
				<nav aria-label=label class=FOOTER_NAV_COLUMN>
					<h4 class=FOOTER_HEADING>{group.heading}</h4>
					<ul class=FOOTER_NAV_LIST>{links}</ul>
				</nav>
			}
		})
		.collect_view();
	let offices = (!offices.is_empty()).then(|| {
		let items = offices
			.into_iter()
			.map(|office| {
				view! {
					<li>
						<strong class=FOOTER_OFFICE_NAME>{office.name}</strong>
						{office.address}
					</li>
				}
			})
			.collect_view();
		view! {
			<div class=FOOTER_WIDE_COLUMN>
				<h4 class=FOOTER_HEADING>{offices_heading}</h4>
				<ul class=FOOTER_OFFICES_LIST>{items}</ul>
			</div>
		}
	});
	let newsletter = newsletter.map(|newsletter| {
		view! {
			<div class=FOOTER_NEWSLETTER_COLUMN>
				<h4 class=FOOTER_HEADING>{newsletter_heading}</h4>
				// What the source's `<Tier tier="alt"><Text>` emits: the alt body size + the info variant, then the caller's mb-4.
				<p class=FOOTER_NEWSLETTER_BLURB>{newsletter_blurb}</p>
				{newsletter.run()}
			</div>
		}
	});
	view! {
		<footer class=FOOTER_ROOT data-slot=Slot::Footer.as_str()>
			{children.map(|c| c())}
			<Container>
				<div class=FOOTER_GRID>
					<div class=FOOTER_WIDE_COLUMN>
						<div class=FOOTER_LOCKUP>
							{mark.map(|m| m.run())} <div class=FOOTER_LOCKUP_TEXT>
								<span class=FOOTER_BRAND>{brand}</span>
								{tagline
									.map(|tagline| {
										view! { <span class=FOOTER_TAGLINE>{tagline}</span> }
									})}
							</div>
						</div>
						<p class=FOOTER_DESCRIPTION>{description}</p>
						<div class=FOOTER_LEGAL_LINKS>{legal}</div>
					</div>
					{groups}
					{offices}
					{newsletter}
				</div>
				<div class=FOOTER_COPYRIGHT>
					<p>
						{copyright}
						{version
							.map(|v| {
								view! {
									" "
									<a href=commit_href class=FOOTER_VERSION>
										{v}
									</a>
								}
							})}
					</p>
				</div>
			</Container>
		</footer>
	}
}
