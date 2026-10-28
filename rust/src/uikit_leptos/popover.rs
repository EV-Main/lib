use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{
		POPOVER_CONTENT,
		primitives::{Controllable, DismissableLayer, Side, use_controllable, use_dismissable_layer},
	},
};

#[component]
pub fn Popover(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let layer = use_dismissable_layer(Signal::derive(move || state.get()), move |_| state.set(false));
	provide_context(PopoverCtx { open: state, layer });
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	view! {
		<div
			class="relative inline-block"
			on:keydown=move |e| {
				layer.on_keydown(&e);
			}
		>
			{children()}
		</div>
	}
}

#[component]
pub fn PopoverTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<PopoverCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="popover-trigger"
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
		</button>
	}
}

/// Optional anchor for API parity with the TS `PopoverAnchor`. Inline placement
/// makes it a passive marker (the content positions against the wrapper).
#[component]
pub fn PopoverAnchor(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=class data-slot="popover-anchor">
			{children()}
		</div>
	}
}

#[component]
pub fn PopoverContent(#[prop(optional)] side: Side, #[prop(default = "center".into(), into)] align: String, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<PopoverCtx>();
	let cls = cn!(POPOVER_CONTENT, "absolute top-full left-1/2 -translate-x-1/2 mt-1", class);
	move || {
		ctx.open.get().then(|| {
			let children = children.clone();
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| ctx.layer.on_outside()></div>
				<div
					class=cls.clone()
					data-slot="popover-content"
					data-state="open"
					data-side=side.as_ref().to_owned()
					data-align=align.clone()
				>
					{children()}
				</div>
			}
		})
	}
}

#[derive(Clone, Copy)]
struct PopoverCtx {
	open: Controllable<bool>,
	layer: DismissableLayer,
}
