//! Toast notifications.
//!
//! Where the TS mirror exposes a module-global `toast(...)` function backed by a
//! singleton store, Dioxus has no clean cross-component mutable singleton, so the
//! Rust kit uses a [`ToasterProvider`] that owns a [`Toasts`] signal in context
//! plus a [`use_toaster`] hook returning a [`ToasterHandle`] with `.toast`,
//! `.positive`, `.error`, `.info`, `.warn`, `.show` and `.dismiss`.
//!
//! ## Stacking + animation
//!
//! Mirrors Sonner (and the TS port) via the shared `data-stack` CSS in
//! `tokens.css`: toasts are absolutely stacked at the pinned edge and pile up
//! collapsed (front three peeking, scaled by depth), expanding into
//! a list on hover / keyboard focus — pure CSS, no host state. Each [`ToastItem`]
//! feeds the layout vars (`--index`, `--offset`, …); because heights can't be
//! measured here (that needs host-only `web-sys`), they're derived from a
//! constant height estimate, so the collapsed pile is exact and the expanded
//! list is uniformly spaced.
//!
//! Animation stays host-timer-free: the enter is a CSS keyframe (it plays on DOM
//! insertion, so no host-only mount flip is needed — Dioxus effect timing can't
//! drive one), [`ToasterHandle::dismiss`] (and the close button) flip
//! the toast to `data-state="closed"` to slide it out, and the
//! live node is dropped on the exit transform's `transitionend`
//! (`ontransitionend`, guarded to the closing state and the transform property)
//! rather than a `setTimeout`.
//! A `prefers-reduced-motion` block reduces the motion.
//!
//! Auto-dismiss is also host-timer-free: a no-op CSS `ev-toast-life` animation of
//! length `--life` runs, and its `animationend` closes the toast (`None` duration
//! = persistent, no life animation). Hovering/focusing the stack pauses just that
//! animation (`animation-play-state`), so the countdown holds and resumes with
//! the elapsed time preserved. Swipe-to-dismiss (pointer physics) stays TS-only;
//! the palette is the single dark theme (no `next-themes`). See the README
//! "Limitations".

use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{DEFAULT_DURATION_MS, ToastQueue, ToastState, ToastView, toaster_style, y_position},
	uikit_dioxus::{TOAST_BASE, TOAST_CLOSE, TOAST_CONTENT, TOAST_TITLE, TOASTER_BASE, ToastPosition, ToastVariant, primitives::is_transform_transition},
};

/// The live toast queue, held in context by [`ToasterProvider`].
#[derive(Clone, Copy)]
pub struct Toasts(Signal<ToastQueue>);

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
	/// [`ToasterHandle::dismiss`]. The convenience methods above use the TS
	/// `DEFAULT_DURATION` (4s); this is the mirror of the TS `toast(msg, { duration })`.
	pub fn show(&self, message: impl Into<String>, variant: ToastVariant, duration: Option<u32>) -> u64 {
		let mut queue = self.toasts.0;
		queue.write().push(message.into(), variant, duration)
	}

	/// Begins the exit animation: the toast flips to `data-state="closed"` and
	/// stays mounted so the exit transition can play; its `transitionend` drops
	/// it — there is no host timer. A toast dismissed before it has ever painted
	/// mounts closed and so never transitions; [`ToastItem`] drops that one on
	/// mount instead.
	pub fn dismiss(&self, id: u64) {
		let mut queue = self.toasts.0;
		queue.write().dismiss(id);
	}

	fn remove(&self, id: u64) {
		let mut queue = self.toasts.0;
		queue.write().remove(id);
	}
}

/// Provides the [`Toasts`] context. Wrap the subtree that both enqueues toasts
/// (via [`use_toaster`]) and renders the [`Toaster`].
#[component]
pub fn ToasterProvider(children: Element) -> Element {
	let queue = use_signal(ToastQueue::default);
	use_context_provider(|| Toasts(queue));
	rsx! {
		{children}
	}
}

/// Returns the imperative [`ToasterHandle`] from the nearest
/// [`ToasterProvider`]. The mirror of importing the TS global `toast`.
pub fn use_toaster() -> ToasterHandle {
	ToasterHandle { toasts: use_context::<Toasts>() }
}

