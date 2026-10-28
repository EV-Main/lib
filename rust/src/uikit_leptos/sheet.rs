use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit::sheet_side_class,
	uikit_leptos::{
		SHEET_CLOSE, SHEET_CONTENT, SHEET_DESCRIPTION, SHEET_FOOTER, SHEET_HEADER, SHEET_OVERLAY, SHEET_TITLE,
		primitives::{Controllable, Side, use_controllable},
	},
};

#[component]
pub fn Sheet(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let provided = SheetCtx { open: state };
	view! {
		<Provider value=provided>
			<div data-slot="sheet">{children()}</div>
		</Provider>
	}
}

#[component]
pub fn SheetTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<SheetCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="sheet-trigger"
			aria-expanded=move || ctx.open.get().to_string()
			on:click=move |_| ctx.open.set(true)
		>
			{children()}
		</button>
	}
}

#[component]
pub fn SheetContent(#[prop(default = Side::Right)] side: Side, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<SheetCtx>();
	let cls = cn!(SHEET_CONTENT, sheet_side_class(side), class);
	move || {
		ctx.open.get().then(|| {
			let children = children.clone();
			// dep-light: native focus order, no trap/portal — see README Limitations
			view! {
				<div
					class=SHEET_OVERLAY
					data-slot="sheet-overlay"
					data-state="open"
					on:click=move |_| ctx.open.set(false)
				></div>
				<div
					role="dialog"
					aria-modal="true"
					class=cls.clone()
					data-slot="sheet-content"
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
					<button
						type="button"
						class=SHEET_CLOSE
						data-slot="sheet-close"
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
							class="size-4"
						>
							<path d="M18 6 6 18M6 6l12 12"></path>
						</svg>
						<span class="sr-only">"Close"</span>
					</button>
				</div>
			}
		})
	}
}

#[component]
pub fn SheetClose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<SheetCtx>();
	view! {
		<button type="button" class=class data-slot="sheet-close" on:click=move |_| ctx.open.set(false)>
			{children()}
		</button>
	}
}

#[component]
pub fn SheetHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(SHEET_HEADER, class) data-slot="sheet-header">
			{children()}
		</div>
	}
}

#[component]
pub fn SheetFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(SHEET_FOOTER, class) data-slot="sheet-footer">
			{children()}
		</div>
	}
}

#[component]
pub fn SheetTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<h2 class=cn!(SHEET_TITLE, class) data-slot="sheet-title">
			{children()}
		</h2>
	}
}

#[component]
pub fn SheetDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<p class=cn!(SHEET_DESCRIPTION, class) data-slot="sheet-description">
			{children()}
		</p>
	}
}

#[derive(Clone, Copy)]
struct SheetCtx {
	open: Controllable<bool>,
}
