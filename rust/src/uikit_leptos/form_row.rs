use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{FORM_ROW_BASE, FORM_ROW_CONTROL, FORM_ROW_DESCRIPTION, FORM_ROW_DIVIDER, FORM_ROW_HINT, FORM_ROW_LABEL, FORM_ROW_LABEL_COLUMN},
};

/// Label on the left, the value or its control on the right; they stack on a
/// phone. Mirrors the TS `FormRow`. A plain `div`, not a list item: its label
/// column has a fixed width so controls line up down the page.
///
/// `r#for` is the id of the control in `children`; with it the label is a
/// `<label for>`, without it a `<span>`, because a `<label>` that labels
/// nothing confuses assistive tech.
#[component]
pub fn FormRow(
	#[prop(into)] label: ViewFn,
	#[prop(optional, into)] r#for: Option<String>,
	#[prop(optional, into)] description: Option<ViewFn>,
	#[prop(optional, into)] hint: Option<ViewFn>,
	#[prop(default = true)] divider: bool,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	view! {
		<FormRowFrame label r#for description hint divider inline=None class>
			{children()}
		</FormRowFrame>
	}
}

/// The markup behind [`FormRow`]. `inline` is the TS `SettingsRow`'s
/// `data-inline`, which the TS spreads onto the row through `...props`; the
/// kit has no such spread, so the settings row reaches the frame directly.
#[component]
pub(crate) fn FormRowFrame(
	label: ViewFn,
	r#for: Option<String>,
	description: Option<ViewFn>,
	hint: Option<ViewFn>,
	divider: bool,
	inline: Option<bool>,
	class: String,
	children: Children,
) -> impl IntoView {
	let cls = cn!(FORM_ROW_BASE, if divider { FORM_ROW_DIVIDER } else { "" }, class);
	view! {
		<div
			class=cls
			data-slot="form-row"
			data-divider=divider.to_string()
			data-inline=inline.map(|on| on.to_string())
		>
			<div class=FORM_ROW_LABEL_COLUMN data-slot="form-row-label">
				{match r#for {
					Some(id) => {
						view! {
							<label class=FORM_ROW_LABEL for=id>
								{label.run()}
							</label>
						}
							.into_any()
					}
					None => view! { <span class=FORM_ROW_LABEL>{label.run()}</span> }.into_any(),
				}}
				{description.map(|d| view! { <span class=FORM_ROW_DESCRIPTION>{d.run()}</span> })}
			</div>
			<div class=FORM_ROW_CONTROL data-slot="form-row-control">
				{children()}
				{hint.map(|h| view! { <span class=FORM_ROW_HINT>{h.run()}</span> })}
			</div>
		</div>
	}
}
