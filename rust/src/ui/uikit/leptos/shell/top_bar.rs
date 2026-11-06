use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{TOP_BAR_BASE, TOP_BAR_END, TOP_BAR_START},
};

/// The wide screen's utility bar over the content column: who and where, never
/// product nav (that is the rail's). Mirrors the TS `TopBar`. Layout only.
#[component]
pub fn TopBar(
	/// The app switcher, or the service's name.
	#[prop(optional, into)]
	start: Option<ViewFn>,
	/// Service stats, then an account menu last.
	#[prop(optional, into)]
	end: Option<ViewFn>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let cls = cn!(TOP_BAR_BASE, class);
	view! {
		<header class=cls data-slot="top-bar">
			<div class=TOP_BAR_START data-slot="top-bar-start">
				{start.map(|s| s.run())}
			</div>
			<div class=TOP_BAR_END data-slot="top-bar-end">
				{end.map(|e| e.run())}
			</div>
		</header>
	}
}
