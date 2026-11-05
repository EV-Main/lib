use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{FormControlContext, INPUT_BASE, Size, input_size_class},
};

/// Picks up the id and `aria-*` of an enclosing `FormControl`; outside one, all
/// three are absent.
///
/// `name` and `required` are what make this submit inside a plain
/// `<form method="post">`, with no hydration and no handler.
#[component]
pub fn Input(
	#[prop(optional)] size: Size,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] r#type: String,
	#[prop(optional, into)] placeholder: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] value: Signal<String>,
	#[prop(optional, into)] name: Option<String>,
	#[prop(optional)] required: bool,
	#[prop(optional)] on_input: Option<Callback<leptos::ev::Event>>,
) -> impl IntoView {
	let cls = cn!(INPUT_BASE, input_size_class(size), class);
	let input_type = if r#type.is_empty() { "text".to_string() } else { r#type };
	let form = use_context::<Signal<FormControlContext>>();
	view! {
		<input
			type=input_type
			class=cls
			data-slot="input"
			data-size=size.as_ref().to_owned()
			id=move || form.map(|f| f.get().id)
			aria-describedby=move || form.map(|f| f.get().described_by)
			aria-invalid=move || form.map(|f| f.get().invalid.to_string())
			name=name
			required=required
			placeholder=placeholder
			disabled=disabled
			value=value.get_untracked()
			prop:value=value
			on:input=move |e| {
				if let Some(h) = on_input {
					h.run(e);
				}
			}
		/>
	}
}
