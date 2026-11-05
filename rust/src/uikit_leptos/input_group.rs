use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{
		ButtonVariant, INPUT_BASE, INPUT_GROUP_ADDON_BASE, INPUT_GROUP_BASE, INPUT_GROUP_BUTTON_BASE, INPUT_GROUP_INPUT_CONTROL, INPUT_GROUP_TEXT, INPUT_GROUP_TEXTAREA_CONTROL,
		InputGroupAddonAlign, InputGroupButtonSize, TEXTAREA_BASE, button::Button, input_group_button_size_class,
	},
};

#[component]
pub fn InputGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(INPUT_GROUP_BASE, class);
	view! {
		<div class=cls data-slot="input-group" role="group">
			{children()}
		</div>
	}
}

#[component]
pub fn InputGroupAddon(#[prop(optional)] align: InputGroupAddonAlign, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(INPUT_GROUP_ADDON_BASE, align.as_class(), class);
	view! {
		<div role="group" class=cls data-slot="input-group-addon" data-align=align.as_ref().to_owned()>
			{children()}
		</div>
	}
}

/// Defaults to `type="button"` like the TS mirror: an addon action sits inside
/// the consumer's `Form`, and the HTML default (`submit`) would navigate away
/// instead of running its handler.
#[component]
pub fn InputGroupButton(
	#[prop(optional)] size: InputGroupButtonSize,
	#[prop(optional)] icon: bool,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(default = "button".to_owned(), into)] r#type: String,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	let cls = cn!(INPUT_GROUP_BUTTON_BASE, input_group_button_size_class(size, icon), class);
	view! {
		<Button
			variant=ButtonVariant::Ghost
			r#type=r#type
			disabled=disabled
			on_click=Callback::new(move |e| {
				if let Some(h) = on_click {
					h.run(e)
				}
			})
			class=cls
		>
			{children()}
		</Button>
	}
}

#[component]
pub fn InputGroupText(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(INPUT_GROUP_TEXT, class);
	view! { <span class=cls>{children()}</span> }
}

#[component]
pub fn InputGroupInput(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] r#type: String,
	#[prop(optional, into)] placeholder: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] value: Signal<String>,
	#[prop(optional)] on_input: Option<Callback<leptos::ev::Event>>,
) -> impl IntoView {
	let cls = cn!(INPUT_BASE, INPUT_GROUP_INPUT_CONTROL, class);
	let input_type = if r#type.is_empty() { "text".to_owned() } else { r#type };
	view! {
		<input
			type=input_type
			class=cls
			data-slot="input-group-control"
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

#[component]
pub fn InputGroupTextarea(
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] placeholder: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] value: Signal<String>,
	#[prop(optional)] on_input: Option<Callback<leptos::ev::Event>>,
) -> impl IntoView {
	let cls = cn!(TEXTAREA_BASE, INPUT_GROUP_TEXTAREA_CONTROL, class);
	view! {
		<textarea
			class=cls
			data-slot="input-group-control"
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
