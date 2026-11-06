use ev_lib_classes::Icon;
use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{
		CHECKBOX_BASE, CHECKBOX_INDICATOR,
		primitives::{icon_shapes, use_controllable},
	},
};

#[component]
pub fn Checkbox(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] checked: MaybeProp<bool>,
	#[prop(optional)] default_checked: bool,
	#[prop(optional)] on_checked_change: Option<Callback<bool>>,
) -> impl IntoView {
	let state = use_controllable(checked, default_checked, on_checked_change);
	let cls = cn!(CHECKBOX_BASE, class);
	view! {
		<button
			type="button"
			role="checkbox"
			class=cls
			data-slot="checkbox"
			data-state=move || if state.get() { "checked" } else { "unchecked" }
			aria-checked=move || state.get().to_string()
			disabled=disabled
			on:click=move |_| state.set(!state.get_untracked())
		>
			{move || {
				state
					.get()
					.then(|| {
						view! {
							<span class=CHECKBOX_INDICATOR data-slot="checkbox-indicator">
								<svg
									xmlns="http://www.w3.org/2000/svg"
									viewBox="0 0 24 24"
									fill="none"
									stroke="currentColor"
									stroke-width="2"
									stroke-linecap="round"
									stroke-linejoin="round"
									class="size-3.5"
									aria-hidden="true"
								>
									{icon_shapes(Icon::Check)}
								</svg>
							</span>
						}
					})
			}}
		</button>
	}
}
