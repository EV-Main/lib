use leptos::prelude::*;

use crate::uikit_leptos::{Size, ToggleVariant, primitives::use_controllable, toggle_classes};

#[component]
pub fn Toggle(
	#[prop(optional)] variant: ToggleVariant,
	#[prop(optional)] size: Size,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] pressed: MaybeProp<bool>,
	#[prop(optional)] default_pressed: bool,
	#[prop(optional)] on_pressed_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(pressed, default_pressed, on_pressed_change);
	let cls = toggle_classes(&variant, size, &class);
	view! {
		<button
			type="button"
			class=cls
			data-slot="toggle"
			data-state=move || if state.get() { "on" } else { "off" }
			aria-pressed=move || state.get().to_string()
			disabled=disabled
			on:click=move |_| state.set(!state.get_untracked())
		>
			{children()}
		</button>
	}
}
