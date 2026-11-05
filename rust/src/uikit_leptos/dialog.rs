use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit_leptos::{
		DIALOG_CLOSE, DIALOG_CONTENT, DIALOG_DESCRIPTION, DIALOG_FOOTER, DIALOG_HEADER, DIALOG_OVERLAY, DIALOG_TITLE,
		primitives::{Controllable, DismissableLayer, use_controllable, use_dismissable_layer},
	},
};

#[component]
pub fn Dialog(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let layer = use_dismissable_layer(Signal::derive(move || state.get()), move |_| state.set(false));
	let provided = DialogCtx { open: state, layer };
	view! {
		<Provider value=provided>
			<div data-slot="dialog">{children()}</div>
		</Provider>
	}
}

#[component]
pub fn DialogTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<DialogCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="dialog-trigger"
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(true)
		>
			{children()}
		</button>
	}
}

/// Pass-through that mirrors the TS `DialogPortal`; the Rust kit renders inline
/// (no portal — see README Limitations).
#[component]
pub fn DialogPortal(children: Children) -> impl IntoView {
	children()
}

/// `aria_labelledby` and `aria_describedby` name the dialog after its title
/// and description: pass the ids given to [`DialogTitle`] and
/// [`DialogDescription`]. The kit does not wire them on its own, unlike the
/// TS `Dialog`, so a dialog left without them stays unnamed.
#[component]
pub fn DialogContent(
	#[prop(default = true)] show_close_button: bool,
	#[prop(optional, into)] aria_labelledby: Option<String>,
	#[prop(optional, into)] aria_describedby: Option<String>,
	#[prop(optional, into)] class: String,
	children: ChildrenFn,
) -> impl IntoView {
	let ctx = expect_context::<DialogCtx>();
	let cls = cn!(DIALOG_CONTENT, class);
	move || {
		ctx.open.get().then(|| {
			let children = children.clone();
			// dep-light: native focus order, no trap/portal — see README Limitations
			view! {
				<div
					class=DIALOG_OVERLAY
					data-slot="dialog-overlay"
					data-state="open"
					on:click=move |_| ctx.layer.on_outside()
				></div>
				<div
					role="dialog"
					aria-modal="true"
					aria-labelledby=aria_labelledby.clone()
					aria-describedby=aria_describedby.clone()
					class=cls.clone()
					data-slot="dialog-content"
					data-state="open"
					tabindex="-1"
					on:click=|e| e.stop_propagation()
					on:keydown=move |e| {
						ctx.layer.on_keydown(&e);
					}
				>
					{children()}
					{show_close_button
						.then(|| {
							view! {
								<button
									type="button"
									class=DIALOG_CLOSE
									data-slot="dialog-close"
									on:click=move |_| ctx.open.set(false)
								>
									<svg
										xmlns="http://www.w3.org/2000/svg"
										width="24"
										height="24"
										viewBox="0 0 24 24"
										fill="none"
										stroke="currentColor"
										stroke-width="2"
										stroke-linecap="round"
										stroke-linejoin="round"
									>
										<path d="M18 6 6 18M6 6l12 12"></path>
									</svg>
									<span class="sr-only">"Close"</span>
								</button>
							}
						})}
				</div>
			}
		})
	}
}

#[component]
pub fn DialogClose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<DialogCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="dialog-close"
			on:click=move |_| ctx.open.set(false)
		>
			{children()}
		</button>
	}
}

#[component]
pub fn DialogHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(DIALOG_HEADER, class) data-slot="dialog-header">
			{children()}
		</div>
	}
}

#[component]
pub fn DialogFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(DIALOG_FOOTER, class) data-slot="dialog-footer">
			{children()}
		</div>
	}
}

#[component]
pub fn DialogTitle(#[prop(optional, into)] id: Option<String>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<h2 id=id class=cn!(DIALOG_TITLE, class) data-slot="dialog-title">
			{children()}
		</h2>
	}
}

#[component]
pub fn DialogDescription(#[prop(optional, into)] id: Option<String>, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<p id=id class=cn!(DIALOG_DESCRIPTION, class) data-slot="dialog-description">
			{children()}
		</p>
	}
}

#[derive(Clone, Copy)]
struct DialogCtx {
	open: Controllable<bool>,
	layer: DismissableLayer,
}
