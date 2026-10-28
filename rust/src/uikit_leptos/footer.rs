use leptos::prelude::*;

pub use crate::uikit::{FooterLink, FooterLinkGroup, FooterOffice};
use crate::uikit_leptos::Container;

const HEADING: &str = "font-mono text-xs text-ink uppercase tracking-widest mb-6";

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
	#[prop(default = "Offices".to_string(), into)] offices_heading: String,
	#[prop(optional)] legal_links: Vec<FooterLink>,
	#[prop(optional, into)] newsletter: Option<ViewFn>,
	#[prop(default = "Newsletter".to_string(), into)] newsletter_heading: String,
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
				{(i > 0).then(|| view! { <span class="text-ink/20">"|"</span> })}
				<a href=link.href class="hover:underline">
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
							<a
								href=link.href
								class="text-xs font-light text-ink/70 hover:text-accent-debug transition-colors"
							>
								{link.label}
							</a>
						</li>
					}
				})
				.collect_view();
			view! {
				<nav aria-label=label class="lg:col-span-2">
					<h4 class=HEADING>{group.heading}</h4>
					<ul class="space-y-3">{links}</ul>
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
						<strong class="text-ink block font-mono text-[10px] uppercase tracking-wider mb-1">
							{office.name}
						</strong>
						{office.address}
					</li>
				}
			})
			.collect_view();
		view! {
			<div class="col-span-2 lg:col-span-3">
				<h4 class=HEADING>{offices_heading}</h4>
				<ul class="space-y-4 text-xs text-ink/70 font-light leading-relaxed">{items}</ul>
			</div>
		}
	});
	let newsletter = newsletter.map(|newsletter| {
		view! {
			<div class="col-span-2 lg:col-span-2">
				<h4 class=HEADING>{newsletter_heading}</h4>
				// What the source's `<Tier tier="alt"><Text>` emits: the alt body size + the info variant, then the caller's mb-4.
				<p class="text-sm sm:text-xs font-light leading-relaxed text-ink/70 mb-4">
					{newsletter_blurb}
				</p>
				{newsletter.run()}
			</div>
		}
	});
	view! {
		<footer class="bg-background border-t border-ink/10 py-16" data-slot="footer">
			{children.map(|c| c())}
			<Container>
				<div class="grid grid-cols-2 gap-x-8 gap-y-8 lg:grid-cols-12 mb-12">
					<div class="col-span-2 lg:col-span-3">
						<div class="flex items-center gap-3 mb-6">
							{mark.map(|m| m.run())} <div class="flex flex-col">
								<span class="font-serif font-bold text-base tracking-wider text-ink">{brand}</span>
								{tagline
									.map(|tagline| {
										view! {
											<span class="text-[8px] font-mono tracking-[0.3em] text-accent-debug uppercase">
												{tagline}
											</span>
										}
									})}
							</div>
						</div>
						<p class="text-ink/40 text-xs font-light max-w-sm leading-relaxed mb-6">
							{description}
						</p>
						<div class="flex gap-4 text-xs font-mono text-accent-debug">{legal}</div>
					</div>
					{groups}
					{offices}
					{newsletter}
				</div>
				<div class="border-t border-ink/10 pt-8 text-[10px] font-mono text-ink/40">
					<p>
						{copyright}
						{version
							.map(|v| {
								view! {
									" "
									<a href=commit_href class="text-ink/30">
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
