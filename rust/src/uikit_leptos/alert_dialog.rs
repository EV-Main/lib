use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{
		ALERT_DIALOG_CONTENT, ALERT_DIALOG_DESCRIPTION, ALERT_DIALOG_FOOTER, ALERT_DIALOG_HEADER, ALERT_DIALOG_OVERLAY, ALERT_DIALOG_TITLE, ButtonVariant, Size, button_classes,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn AlertDialog(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	provide_context(AlertDialogCtx { open: state });
	view! { <div data-slot="alert-dialog">{children()}</div> }
}

#[component]
pub fn AlertDialogTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<AlertDialogCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="alert-dialog-trigger"
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(true)
		>
			{children()}
		</button>
	}
}

#[component]
pub fn AlertDialogContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<AlertDialogCtx>();
	let cls = cn!(ALERT_DIALOG_CONTENT, class);
	move || {
		ctx.open.get().then(|| {
			let children = children.clone();
			// dep-light: native focus order, no trap/portal — see README Limitations
			view! {
				<div
					class=ALERT_DIALOG_OVERLAY
					data-slot="alert-dialog-overlay"
					data-state="open"
					on:click=move |_| ctx.open.set(false)
				></div>
				<div
					role="alertdialog"
					aria-modal="true"
					class=cls.clone()
					data-slot="alert-dialog-content"
					data-state="open"
					tabindex="-1"
					on:click=|e| e.stop_propagation()
					on:keydown=move |e| {
						if e.key() == "Escape" {
							ctx.open.set(false);
						}
					}
				>
					{children()}
				</div>
			}
		})
	}
}

#[component]
pub fn AlertDialogHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(ALERT_DIALOG_HEADER, class) data-slot="alert-dialog-header">
			{children()}
		</div>
	}
}

#[component]
pub fn AlertDialogFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(ALERT_DIALOG_FOOTER, class) data-slot="alert-dialog-footer">
			{children()}
		</div>
	}
}

#[component]
pub fn AlertDialogTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<h2 class=cn!(ALERT_DIALOG_TITLE, class) data-slot="alert-dialog-title">
			{children()}
		</h2>
	}
}

#[component]
pub fn AlertDialogDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<p class=cn!(ALERT_DIALOG_DESCRIPTION, class) data-slot="alert-dialog-description">
			{children()}
		</p>
	}
}

/// The confirm button: runs `on_click` — the consumer's destructive action —
/// then closes the dialog.
#[component]
pub fn AlertDialogAction(#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<AlertDialogCtx>();
	let cls = button_classes(&ButtonVariant::Primary, Size::Md, false, None, &class);
	view! {
		<button
			type="button"
			class=cls
			data-slot="alert-dialog-action"
			on:click=move |e| ctx.run_then_close(on_click, e)
		>
			{children()}
		</button>
	}
}

/// The dismiss button: runs `on_click`, then closes the dialog.
#[component]
pub fn AlertDialogCancel(#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<AlertDialogCtx>();
	let cls = button_classes(&ButtonVariant::Outline, Size::Md, false, None, &class);
	view! {
		<button
			type="button"
			class=cls
			data-slot="alert-dialog-cancel"
			on:click=move |e| ctx.run_then_close(on_click, e)
		>
			{children()}
		</button>
	}
}

#[derive(Clone, Copy)]
struct AlertDialogCtx {
	open: Controllable<bool>,
}
impl AlertDialogCtx {
	fn run_then_close(&self, on_click: Option<Callback<leptos::ev::MouseEvent>>, e: leptos::ev::MouseEvent) {
		if let Some(h) = on_click {
			h.run(e);
		}
		self.open.set(false);
	}
}
