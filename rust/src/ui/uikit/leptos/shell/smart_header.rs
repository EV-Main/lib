pub use ev_lib_classes::SmartHeaderState;
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{SMART_HEADER_BAR, SMART_HEADER_BASE, SMART_HEADER_CONTEXT, SMART_HEADER_TOOLBAR, TopBar},
};

/// A page header that adapts to scroll and alerts. Mirrors the TS
/// `SmartHeader`.
///
/// The bar is the same in every state, so switching never moves the brand or
/// the account: only the rows around it come and go. Layout only: the caller
/// owns the state and what changes it (a scroll observer, a feed status, the
/// search button), so the kit needs no scroll listener.
#[component]
pub fn SmartHeader(
	#[prop(optional)] state: SmartHeaderState,
	/// Brand, workspace tabs: the left side of the bar.
	#[prop(optional, into)]
	start: Option<ViewFn>,
	/// Icon actions, balance, bell, account menu: the right side of the bar.
	#[prop(optional, into)]
	end: Option<ViewFn>,
	/// The folded toolbar after `start`; shown in `Collapsed` only.
	#[prop(optional, into)]
	collapsed: Option<ViewFn>,
	/// A `SystemBanner`; shown in `Banner` only.
	#[prop(optional, into)]
	banner: Option<ViewFn>,
	/// The second row; shown in `Default` only.
	#[prop(optional, into)]
	toolbar: Option<ViewFn>,
	/// The slim line on the page fill under the toolbar; shown in `Default` only.
	#[prop(optional, into)]
	context: Option<ViewFn>,
	/// A `CommandPalette` or another panel that drops from the bar; shown in
	/// `Search` only.
	#[prop(optional, into)]
	palette: Option<ViewFn>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let cls = cn!(SMART_HEADER_BASE, class);
	let banner = banner.filter(|_| state == SmartHeaderState::Banner);
	let collapsed = collapsed.filter(|_| state == SmartHeaderState::Collapsed);
	let toolbar = toolbar.filter(|_| state == SmartHeaderState::Default);
	let context = context.filter(|_| state == SmartHeaderState::Default);
	let palette = palette.filter(|_| state == SmartHeaderState::Search);
	let bar_start = move || {
		view! {
			{start.as_ref().map(ViewFn::run)}
			{collapsed.as_ref().map(ViewFn::run)}
		}
	};
	view! {
		<div class=cls data-slot="smart-header" data-state=state.as_ref().to_owned()>
			{banner.map(|b| b.run())}
			<TopBar class=SMART_HEADER_BAR start=bar_start end=move || end.as_ref().map(ViewFn::run) />
			{toolbar
				.map(|t| {
					view! {
						<div class=SMART_HEADER_TOOLBAR data-slot="smart-header-toolbar">
							{t.run()}
						</div>
					}
				})}
			{context
				.map(|c| {
					view! {
						<div class=SMART_HEADER_CONTEXT data-slot="smart-header-context">
							{c.run()}
						</div>
					}
				})}
			{palette.map(|p| view! { <div data-slot="smart-header-palette">{p.run()}</div> })}
		</div>
	}
}
