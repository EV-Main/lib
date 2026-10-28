use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit_leptos::{
		TOOLTIP_CONTENT,
		primitives::{Controllable, use_controllable},
	},
};

/// Optional, no-op wrapper kept for API parity with the TS `TooltipProvider`.
#[component]
pub fn TooltipProvider(children: Children) -> impl IntoView {
	children()
}

#[component]
pub fn Tooltip(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let provided = TooltipCtx { open: state };
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	view! {
		<Provider value=provided>
			<div
				class="relative inline-block"
				on:keydown=move |e| {
					if e.key() == "Escape" {
						state.set(false);
					}
				}
			>
				{children()}
			</div>
		</Provider>
	}
}

#[component]
pub fn TooltipTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<TooltipCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="tooltip-trigger"
			on:mouseenter=move |_| ctx.open.set(true)
			on:mouseleave=move |_| ctx.open.set(false)
			on:focusin=move |_| ctx.open.set(true)
			on:focusout=move |_| ctx.open.set(false)
		>
			{children()}
		</button>
	}
}

#[component]
pub fn TooltipContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<TooltipCtx>();
	let cls = cn!(TOOLTIP_CONTENT, "absolute bottom-full left-1/2 -translate-x-1/2 mb-1", class);
	move || {
		ctx.open.get().then(|| {
			view! {
				<div
					class=cls.clone()
					role="tooltip"
					data-slot="tooltip-content"
					data-state="open"
					data-side="top"
				>
					{children()}
				</div>
			}
		})
	}
}

#[derive(Clone, Copy)]
struct TooltipCtx {
	open: Controllable<bool>,
}
