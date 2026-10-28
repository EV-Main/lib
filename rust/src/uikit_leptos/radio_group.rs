use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit_leptos::{
		RADIO_GROUP_ITEM, RADIO_GROUP_ROOT,
		primitives::{Controllable, RovingFocus, RovingOrientation, use_controllable, use_roving_focus, use_roving_item},
	},
};

#[component]
pub fn RadioGroup(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	children: Children,
) -> impl IntoView {
	let value = use_controllable(value, default_value, on_value_change);
	let roving = use_roving_focus(RovingOrientation::Vertical);
	let cls = cn!(RADIO_GROUP_ROOT, class);
	view! {
		<div
			class=cls
			data-slot="radio-group"
			role="radiogroup"
			// The arrows both move and check, as native radios do — which is
			// what the `role="radio"` contract promises ("1 of 3, use arrow
			// keys") and what keeps the group's lone tab stop under focus.
			on:keydown=move |e| {
				if let Some(next) = roving.next(&e, &value.get_untracked()) {
					e.prevent_default();
					value.set(next.clone());
					roving.focus(&next);
				}
			}
		>
			<Provider value=RadioGroupContext { value }>
				<Provider value=roving>{children()}</Provider>
			</Provider>
		</div>
	}
}

#[component]
pub fn RadioGroupItem(
	#[prop(into)] value: String,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional)] children: Option<Children>,
) -> impl IntoView {
	let ctx = expect_context::<RadioGroupContext>();
	let roving = expect_context::<RovingFocus>();
	let node = NodeRef::<leptos::html::Button>::new();
	use_roving_item(roving, Signal::stored(value.clone()), node);
	let key = StoredValue::new(value);
	let checked = Memo::new(move |_| key.with_value(|k| ctx.value.get() == *k));
	let cls = cn!(RADIO_GROUP_ITEM, class);
	view! {
		<button
			type="button"
			class=cls
			data-slot="radio-group-item"
			data-state=move || if checked.get() { "checked" } else { "unchecked" }
			role="radio"
			aria-checked=move || checked.get().to_string()
			disabled=disabled
			tabindex=move || {
				if key.with_value(|k| roving.is_tab_stop(k, &ctx.value.get())) { "0" } else { "-1" }
			}
			node_ref=node
			on:click=move |_| ctx.value.set(key.get_value())
		>
			{move || {
				checked
					.get()
					.then(|| {
						view! {
							<span
								data-slot="radio-group-indicator"
								class="relative flex items-center justify-center"
							>
								<svg
									class="fill-primary-ink absolute top-1/2 left-1/2 size-2 -translate-x-1/2 -translate-y-1/2"
									viewBox="0 0 24 24"
								>
									<circle cx="12" cy="12" r="10"></circle>
								</svg>
							</span>
						}
					})
			}}
			{children.map(|c| c())}
		</button>
	}
}

/// Shared state for a [`RadioGroup`], provided to descendant [`RadioGroupItem`]s
/// via context — the mirror of the TS React context the radio group publishes.
#[derive(Clone, Copy)]
struct RadioGroupContext {
	value: Controllable<String>,
}
