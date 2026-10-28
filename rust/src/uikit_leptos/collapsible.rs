use leptos::prelude::*;

use crate::uikit_leptos::primitives::{Controllable, use_controllable};

#[component]
pub fn Collapsible(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let open = use_controllable(open, default_open, on_open_change);
	provide_context(CollapsibleCtx { open });
	view! {
		<div class=class data-slot="collapsible" data-state=move || data_state(open.get())>
			{children()}
		</div>
	}
}

#[component]
pub fn CollapsibleTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<CollapsibleCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="collapsible-trigger"
			data-state=move || data_state(ctx.open.get())
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			{children()}
		</button>
	}
}

#[component]
pub fn CollapsibleContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<CollapsibleCtx>();
	move || {
		ctx.open.get().then(|| {
			view! {
				<div class=class.clone() data-slot="collapsible-content" data-state="open">
					{children()}
				</div>
			}
		})
	}
}

fn data_state(open: bool) -> &'static str {
	if open { "open" } else { "closed" }
}

#[derive(Clone, Copy)]
struct CollapsibleCtx {
	open: Controllable<bool>,
}
