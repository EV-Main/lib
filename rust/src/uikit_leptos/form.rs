//! Dep-light, presentational form primitives — the kit keeps the ARIA id-wiring
//! of shadcn's form (label/control/description/message share an id so
//! `aria-describedby` and `id` line up) but drops `react-hook-form`'s state
//! engine: validation and field state are the consumer's job.
//!
//! [`FormItem`] derives an id from its place in the tree and provides it via
//! context; [`FormLabel`], [`FormDescription`] and [`FormMessage`] read it. There
//! is no `Slot`, so [`FormControl`] publishes the control's `id`/`aria-*` as a
//! `Signal<`[`FormControlContext`](crate::uikit_leptos::FormControlContext)`>` and the kit's own controls
//! ([`Input`](crate::uikit_leptos::Input), …) apply them to themselves. Wrapping
//! a bare element is the one case the consumer still wires by hand.

use leptos::prelude::*;

use crate::{
	cn,
	uikit::FormItemContext,
	uikit_leptos::{FORM_DESCRIPTION, FORM_ITEM, FORM_LABEL, FORM_MESSAGE, label::Label, primitives::use_stable_id},
};

/// Passthrough `<form>` wrapper. Mirrors TS `Form` (which is rhf's
/// `FormProvider`); here it owns no state, so consumers wire submit.
#[component]
pub fn Form(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<form class=class data-slot="form">
			{children()}
		</form>
	}
}

/// Provides an id via context so the label/control/description/message
/// underneath share `aria-describedby`/`id`. The id follows the item's place in
/// the tree ([`use_stable_id`]), so it matches across SSR and hydration.
#[component]
pub fn FormItem(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	provide_context(FormItemContext { id: use_stable_id("form-item") });
	let cls = cn!(FORM_ITEM, class);
	view! {
		<div class=cls data-slot="form-item">
			{children()}
		</div>
	}
}

/// Wraps [`Label`], pointing `for` at the control id. `error` folds the
/// destructive colour into the class: `Label` forwards no `data-error` for a
/// `data-[error=true]:` variant to react to.
#[component]
pub fn FormLabel(#[prop(optional, into)] error: Signal<bool>, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<FormItemContext>();
	move || {
		let cls = cn!(if error.get() { FORM_LABEL } else { "" }, class.as_str());
		let children = children.clone();
		view! {
			<Label class=cls r#for=ctx.form_item_id()>
				{children()}
			</Label>
		}
	}
}

/// Hands the control id and `aria-*` ids to the control underneath, as a
/// `Signal<`[`FormControlContext`](crate::uikit_leptos::FormControlContext)`>`; `aria-invalid` follows `error`. They must
/// land on the control, not on this wrapper: `FormLabel`'s `for` only focuses a
/// labelable element, and `aria-invalid:*` styling lives on the control's own
/// classes.
#[component]
pub fn FormControl(#[prop(optional, into)] error: Signal<bool>, children: Children) -> impl IntoView {
	let ctx = expect_context::<FormItemContext>();
	provide_context(Signal::derive(move || ctx.control(error.get())));
	view! { <div data-slot="form-control">{children()}</div> }
}

/// Muted helper copy, carrying the description id the control points at.
#[component]
pub fn FormDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<FormItemContext>();
	let cls = cn!(FORM_DESCRIPTION, class);
	view! {
		<p class=cls data-slot="form-description" id=ctx.form_description_id()>
			{children()}
		</p>
	}
}

/// Error/validation text. Renders only while `error`, the same flag the
/// `FormControl` gets — so the control is described by the message exactly when
/// it exists, and a permanently mounted message on a valid field leaves no empty
/// row in the item's grid.
#[component]
pub fn FormMessage(#[prop(optional, into)] error: Signal<bool>, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<FormItemContext>();
	let cls = cn!(FORM_MESSAGE, class);
	move || {
		error.get().then(|| {
			let children = children.clone();
			view! {
				<p class=cls.clone() data-slot="form-message" id=ctx.form_message_id()>
					{children()}
				</p>
			}
		})
	}
}
