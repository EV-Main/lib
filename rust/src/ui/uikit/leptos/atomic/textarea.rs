use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{FormControlContext, Size, Slot, TEXTAREA_BASE, textarea_size_class},
};

/// Picks up the id and `aria-*` of an enclosing `FormControl`; outside one, all
/// three are absent.
///
/// `aria_label`, `rows`, `on_keydown` and `node_ref` are opt-in for a field
/// that a composite part owns: a chat composer names its field without a
/// label, starts it one line tall, sends on Enter, and puts focus back on it.
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
	#[prop(optional, into)] aria_label: Option<String>,
	#[prop(optional)] rows: Option<u32>,
	#[prop(optional)] on_keydown: Option<Callback<leptos::ev::KeyboardEvent>>,
	#[prop(optional)] node_ref: Option<NodeRef<leptos::html::Textarea>>,
) -> impl IntoView {
	let node = node_ref.unwrap_or_default(); // a field nobody asked to reach
	let cls = cn!(TEXTAREA_BASE, textarea_size_class(size), class);
	let form = use_context::<Signal<FormControlContext>>();
	view! {
		<textarea
			class=cls
			data-slot=Slot::Textarea.as_str()
			data-size=size.as_ref().to_owned()
			id=move || form.map(|f| f.get().id)
			aria-describedby=move || form.map(|f| f.get().described_by)
			aria-invalid=move || form.map(|f| f.get().invalid.to_string())
			aria-label=aria_label
			rows=rows.map(|r| r.to_string())
			name=name
			required=required
			placeholder=placeholder
			disabled=disabled
			prop:value=value
			node_ref=node
			on:input=move |e| {
				if let Some(h) = on_input {
					h.run(e);
				}
			}
			on:keydown=move |e| {
				if let Some(h) = on_keydown {
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
