use ev_lib_classes::{ToastPosition, ToastVariant};

/// Only the front three toasts show while the stack is collapsed.
const VISIBLE_TOASTS: usize = 3;
/// Gap (px) between toasts once the stack is expanded.
const GAP: u32 = 14;
/// The kit measures nothing, so the stack assumes this height (px) for the
/// collapse clamp and the expanded spacing: the collapsed pile is exact, the
/// expanded list is uniformly spaced. Tuned to a single-line toast (p-4 + one
/// text-sm line).
const TOAST_HEIGHT_EST: u32 = 56;
/// Default auto-dismiss lifetime (ms). Mirrors the TS `DEFAULT_DURATION`.
pub(crate) const DEFAULT_DURATION_MS: u32 = 4000;
/// The `Toaster` root's layout vars.
pub(crate) fn toaster_style() -> String {
	format!("--front-height: {TOAST_HEIGHT_EST}px; --gap: {GAP}px;")
}

/// The edge a `Toaster` at `position` stacks against, for `data-y-position`.
pub(crate) fn y_position(position: ToastPosition) -> &'static str {
	match position {
		ToastPosition::TopLeft | ToastPosition::TopCenter | ToastPosition::TopRight => "top",
		ToastPosition::BottomLeft | ToastPosition::BottomCenter | ToastPosition::BottomRight => "bottom",
	}
}

/// Lifecycle phase of a single toast. A toast mounts `Open` (plays the enter
/// keyframe); a dismiss flips it to `Closing` (slides it out), and it is dropped
/// from the queue once the exit `transitionend` fires. Mirrors the TS
/// `data-state` of `"open"` / `"closed"`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ToastState {
	Open,
	Closing,
}
impl ToastState {
	pub(crate) fn as_str(&self) -> &'static str {
		match self {
			ToastState::Open => "open",
			ToastState::Closing => "closed",
		}
	}
}

#[derive(Clone, PartialEq)]
struct Toast {
	id: u64,
	message: String,
	variant: ToastVariant,
	state: ToastState,
	/// Auto-dismiss lifetime in ms, or `None` to persist until dismissed.
	duration: Option<u32>,
}

/// The live toasts, oldest first. Time-free: a lifetime runs out as a CSS
/// animation in the page, whose `animationend` the port turns into
/// [`ToastQueue::dismiss`].
#[derive(Clone, Default)]
pub(crate) struct ToastQueue {
	items: Vec<Toast>,
	next_id: u64,
}
impl ToastQueue {
	pub(crate) fn push(&mut self, message: String, variant: ToastVariant, duration: Option<u32>) -> u64 {
		let id = self.next_id;
		self.next_id += 1;
		self.items.push(Toast {
			id,
			message,
			variant,
			state: ToastState::Open,
			duration,
		});
		id
	}

	/// Begins the exit: the toast stays queued, `Closing`, so its slide-out can
	/// play. An id already gone is a no-op — a close click can race the
	/// `transitionend` of an earlier dismiss.
	pub(crate) fn dismiss(&mut self, id: u64) {
		if let Some(toast) = self.items.iter_mut().find(|t| t.id == id) {
			toast.state = ToastState::Closing;
		}
	}

	/// Drops a toast outright, once its exit has played.
	pub(crate) fn remove(&mut self, id: u64) {
		self.items.retain(|t| t.id != id);
	}

	/// Newest first: the newest toast is the front of the stack (index 0).
	pub(crate) fn stack(&self) -> Vec<ToastView> {
		let total = self.items.len();
		self.items.iter().rev().enumerate().map(|(index, t)| view(t, index, total)).collect()
	}

	/// The toast `id` as [`ToastQueue::stack`] renders it, `None` once removed.
	pub(crate) fn view_of(&self, id: u64) -> Option<ToastView> {
		let total = self.items.len();
		let at = self.items.iter().position(|t| t.id == id)?;
		Some(view(&self.items[at], total - 1 - at, total))
	}
}

/// One toast as the stack renders it.
#[derive(Clone, PartialEq)]
pub(crate) struct ToastView {
	pub id: u64,
	pub message: String,
	pub variant: ToastVariant,
	pub state: ToastState,
	/// The `data-*` attributes and layout vars, in render order.
	pub front: bool,
	pub visible: bool,
	pub autodismiss: bool,
	pub style: String,
}

fn view(toast: &Toast, index: usize, total: usize) -> ToastView {
	let offset = index as u32 * (TOAST_HEIGHT_EST + GAP);
	let z = total - index;
	let life = toast.duration.unwrap_or(0); // a persistent toast runs no life animation; 0 fills the var
	ToastView {
		id: toast.id,
		message: toast.message.clone(),
		variant: toast.variant,
		state: toast.state,
		front: index == 0,
		visible: index < VISIBLE_TOASTS,
		autodismiss: toast.duration.is_some(),
		style: format!("--index: {index}; --z-index: {z}; --offset: {offset}px; --initial-height: {TOAST_HEIGHT_EST}px; --life: {life}ms;"),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_newest_toast_fronts_the_stack_and_only_three_show() {
		let mut q = ToastQueue::default();
		let ids: Vec<u64> = (0..4).map(|n| q.push(format!("t{n}"), ToastVariant::Info, Some(DEFAULT_DURATION_MS))).collect();
		let stack = q.stack();
		assert_eq!(stack.iter().map(|t| t.id).collect::<Vec<_>>(), ids.iter().rev().copied().collect::<Vec<_>>());
		assert_eq!(stack.iter().map(|t| t.front).collect::<Vec<_>>(), [true, false, false, false]);
		assert_eq!(stack.iter().map(|t| t.visible).collect::<Vec<_>>(), [true, true, true, false]);
		assert!(stack[1].style.starts_with("--index: 1; --z-index: 3; --offset: 70px;"), "{}", stack[1].style);
		assert!(q.view_of(ids[2]) == Some(stack[1].clone()));
	}

	#[test]
	fn dismiss_keeps_the_toast_until_its_exit_has_played() {
		let mut q = ToastQueue::default();
		let id = q.push("bye".to_owned(), ToastVariant::Positive, None);
		q.dismiss(id);
		assert_eq!(q.view_of(id).map(|t| t.state), Some(ToastState::Closing));
		q.remove(id);
		assert!(q.view_of(id).is_none());
		q.dismiss(id);
		assert!(q.stack().is_empty());
	}

	#[test]
	fn only_a_lifetime_auto_dismisses() {
		let mut q = ToastQueue::default();
		q.push("times out".to_owned(), ToastVariant::Neutral, Some(DEFAULT_DURATION_MS));
		q.push("sticky".to_owned(), ToastVariant::Neutral, None);
		let stack = q.stack();
		assert!(!stack[0].autodismiss && stack[0].style.ends_with("--life: 0ms;"));
		assert!(stack[1].autodismiss && stack[1].style.ends_with("--life: 4000ms;"));
	}
}
