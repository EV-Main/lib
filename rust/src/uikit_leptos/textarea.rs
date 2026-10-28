use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{FormControlContext, Size, TEXTAREA_BASE, textarea_size_class},
};

/// Picks up the id and `aria-*` of an enclosing `FormControl`; outside one, all
/// three are absent.
#[component]
pub fn Textarea(
	#[prop(optional)] size: Size,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] placeholder: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] value: Signal<String>,
	#[prop(optional, into)] name: Option<String>,
	#[prop(optional)] required: bool,
	#[prop(optional)] on_input: Option<Callback<leptos::ev::Event>>,
) -> impl IntoView {
	let cls = cn!(TEXTAREA_BASE, textarea_size_class(size), class);
	let form = use_context::<Signal<FormControlContext>>();
	view! {
		<textarea
			class=cls
			data-slot="textarea"
			data-size=size.as_ref().to_owned()
			id=move || form.map(|f| f.get().id)
			aria-describedby=move || form.map(|f| f.get().described_by)
			aria-invalid=move || form.map(|f| f.get().invalid.to_string())
			name=name
			required=required
			placeholder=placeholder
			disabled=disabled
			prop:value=value
			on:input=move |e| {
				if let Some(h) = on_input {
					h.run(e);
				}
			}
		>
			{value.get_untracked()}
		</textarea>
	}
	// inert in a browser; the Dioxus port renders it because it drives the live value there
	.attr("value", value.get_untracked())
}
