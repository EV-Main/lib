use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{PROGRESS_INDICATOR, PROGRESS_TRACK, Slot},
};

#[component]
pub fn Progress(#[prop(optional, into)] value: Signal<f64>, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(PROGRESS_TRACK, class);
	view! {
		<div
			class=cls
			data-slot=Slot::Progress.as_str()
			role="progressbar"
			aria-valuenow=move || value.get()
			aria-valuemin=0
			aria-valuemax=100
		>
			<div
				class=PROGRESS_INDICATOR
				data-slot=Slot::ProgressIndicator.as_str()
				style=move || format!("transform: translateX(-{}%)", 100.0 - value.get())
			></div>
		</div>
	}
}
