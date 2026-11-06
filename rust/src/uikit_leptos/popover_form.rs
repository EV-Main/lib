use leptos::prelude::*;

pub use crate::uikit::PopoverFormLabels;
use crate::{
	cn,
	uikit_leptos::{
		Button, ButtonVariant, POPOVER_FORM_ACTIONS, POPOVER_FORM_CONTENT, POPOVER_FORM_FIELDS, POPOVER_FORM_FORM, POPOVER_FORM_TITLE, Popover, PopoverContent, PopoverTrigger, Size,
		button_classes,
		primitives::{use_controllable, use_stable_id},
	},
};

/// A filter-style form in a popover: a title, fields, Reset and Apply.
/// Mirrors the TS `PopoverForm`.
///
/// `trigger` is the content of the opening button, not a button of its own:
/// the kit has no `asChild` slot, so [`PopoverTrigger`] renders the button.
///
/// It is a [`Popover`], so it keeps the kit's overlay limits: rendered inline,
/// no portal and no focus trap.
#[component]
pub fn PopoverForm(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	#[prop(into)] trigger: ViewFn,
	/// Fused over the secondary button face the trigger wears: the TS caller
	/// passes a `Button`, and Rust has no `asChild` to take its place.
	#[prop(optional, into)]
	trigger_class: String,
	/// The heading; it also names the form's dialog.
	#[prop(into)]
	title: ViewFn,
	/// Called on submit; the popover closes afterwards.
	#[prop(optional)]
	on_submit: Option<Callback<leptos::ev::SubmitEvent>>,
	/// Called when Reset is pressed; the browser resets native fields first.
	#[prop(optional)]
	on_reset: Option<Callback<()>>,
	#[prop(optional)] labels: PopoverFormLabels,
	#[prop(optional, into)] class: String,
	/// The fields: `Field`s with an `Input`, `Select` or `Checkbox`.
	children: ChildrenFn,
) -> impl IntoView {
	// Owned here, not left to the `Popover`, because submitting has to close it
	// and the popover's state is private to its parts.
	let state = use_controllable(open, default_open, on_open_change);
	let title_id = use_stable_id("popover-form-title");
	let cls = cn!(POPOVER_FORM_CONTENT, class);
	let trigger_cls = button_classes(&ButtonVariant::Secondary, Size::Md, false, None, &trigger_class);
	let submit_label = labels.submit.unwrap_or_else(|| String::from("Apply"));
	let reset_label = labels.reset.unwrap_or_else(|| String::from("Reset"));
	let labelled_by = title_id.clone();
	// Built per opening; each run clones what the inner parts take by value.
	let form = move || {
		let (submit_label, reset_label) = (submit_label.clone(), reset_label.clone());
		view! {
			<form
				class=POPOVER_FORM_FORM
				data-slot="popover-form"
				on:submit=move |e| {
					e.prevent_default();
					if let Some(h) = on_submit {
						h.run(e);
					}
					state.set(false);
				}
				on:reset=move |_| {
					if let Some(h) = on_reset {
						h.run(());
					}
				}
			>
				<h3 id=title_id.clone() class=POPOVER_FORM_TITLE data-slot="popover-form-title">
					{title.run()}
				</h3>
				<div class=POPOVER_FORM_FIELDS data-slot="popover-form-fields">
					{children()}
				</div>
				<div class=POPOVER_FORM_ACTIONS data-slot="popover-form-actions">
					<Button r#type="reset" variant=ButtonVariant::Ghost size=Size::Sm>
						{reset_label}
					</Button>
					<Button r#type="submit" variant=ButtonVariant::Primary size=Size::Sm>
						{submit_label}
					</Button>
				</div>
			</form>
		}
	};
	view! {
		<Popover
			open=Signal::derive(move || state.get())
			on_open_change=Callback::new(move |next| state.set(next))
		>
			<PopoverTrigger class=trigger_cls>{trigger.run()}</PopoverTrigger>
			<PopoverContent align="start" role="dialog" aria_labelledby=labelled_by class=cls>
				{form()}
			</PopoverContent>
		</Popover>
	}
}
