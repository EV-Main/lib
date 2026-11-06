use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{SWITCH_BASE, SWITCH_THUMB, Slot, primitives::use_controllable},
};

#[component]
pub fn Switch(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] checked: MaybeProp<bool>,
	#[prop(optional)] default_checked: bool,
	#[prop(optional)] on_checked_change: Option<Callback<bool>>,
) -> impl IntoView {
	let state = use_controllable(checked, default_checked, on_checked_change);
	let data_state = move || if state.get() { "checked" } else { "unchecked" };
	let cls = cn!(SWITCH_BASE, class);
	view! {
		<button
			type="button"
			role="switch"
			class=cls
			data-slot=Slot::Switch.as_str()
			data-state=data_state
			aria-checked=move || state.get().to_string()
			disabled=disabled
			on:click=move |_| state.set(!state.get_untracked())
		>
			<span class=SWITCH_THUMB data-slot=Slot::SwitchThumb.as_str() data-state=data_state></span>
		</button>
	}
}
