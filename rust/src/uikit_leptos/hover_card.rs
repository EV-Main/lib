use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{
		HOVER_CARD_CONTENT,
		primitives::{Controllable, Side, use_controllable},
	},
};

#[component]
pub fn HoverCard(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	provide_context(HoverCardCtx { open: state });
	// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations
	view! {
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
	}
}

#[component]
pub fn HoverCardTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<HoverCardCtx>();
	view! {
		<a
			class=class
			data-slot="hover-card-trigger"
			on:mouseenter=move |_| ctx.open.set(true)
			on:mouseleave=move |_| ctx.open.set(false)
		>
			{children()}
		</a>
	}
}

#[component]
pub fn HoverCardContent(#[prop(optional)] side: Side, #[prop(default = "center".into(), into)] align: String, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<HoverCardCtx>();
	let cls = cn!(HOVER_CARD_CONTENT, "absolute top-full left-1/2 -translate-x-1/2 mt-1", class);
	move || {
		ctx.open.get().then(|| {
			view! {
				<div
					class=cls.clone()
					data-slot="hover-card-content"
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
struct HoverCardCtx {
	open: Controllable<bool>,
}
