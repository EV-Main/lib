use leptos::prelude::*;

pub use crate::uikit::FieldLegendVariant;
use crate::{
	cn,
	uikit_leptos::{
		FIELD_BASE, FIELD_CONTENT, FIELD_DESCRIPTION, FIELD_ERROR, FIELD_GROUP, FIELD_LABEL, FIELD_LEGEND, FIELD_SEPARATOR, FIELD_SEPARATOR_CONTENT, FIELD_SEPARATOR_LINE, FIELD_SET,
		FIELD_TITLE, FieldOrientation, label::Label,
	},
};

#[component]
pub fn FieldSet(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_SET, class);
	view! {
		<fieldset class=cls data-slot="field-set">
			{children()}
		</fieldset>
	}
}

#[component]
pub fn FieldLegend(#[prop(optional)] variant: FieldLegendVariant, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_LEGEND, class);
	view! {
		<legend class=cls data-slot="field-legend" data-variant=variant.as_ref().to_owned()>
			{children()}
		</legend>
	}
}

#[component]
pub fn FieldGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_GROUP, class);
	view! {
		<div class=cls data-slot="field-group">
			{children()}
		</div>
	}
}

#[component]
pub fn Field(#[prop(optional)] orientation: FieldOrientation, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_BASE, orientation.as_class(), class);
	view! {
		<div role="group" class=cls data-slot="field" data-orientation=orientation.as_ref().to_owned()>
			{children()}
		</div>
	}
}

#[component]
pub fn FieldContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_CONTENT, class);
	view! {
		<div class=cls data-slot="field-content">
			{children()}
		</div>
	}
}

#[component]
pub fn FieldLabel(#[prop(optional, into)] class: String, #[prop(optional, into)] r#for: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_LABEL, class);
	view! {
		<Label class=cls r#for=r#for>
			{children()}
		</Label>
	}
}

#[component]
pub fn FieldTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_TITLE, class);
	view! {
		<div class=cls data-slot="field-label">
			{children()}
		</div>
	}
}

#[component]
pub fn FieldDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_DESCRIPTION, class);
	view! {
		<p class=cls data-slot="field-description">
			{children()}
		</p>
	}
}

/// A rule with optional centred copy ("or"), rendered only with `content`:
/// the copy's opaque `bg-background` pill would otherwise notch the line.
#[component]
pub fn FieldSeparator(#[prop(optional)] content: bool, #[prop(optional, into)] class: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
	let cls = cn!(FIELD_SEPARATOR, class);
	view! {
		<div class=cls data-slot="field-separator" data-content=if content { "true" } else { "false" }>
			<div role="separator" class=FIELD_SEPARATOR_LINE></div>
			{content
				.then(|| {
					view! {
						<span class=FIELD_SEPARATOR_CONTENT data-slot="field-separator-content">
							{children.map(|c| c())}
						</span>
					}
				})}
		</div>
	}
}

#[component]
pub fn FieldError(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(FIELD_ERROR, class);
	view! {
		<div role="alert" class=cls data-slot="field-error">
			{children()}
		</div>
	}
}
