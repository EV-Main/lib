//! Drawer — the Vaul-style edge sheet. Motion is the shared `motion.css`
//! contract, keyed on `data-slot` + `data-vaul-drawer-direction` + `data-state`;
//! the panel stays mounted after `open` turns false, until its exit `transform`'s
//! `transitionend`. Drag-to-dismiss is TS-only; see the README "Limitations".

use leptos::prelude::*;

use crate::{
	cn,
	uikit::Presence,
	uikit_leptos::{
		DRAWER_BODY, DRAWER_CONTENT_BASE, DRAWER_DESCRIPTION, DRAWER_FOOTER, DRAWER_HANDLE, DRAWER_HEADER, DRAWER_OVERLAY, DRAWER_TITLE, DrawerDirection,
		primitives::{Controllable, use_controllable},
	},
};

// dep-light: inline positioning + backdrop; no portal/floating — see README Limitations

#[component]
pub fn Drawer(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	#[prop(optional)] direction: DrawerDirection,
	children: Children,
) -> impl IntoView {
	let open = use_controllable(open, default_open, on_open_change);
	provide_context(DrawerCtx { open, direction });
	view! { <div data-slot="drawer">{children()}</div> }
}

#[component]
pub fn DrawerTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<DrawerCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="drawer-trigger"
			on:click=move |_| ctx.open.set(true)
		>
			{children()}
		</button>
	}
}

/// Standalone scrim, redundant with the one [`DrawerContent`] renders (as in
/// shadcn). Mounted only while open; the animated exit lives on the content's
/// own scrim.
#[component]
pub fn DrawerOverlay(#[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = expect_context::<DrawerCtx>();
	let cls = cn!(DRAWER_OVERLAY, class);
	move || {
		ctx.open
			.get()
			.then(|| view! {
				<div
					class=cls.clone()
					data-slot="drawer-overlay"
					data-state="open"
					on:click=move |_| ctx.open.set(false)
				></div>
			})
	}
}

/// The panel plus its scrim. It outlives `open`: closing flips it to
/// `data-state="closed"`, and it unmounts when its exit `transform` transition ends.
#[component]
pub fn DrawerContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<DrawerCtx>();
	let presence = RwSignal::new(Presence::new(ctx.open.get_untracked()));
	Effect::new(move |_| {
		let next = presence.get_untracked().follow(ctx.open.get());
		if presence.get_untracked() != next {
			presence.set(next);
		}
	});
	// The panel must survive open → closed in place, or the exit has nothing to transition.
	let present = Memo::new(move |_| presence.get() != Presence::Absent);
	let state = move || presence.get().state();
	let direction = ctx.direction;
	let cls = cn!(DRAWER_CONTENT_BASE, direction.as_class(), class);
	let panel = NodeRef::<leptos::html::Div>::new();
	move || {
		present.get().then(|| {
			let children = children.clone();
			view! {
				<div
					class=DRAWER_OVERLAY
					data-slot="drawer-overlay"
					data-state=state
					on:click=move |_| ctx.open.set(false)
				></div>
				<div
					role="dialog"
					aria-modal="true"
					class=cls.clone()
					data-slot="drawer-content"
					data-state=state
					data-vaul-drawer-direction=direction.as_ref().to_owned()
					node_ref=panel
					on:keydown=move |e| {
						if e.key() == "Escape" {
							ctx.open.set(false);
						}
					}
					on:transitionend=move |e| {
						let next = presence.get_untracked().exit_ended();
						if next != presence.get_untracked() && is_own_transform(&e, panel) {
							presence.set(next);
						}
					}
				>
					{(direction == DrawerDirection::Bottom)
						.then(|| view! { <div class=DRAWER_HANDLE data-slot="drawer-handle"></div> })}
					<div class=DRAWER_BODY data-slot="drawer-body">
						{children()}
					</div>
				</div>
			}
		})
	}
}

#[component]
pub fn DrawerClose(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<DrawerCtx>();
	view! {
		<button
			type="button"
			class=class
			data-slot="drawer-close"
			on:click=move |_| ctx.open.set(false)
		>
			{children()}
		</button>
	}
}

#[component]
pub fn DrawerHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(DRAWER_HEADER, class) data-slot="drawer-header">
			{children()}
		</div>
	}
}

#[component]
pub fn DrawerFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(DRAWER_FOOTER, class) data-slot="drawer-footer">
			{children()}
		</div>
	}
}

#[component]
pub fn DrawerTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(DRAWER_TITLE, class) data-slot="drawer-title">
			{children()}
		</div>
	}
}

#[component]
pub fn DrawerDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	view! {
		<div class=cn!(DRAWER_DESCRIPTION, class) data-slot="drawer-description">
			{children()}
		</div>
	}
}

#[derive(Clone, Copy)]
struct DrawerCtx {
	open: Controllable<bool>,
	direction: DrawerDirection,
}

/// Whether a `transitionend` is the panel's own exit `transform` finishing:
/// `transitionend` bubbles, and a descendant's (a button's colour fade, a
/// child's own transform) must not cut the slide-out short. The target is
/// compared with the panel rather than `currentTarget`, which Leptos's event
/// delegation leaves on the document. Mirrors the TS `propertyName === "transform"` guard.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn is_own_transform(e: &leptos::ev::TransitionEvent, panel: NodeRef<leptos::html::Div>) -> bool {
	use leptos::wasm_bindgen::JsCast;
	let own = match (e.target(), panel.get_untracked()) {
		(Some(target), Some(panel)) => target.dyn_ref::<web_sys::Element>() == Some(&**panel),
		_ => false,
	};
	own && e.property_name() == "transform"
}
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
fn is_own_transform(_: &leptos::ev::TransitionEvent, _: NodeRef<leptos::html::Div>) -> bool {
	true
}
