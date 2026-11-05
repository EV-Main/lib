use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit::popover_placement,
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
	let provided = PopoverCtx { open: state, layer };
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	view! {
		<Provider value=provided>
			<div
				class="relative inline-block"
				on:keydown=move |e| {
					layer.on_keydown(&e);
				}
			>
				{children()}
			</div>
		</Provider>
	}
}

/// `aria_label` names a trigger that holds only a glyph: the kit has no
/// `asChild`, so an icon trigger cannot borrow the name of an `IconButton`.
#[component]
pub fn PopoverTrigger(#[prop(optional, into)] class: String, #[prop(optional, into)] aria_label: Option<String>, children: Children) -> impl IntoView {
	let ctx = expect_context::<PopoverCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="popover-trigger"
			aria-label=aria_label
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

/// `align` pins the panel's edge to the trigger's: `start` is the left edge,
/// `end` the right, anything else centres it. Without a measuring layer this is
/// all the placement there is, and a wide panel under a trigger at the edge of
/// its container needs `start` or `end` to stay in view.
///
/// `role` and `aria_labelledby` are for content that is a dialog of its own,
/// such as a form: the TS port forwards them as plain props.
#[component]
pub fn PopoverContent(
	#[prop(optional)] side: Side,
	#[prop(default = "center".into(), into)] align: String,
	#[prop(optional, into)] role: Option<String>,
	#[prop(optional, into)] aria_labelledby: Option<String>,
	#[prop(optional, into)] class: String,
	children: ChildrenFn,
) -> impl IntoView {
	let ctx = expect_context::<PopoverCtx>();
	let cls = cn!(POPOVER_CONTENT, popover_placement(&align), class);
	move || {
		ctx.open.get().then(|| {
			let children = children.clone();
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| ctx.layer.on_outside()></div>
				<div
					role=role.clone()
					aria-labelledby=aria_labelledby.clone()
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