/// Renders the toast stack from context as a Sonner-style pile: collapsed by
/// default (front three peeking, scaled by depth), expanding into a list on
/// hover / keyboard focus (pure CSS). Fixed-positioned per `position` (default
/// bottom-right). Unlike the TS mirror there is no `setTimeout` (a lifetime is a
/// CSS animation, paused on hover) and no swipe; dismiss happens on the close
/// button, at the end of the lifetime or via [`ToasterHandle::dismiss`], which
/// slides the toast out and drops it on its `transitionend`.
#[component]
pub fn Toaster(#[props(default)] position: ToastPosition, #[props(default)] class: String) -> Element {
	let toasts = use_context::<Toasts>();
	let cls = cn!(TOASTER_BASE, position.as_class(), class);
	let stack = toasts.0.read().stack();
	rsx! {
		ol {
			class: cls,
			"data-slot": "toaster",
			"data-position": position.as_ref(),
			"data-y-position": y_position(position),
			"data-stack": "",
			style: toaster_style(),
			for toast in stack {
				ToastItem { key: "{toast.id}", toast }
			}
		}
	}
}

/// One toast in the stack. Mirrors the TS `ToastItem`: the enter is a CSS
/// keyframe (plays on insertion), `data-state="closed"` slides it out, and the
/// live node is dropped on the exit transform's `transitionend` (guarded to the
/// closing state and to the transform property, so neither an open-state restack
/// nor a child's own transition removes it) — or straight away on mount, for a
/// toast dismissed before its first paint, which never transitions at all.
/// Layout vars come from the constant height — no measuring.
#[component]
fn ToastItem(toast: ToastView) -> Element {
	let handle = use_toaster();
	let id = toast.id;
	let state = toast.state;
	// Dismissed before its first paint (pushed and dismissed in one handler), so it
	// mounts already closed: its computed style never changes, no transition runs,
	// and the `ontransitionend` exit path below would never fire — stranding it in
	// the list forever. Nothing was painted, so there is nothing to animate: drop it
	// once mounted. The effect never runs under SSR, which has no lifecycle to miss.
	let mounted_closing = use_hook(|| state == ToastState::Closing);
	use_effect(move || {
		if mounted_closing {
			handle.remove(id);
		}
	});

	rsx! {
		li {
			role: "status",
			"aria-live": "polite",
			"data-slot": "toast",
			"data-variant": toast.variant.as_ref(),
			"data-state": state.as_str(),
			"data-front": "{toast.front}",
			"data-visible": "{toast.visible}",
			"data-autodismiss": "{toast.autodismiss}",
			style: toast.style,
			onanimationend: move |e| {
				if e.animation_name() == "ev-toast-life" {
					handle.dismiss(id);
				}
			},
			ontransitionend: move |e| {
				if state == ToastState::Closing && is_transform_transition(&e) {
					handle.remove(id);
				}
			},
			class: cn!(TOAST_BASE, toast.variant.as_class()),
			div { class: TOAST_CONTENT,
				div { class: TOAST_TITLE, "{toast.message}" }
			}
			button {
				r#type: "button",
				"aria-label": "Close",
				"data-slot": "toast-close",
				class: TOAST_CLOSE,
				onclick: move |_| handle.dismiss(id),
				svg {
					xmlns: "http://www.w3.org/2000/svg",
					width: "16",
					height: "16",
					view_box: "0 0 24 24",
					fill: "none",
					stroke: "currentColor",
					"stroke-width": "2",
					"stroke-linecap": "round",
					"stroke-linejoin": "round",
					"aria-hidden": "true",
					path { d: "M18 6 6 18M6 6l12 12" }
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::test_util::render;

	#[test]
	fn enqueued_toast_renders_with_role_and_variant() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Seed {}
					Toaster {}
				}
			}
		}
		#[component]
		fn Seed() -> Element {
			let toaster = use_toaster();
			use_hook(move || toaster.positive("Done"));
			rsx! {}
		}
		let html = render(app);
		assert!(html.contains("role=\"status\""), "{html}");
		assert!(html.contains("data-variant=\"positive\""), "{html}");
		assert!(html.contains("data-state=\"open\""), "fresh toast mounts open: {html}");
		assert!(html.contains("Done"), "{html}");
		assert!(html.contains("data-slot=\"toast\""), "{html}");
	}

	#[test]
	fn close_button_carries_the_dismiss_path() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Seed {}
					Toaster {}
				}
			}
		}
		#[component]
		fn Seed() -> Element {
			let toaster = use_toaster();
			use_hook(move || toaster.error("Oops"));
			rsx! {}
		}
		let html = render(app);
		assert!(html.contains("M18 6 6 18M6 6l12 12"), "{html}");
		assert!(html.contains("aria-label=\"Close\""), "{html}");
	}

	#[test]
	fn every_variant_helper_enqueues() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Seed {}
					Toaster {}
				}
			}
		}
		#[component]
		fn Seed() -> Element {
			let toaster = use_toaster();
			use_hook(move || {
				toaster.toast("plain");
				toaster.info("info");
				toaster.warn("warn");
			});
			rsx! {}
		}
		let html = render(app);
		assert!(html.contains("data-variant=\"info\""), "{html}");
		assert!(html.contains("data-variant=\"warn\""), "{html}");
	}

	#[test]
	fn dismiss_marks_toast_closing_for_exit_animation() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Seed {}
					Toaster {}
				}
			}
		}
		#[component]
		fn Seed() -> Element {
			let toaster = use_toaster();
			use_hook(move || {
				let id = toaster.positive("keep");
				toaster.dismiss(id);
			});
			rsx! {}
		}
		// Dismiss animates out rather than removing: the toast stays mounted in
		// the closing state (the live node is dropped on `transitionend`, which
		// the static SSR render cannot fire).
		let html = render(app);
		assert!(html.contains("keep"), "closing toast still mounted: {html}");
		assert!(html.contains("data-state=\"closed\""), "{html}");
	}

	#[test]
	fn a_toast_dismissed_before_first_paint_is_dropped_on_mount() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Seed {}
					Toaster {}
				}
			}
		}
		#[component]
		fn Seed() -> Element {
			let toaster = use_toaster();
			use_hook(move || {
				let id = toaster.info("cancelled");
				toaster.dismiss(id);
			});
			rsx! {}
		}
		// Mounted closed, it never transitions, so `ontransitionend` can't drop it —
		// the mount effect must, or it is stranded in the list forever. SSR runs no
		// effects, so drive it through a live VirtualDom.
		let mut dom = VirtualDom::new(app);
		dom.rebuild_in_place();
		dom.render_immediate(&mut dioxus_core::NoOpMutations);
		let html = dioxus_ssr::render(&dom);
		assert!(!html.contains("cancelled"), "ghost toast left mounted: {html}");
	}

	#[test]
	fn position_drives_the_root_attribute() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Toaster { position: ToastPosition::TopCenter }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-position=\"top-center\""), "{html}");
		assert!(html.contains("data-y-position=\"top\""), "{html}");
		assert!(html.contains("data-stack"), "stack model is on: {html}");
		assert!(html.contains("data-slot=\"toaster\""), "{html}");
	}

	#[test]
	fn stack_marks_front_and_depth() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Seed {}
					Toaster {}
				}
			}
		}
		#[component]
		fn Seed() -> Element {
			let toaster = use_toaster();
			use_hook(move || {
				toaster.info("older");
				toaster.warn("newer");
			});
			rsx! {}
		}
		// newest toast is the front of the stack (index 0); both carry stacking vars
		let html = render(app);
		assert!(html.contains("data-front=\"true\""), "a front toast: {html}");
		assert!(html.contains("data-front=\"false\""), "a back toast: {html}");
		assert!(html.contains("--index: 0"), "{html}");
		assert!(html.contains("--index: 1"), "{html}");
		assert!(html.contains("data-visible=\"true\""), "{html}");
	}

	#[test]
	fn default_toasts_auto_dismiss_persistent_ones_do_not() {
		fn app() -> Element {
			rsx! {
				ToasterProvider {
					Seed {}
					Toaster {}
				}
			}
		}
		#[component]
		fn Seed() -> Element {
			let toaster = use_toaster();
			use_hook(move || {
				toaster.positive("times out");
				toaster.show("sticky", ToastVariant::Info, None);
			});
			rsx! {}
		}
		let html = render(app);
		assert!(html.contains("data-autodismiss=\"true\""), "default auto-dismisses: {html}");
		assert!(html.contains("--life: 4000ms"), "{html}");
		assert!(html.contains("data-autodismiss=\"false\""), "persistent stays: {html}");
	}

	#[test]
	fn position_serializes_kebab() {
		assert_eq!(ToastPosition::TopLeft.as_ref(), "top-left");
	}
}
