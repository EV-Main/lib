//! Toast notifications. A [`ToasterProvider`] owns the queue in context and
//! [`use_toaster`] hands out a [`ToasterHandle`] with `.toast`, `.positive`,
//! `.error`, `.info`, `.warn`, `.show` and `.dismiss` — the mirror of the TS
//! global `toast`.
//!
//! Stacking, animation and auto-dismiss work as in the Dioxus port, with no host
//! timer: the stack layout is CSS (`data-stack` in `tokens.css`) fed by each
//! toast's layout vars, the enter is a keyframe, a lifetime is the no-op
//! `ev-toast-life` animation whose `animationend` dismisses (paused on hover),
//! and a dismissed toast is dropped on its exit transform's `transitionend`.

use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit::{DEFAULT_DURATION_MS, ToastQueue, ToastState, ToastView, toaster_style, y_position},
	uikit_leptos::{TOAST_BASE, TOAST_CLOSE, TOAST_CONTENT, TOAST_TITLE, TOASTER_BASE, ToastPosition, ToastVariant, primitives::is_transform_transition},
};

/// The live toast queue, held in context by [`ToasterProvider`].
#[derive(Clone, Copy)]
pub struct Toasts(RwSignal<ToastQueue>);

/// Imperative handle, the mirror of the TS global `toast` object. Obtained via
/// [`use_toaster`]; each method enqueues a toast of the matching variant and
/// returns its id, and [`ToasterHandle::dismiss`] closes one by id.
#[derive(Clone, Copy)]
pub struct ToasterHandle {
	toasts: Toasts,
}

impl ToasterHandle {
	pub fn toast(&self, message: impl Into<String>) -> u64 {
		self.show(message, ToastVariant::Neutral, Some(DEFAULT_DURATION_MS))
	}

	pub fn positive(&self, message: impl Into<String>) -> u64 {
		self.show(message, ToastVariant::Positive, Some(DEFAULT_DURATION_MS))
	}

	pub fn error(&self, message: impl Into<String>) -> u64 {
		self.show(message, ToastVariant::Error, Some(DEFAULT_DURATION_MS))
	}

	pub fn info(&self, message: impl Into<String>) -> u64 {
		self.show(message, ToastVariant::Info, Some(DEFAULT_DURATION_MS))
	}

	pub fn warn(&self, message: impl Into<String>) -> u64 {
		self.show(message, ToastVariant::Warn, Some(DEFAULT_DURATION_MS))
	}

	/// Full control over variant + lifetime. `duration` is ms, or `None` for a
	/// **persistent** toast that stays until the close button or
	/// [`ToasterHandle::dismiss`]; the mirror of the TS `toast(msg, { duration })`.
	pub fn show(&self, message: impl Into<String>, variant: ToastVariant, duration: Option<u32>) -> u64 {
		let message = message.into();
		let mut id = None;
		self.toasts.0.update(|q| id = Some(q.push(message, variant, duration)));
		id.expect("`update` runs its closure")
	}

	/// Begins the exit animation: the toast flips to `data-state="closed"` and
	/// stays mounted so the exit transition can play; its `transitionend` drops
	/// it. A toast dismissed before it has ever painted mounts closed and never
	/// transitions, so it is dropped on mount instead.
	pub fn dismiss(&self, id: u64) {
		self.toasts.0.update(|q| q.dismiss(id));
	}

	fn remove(&self, id: u64) {
		self.toasts.0.update(|q| q.remove(id));
	}
}

/// Provides the [`Toasts`] context. Wrap the subtree that both enqueues toasts
/// (via [`use_toaster`]) and renders the [`Toaster`].
#[component]
pub fn ToasterProvider(children: Children) -> impl IntoView {
	// A Provider, not `provide_context`: a component has no owner of its own, so a bare provide would reach its siblings.
	view! { <Provider value=Toasts(RwSignal::new(ToastQueue::default()))>{children()}</Provider> }
}

/// Returns the imperative [`ToasterHandle`] from the nearest
/// [`ToasterProvider`]. The mirror of importing the TS global `toast`.
pub fn use_toaster() -> ToasterHandle {
	ToasterHandle { toasts: expect_context::<Toasts>() }
}

/// Renders the toast stack from context as a Sonner-style pile: collapsed by
/// default (front three peeking, scaled by depth), expanding into a list on
/// hover / keyboard focus (pure CSS). Fixed-positioned per `position` (default
/// bottom-right). No swipe; dismiss happens on the close button, at the end of
/// the lifetime or via [`ToasterHandle::dismiss`].
#[component]
pub fn Toaster(#[prop(optional)] position: ToastPosition, #[prop(optional, into)] class: String) -> impl IntoView {
	let toasts = use_toaster().toasts;
	let cls = cn!(TOASTER_BASE, position.as_class(), class);
	view! {
		<ol
			class=cls
			data-slot="toaster"
			data-position=position.as_ref().to_owned()
			data-y-position=y_position(position)
			data-stack=""
			style=toaster_style()
		>
			// Keyed by id, so a toast keeps its node — and its running transition — as the stack shifts.
			<For each=move || toasts.0.with(ToastQueue::stack) key=|t| t.id let:toast>
				<ToastItem toast=toast />
			</For>
		</ol>
	}
}

/// One toast in the stack, mirroring the TS `ToastItem`; its place in the stack
/// is read back from the queue as toasts arrive and leave around it.
#[component]
fn ToastItem(toast: ToastView) -> impl IntoView {
	let handle = use_toaster();
	let id = toast.id;
	// The For drops this row on removal, but its attributes may re-read the
	// queue first: the last view stands for the moment that is left.
	let view = Memo::new(move |last: Option<&ToastView>| {
		handle
			.toasts
			.0
			.with(|q| q.stack().into_iter().find(|t| t.id == id))
			.or_else(|| last.cloned())
			.expect("a row starts from its toast in the queue")
	});
	// Mounted closed (pushed and dismissed in one handler), it never transitions,
	// so `on:transitionend` would never drop it. Effects never run under SSR.
	if toast.state == ToastState::Closing {
		Effect::new(move |_| handle.remove(id));
	}
	let flag = move |f: fn(&ToastView) -> bool| move || view.with(f).to_string();
	let node = NodeRef::<leptos::html::Li>::new();
	view! {
		<li
			node_ref=node
			role="status"
			aria-live="polite"
			data-slot="toast"
			data-variant=toast.variant.as_ref().to_owned()
			data-state=move || view.with(|t| t.state.as_str())
			data-front=flag(|t| t.front)
			data-visible=flag(|t| t.visible)
			data-autodismiss=flag(|t| t.autodismiss)
			style=move || view.with(|t| t.style.clone())
			on:animationend=move |e| {
				if e.animation_name() == "ev-toast-life" {
					handle.dismiss(id);
				}
			}
			on:transitionend=move |e| {
				if view.with_untracked(|t| t.state == ToastState::Closing)
					&& is_transform_transition(&e, node)
				{
					handle.remove(id);
				}
			}
			class=cn!(TOAST_BASE, toast.variant.as_class())
		>
			<div class=TOAST_CONTENT>
				<div class=TOAST_TITLE>{toast.message}</div>
			</div>
			<button
				type="button"
				aria-label="Close"
				data-slot="toast-close"
				class=TOAST_CLOSE
				on:click=move |_| handle.dismiss(id)
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="16"
					height="16"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					aria-hidden="true"
				>
					<path d="M18 6 6 18M6 6l12 12"></path>
				</svg>
			</button>
		</li>
	}
}
