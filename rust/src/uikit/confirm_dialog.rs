use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		AlertDialog, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger, ButtonVariant, CONFIRM_DIALOG_CONTENT,
		CONFIRM_DIALOG_DESCRIPTION, CONFIRM_DIALOG_ERROR, CONFIRM_DIALOG_FOOTER, CONFIRM_DIALOG_HEADER, CONFIRM_DIALOG_ICON, CONFIRM_DIALOG_PENDING, CONFIRM_DIALOG_SPINNER,
		CONFIRM_DIALOG_TEXT, CONFIRM_DIALOG_TITLE, ConfirmDialogTone, Size, Spinner,
		button::button_classes,
		primitives::{use_controllable, use_stable_id},
	},
};

/// The dialog's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfirmDialogLabels {
	/// The cancel button; "Cancel".
	pub cancel: Option<String>,
	/// Shown when the work fails with an empty `error`; "Something went wrong".
	pub error: Option<String>,
}

/// An alert dialog that asks before an action and holds while the action
/// runs. Mirrors the TS `ConfirmDialog`.
///
/// The TS `onConfirm` returns a promise and the dialog waits on it. Rust has
/// no promise to hand back from an event handler, so the parent reports its
/// work through two props instead:
///
/// - A synchronous `on_confirm` that leaves `pending` false closes the dialog
///   once it returns.
/// - Async work sets `pending` from `on_confirm` and clears it when the work
///   settles. While `pending` is set, both buttons are `aria-disabled`, the
///   confirm button is busy, and confirm, cancel, Escape and the scrim do
///   nothing. When `pending` clears, the dialog closes, unless `error` is set:
///   then it stays open and shows the error under the text.
///
/// `error` is read when the work settles, and only that confirm shows it:
/// a reopened dialog starts without it, as in TS. Clear it before the next
/// attempt succeeds, or that attempt fails too.
///
/// `trigger` is the content of the opening button: the kit has no `asChild`
/// slot. It is an [`AlertDialog`], so it keeps the kit's overlay limits:
/// rendered inline with `position: fixed`, no portal, no focus trap and no
/// focus return to the trigger.
#[component]
pub fn ConfirmDialog(
	open: Option<bool>,
	#[props(default)] default_open: bool,
	on_open_change: Option<EventHandler<bool>>,
	trigger: Option<Element>,
	/// The heading; it also names the dialog.
	title: Element,
	description: Option<Element>,
	#[props(default)] tone: ConfirmDialogTone,
	confirm_label: Element,
	on_confirm: EventHandler<()>,
	#[props(default)] pending: bool,
	error: Option<String>,
	/// Replaces the tone's glyph.
	icon: Option<Element>,
	#[props(default)] labels: ConfirmDialogLabels,
	#[props(default)] class: String,
) -> Element {
	let state = use_controllable(open, default_open, on_open_change);
	let is_open = state.get();
	// Set by a confirm until its work settles, so the dialog closes on its own
	// only for a confirm it started, and a press before the parent's `pending`
	// arrives cannot start the work twice.
	let mut awaiting = use_signal(|| false);
	let mut show_error = use_signal(|| false);
	// A controlled parent can reopen the dialog without going through `change`.
	use_effect(use_reactive!(|is_open| {
		if is_open && *show_error.peek() {
			show_error.set(false);
		}
	}));
	use_effect(use_reactive!(|pending, error| {
		if !awaiting() || pending {
			return;
		}
		awaiting.set(false);
		if error.is_some() {
			show_error.set(true);
		} else {
			state.set(false);
		}
	}));

	let mut change = move |next: bool| {
		if pending || *awaiting.peek() {
			return;
		}
		if next {
			show_error.set(false);
		}
		state.set(next);
	};
	let confirm = move |_| {
		if pending || *awaiting.peek() {
			return;
		}
		show_error.set(false);
		awaiting.set(true);
		on_confirm.call(());
	};

	let title_id = use_stable_id("confirm-dialog-title");
	let description_id = use_stable_id("confirm-dialog-description");
	let described_by = description.as_ref().map(|_| description_id.clone());
	let shown_error = error.filter(|_| show_error()).map(|e| {
		if e.is_empty() {
			labels.error.clone().unwrap_or_else(|| String::from("Something went wrong"))
		} else {
			e
		}
	});
	let busy = if pending { "true" } else { "false" };
	let danger = tone == ConfirmDialogTone::Danger;
	let confirm_variant = if danger { ButtonVariant::Danger } else { ButtonVariant::Solid };
	let cancel_cls = button_classes(&ButtonVariant::Ghost, Size::Md, false, None, CONFIRM_DIALOG_PENDING);
	let confirm_cls = button_classes(&confirm_variant, Size::Md, false, None, CONFIRM_DIALOG_PENDING);
	let icon_cls = cn!(CONFIRM_DIALOG_ICON, tone.as_class());
	let glyph = icon.unwrap_or_else(|| rsx! { AlertIcon {} });
	let cancel_label = labels.cancel.unwrap_or_else(|| String::from("Cancel"));
	let cls = cn!(CONFIRM_DIALOG_CONTENT, class);
	rsx! {
		AlertDialog { open: is_open, on_open_change: change,
			if let Some(trigger) = trigger {
				AlertDialogTrigger { {trigger} }
			}
			AlertDialogContent { aria_labelledby: title_id.clone(), aria_describedby: described_by, class: cls,
				AlertDialogHeader { class: CONFIRM_DIALOG_HEADER,
					span { class: icon_cls, "data-slot": "confirm-dialog-icon", "data-tone": tone.as_ref(), {glyph} }
					div { class: CONFIRM_DIALOG_TEXT,
						AlertDialogTitle { id: title_id, class: CONFIRM_DIALOG_TITLE, {title} }
						if let Some(description) = description {
							AlertDialogDescription { id: description_id, class: CONFIRM_DIALOG_DESCRIPTION, {description} }
						}
						if let Some(message) = shown_error {
							p { role: "alert", class: CONFIRM_DIALOG_ERROR, "data-slot": "confirm-dialog-error", {message} }
						}
					}
				}
				AlertDialogFooter { class: CONFIRM_DIALOG_FOOTER,
					button {
						r#type: "button",
						class: cancel_cls,
						"data-slot": "confirm-dialog-cancel",
						"aria-disabled": busy,
						onclick: move |_| change(false),
						{cancel_label}
					}
					button {
						r#type: "button",
						class: confirm_cls,
						"data-slot": "confirm-dialog-confirm",
						"aria-disabled": busy,
						"aria-busy": busy,
						onclick: confirm,
						if pending {
							Spinner { class: CONFIRM_DIALOG_SPINNER }
						}
						{confirm_label}
					}
				}
			}
		}
	}
}

/// The TS `UiIcon` "alert" glyph, in its `size-4` frame.
#[component]
fn AlertIcon() -> Element {
	rsx! {
		svg {
			class: "size-4",
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.75",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			"aria-hidden": "true",
			circle { cx: "12", cy: "12", r: "10" }
			path { d: "M12 8v4M12 16h.01" }
		}
	}
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicUsize, Ordering};

	use dioxus::dioxus_core::ElementId;

	use super::*;
	use crate::uikit::test_util::{Harness, render, render_after_click};

	/// The elements a test presses, read off the mounted dialog: the scrim is
	/// the first click listener, the content the only keydown listener, and the
	/// confirm the only `aria-busy` carrier.
	struct Parts {
		scrim: ElementId,
		content: ElementId,
		cancel: ElementId,
		confirm: ElementId,
	}
	fn parts(h: &Harness) -> Parts {
		let confirm = h.with_attribute("aria-busy")[0];
		let cancel = h
			.with_attribute("aria-disabled")
			.into_iter()
			.find(|id| *id != confirm)
			.unwrap_or_else(|| panic!("no cancel: {}", h.html()));
		Parts {
			scrim: h.listening("click")[0],
			content: h.listening("keydown")[0],
			cancel,
			confirm,
		}
	}
	fn open(html: &str) -> bool {
		html.contains("role=\"alertdialog\"")
	}

	#[test]
	fn is_an_alert_dialog_named_by_its_title_and_described_by_the_text() {
		fn app() -> Element {
			rsx! {
				ConfirmDialog {
					open: true,
					title: rsx! { "Удалить рабочее пространство?" },
					description: rsx! { "Это нельзя отменить." },
					confirm_label: rsx! { "Удалить" },
					on_confirm: |_| {},
				}
			}
		}
		let html = render(app);
		assert!(open(&html), "{html}");
		assert!(html.contains("aria-labelledby=\"confirm-dialog-title-"), "{html}");
		assert!(html.contains("aria-describedby=\"confirm-dialog-description-"), "{html}");
		let named_by = html.split("aria-labelledby=\"").nth(1).and_then(|rest| rest.split('"').next()).unwrap_or_default();
		assert!(html.contains(&format!("<h2 id=\"{named_by}\"")), "the dialog is named by its title: {html}");
		assert!(html.contains("Удалить рабочее пространство?"), "{html}");
		assert!(html.contains("Это нельзя отменить."), "{html}");
		assert!(!html.contains("role=\"alert\""), "no error before a confirm: {html}");
	}

	#[test]
	fn closes_after_a_synchronous_confirm() {
		static CONFIRMS: AtomicUsize = AtomicUsize::new(0);
		static CLOSES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				ConfirmDialog {
					default_open: true,
					on_open_change: |open: bool| {
						if !open {
							CLOSES.fetch_add(1, Ordering::SeqCst);
						}
					},
					title: rsx! { "Точно?" },
					confirm_label: rsx! { "Да" },
					on_confirm: |_| {
						CONFIRMS.fetch_add(1, Ordering::SeqCst);
					},
				}
			}
		}
		let mut h = Harness::new(app);
		let p = parts(&h);
		h.click(p.confirm);
		assert_eq!(CONFIRMS.load(Ordering::SeqCst), 1);
		assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
		assert!(!open(&h.html()), "{}", h.html());
	}

	#[test]
	fn stays_open_and_locked_while_pending_then_closes() {
		static CONFIRMS: AtomicUsize = AtomicUsize::new(0);
		static CLOSES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			let mut busy = use_signal(|| false);
			rsx! {
				ConfirmDialog {
					default_open: true,
					on_open_change: |open: bool| {
						if !open {
							CLOSES.fetch_add(1, Ordering::SeqCst);
						}
					},
					title: rsx! { "Точно?" },
					confirm_label: rsx! { "Да" },
					pending: busy(),
					on_confirm: move |_| {
						CONFIRMS.fetch_add(1, Ordering::SeqCst);
						busy.set(true);
					},
				}
				button { onclick: move |_| busy.set(false), "finish" }
			}
		}
		let mut h = Harness::new(app);
		let finish = *h.listening("click").last().unwrap_or_else(|| panic!("no finish: {}", h.html()));
		let p = parts(&h);
		h.click(p.confirm);
		let html = h.html();
		assert!(open(&html), "{html}");
		assert_eq!(html.matches("aria-disabled=\"true\"").count(), 2, "both buttons lock: {html}");
		assert!(html.contains("aria-busy=\"true\""), "{html}");
		assert!(html.contains("role=\"status\""), "the confirm shows the spinner: {html}");
		assert!(!html.contains(" disabled=\""), "aria-disabled, not disabled, so focus stays on the button: {html}");

		h.key(p.content, Key::Escape);
		h.click(p.scrim);
		h.click(p.cancel);
		h.click(p.confirm);
		assert!(open(&h.html()), "nothing closes it while pending: {}", h.html());
		assert_eq!(CLOSES.load(Ordering::SeqCst), 0);
		assert_eq!(CONFIRMS.load(Ordering::SeqCst), 1, "a second press while pending is ignored");

		h.click(finish);
		assert!(!open(&h.html()), "the dialog closes when the work settles: {}", h.html());
		assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn keeps_the_dialog_open_and_shows_the_error_when_the_work_fails() {
		static CONFIRMS: AtomicUsize = AtomicUsize::new(0);
		static CLOSES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			let mut busy = use_signal(|| false);
			let mut error = use_signal(|| None::<String>);
			rsx! {
				ConfirmDialog {
					default_open: true,
					on_open_change: |open: bool| {
						if !open {
							CLOSES.fetch_add(1, Ordering::SeqCst);
						}
					},
					title: rsx! { "Точно?" },
					confirm_label: rsx! { "Да" },
					pending: busy(),
					error: error(),
					on_confirm: move |_| {
						CONFIRMS.fetch_add(1, Ordering::SeqCst);
						busy.set(true);
					},
				}
				button {
					onclick: move |_| {
						error.set(Some("Сервер отказал".into()));
						busy.set(false);
					},
					"finish"
				}
			}
		}
		let mut h = Harness::new(app);
		let finish = *h.listening("click").last().unwrap_or_else(|| panic!("no finish: {}", h.html()));
		let p = parts(&h);
		h.click(p.confirm);
		h.click(finish);
		let html = h.html();
		assert!(open(&html), "a failure keeps the dialog open: {html}");
		assert!(html.contains("role=\"alert\""), "{html}");
		assert!(html.contains("Сервер отказал"), "{html}");
		assert!(html.contains("aria-busy=\"false\""), "the confirm is live again: {html}");
		assert!(!html.contains("aria-disabled=\"true\""), "{html}");
		assert_eq!(CLOSES.load(Ordering::SeqCst), 0);
		assert_eq!(CONFIRMS.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn an_empty_error_falls_back_to_the_label() {
		fn app() -> Element {
			let mut busy = use_signal(|| false);
			let mut error = use_signal(|| None::<String>);
			rsx! {
				ConfirmDialog {
					default_open: true,
					title: rsx! { "Точно?" },
					confirm_label: rsx! { "Да" },
					pending: busy(),
					error: error(),
					labels: ConfirmDialogLabels { error: Some("Что-то пошло не так".into()), ..Default::default() },
					on_confirm: move |_| busy.set(true),
				}
				button {
					onclick: move |_| {
						error.set(Some(String::new()));
						busy.set(false);
					},
					"finish"
				}
			}
		}
		let mut h = Harness::new(app);
		let finish = *h.listening("click").last().unwrap_or_else(|| panic!("no finish: {}", h.html()));
		let p = parts(&h);
		h.click(p.confirm);
		h.click(finish);
		assert!(h.html().contains("Что-то пошло не так"), "{}", h.html());
	}

	#[test]
	fn drops_a_stale_error_when_a_controlled_parent_reopens_the_dialog() {
		fn app() -> Element {
			let mut open = use_signal(|| true);
			let mut busy = use_signal(|| false);
			let mut error = use_signal(|| None::<String>);
			rsx! {
				ConfirmDialog {
					open: open(),
					title: rsx! { "Точно?" },
					confirm_label: rsx! { "Да" },
					pending: busy(),
					error: error(),
					on_confirm: move |_| busy.set(true),
				}
				button {
					onclick: move |_| {
						error.set(Some("Нет".into()));
						busy.set(false);
					},
					"finish"
				}
				button { onclick: move |_| open.toggle(), "toggle" }
			}
		}
		let mut h = Harness::new(app);
		let clicks = h.listening("click");
		let (finish, toggle) = (clicks[clicks.len() - 2], clicks[clicks.len() - 1]);
		let p = parts(&h);
		h.click(p.confirm);
		h.click(finish);
		assert!(h.html().contains("role=\"alert\""), "{}", h.html());
		h.click(toggle);
		assert!(!open(&h.html()), "{}", h.html());
		h.click(toggle);
		let html = h.html();
		assert!(open(&html), "{html}");
		assert!(!html.contains("role=\"alert\""), "a reopened dialog drops the error: {html}");
	}

	#[test]
	fn uses_the_danger_face_for_the_confirm_button() {
		fn app() -> Element {
			rsx! {
				ConfirmDialog {
					open: true,
					tone: ConfirmDialogTone::Danger,
					title: rsx! { "Удалить?" },
					confirm_label: rsx! { "Удалить" },
					on_confirm: |_| {},
				}
			}
		}
		let html = render(app);
		let confirm = html
			.split("data-slot=\"confirm-dialog-confirm\"")
			.next()
			.and_then(|h| h.rsplit("<button").next())
			.unwrap_or_default();
		assert!(confirm.contains("bg-button-danger-bg"), "{confirm}");
		assert!(html.contains("data-tone=\"danger\""), "{html}");
		assert!(html.contains("bg-chip-error-bg"), "the glyph tile follows the tone: {html}");
	}

	#[test]
	fn the_default_tone_confirms_with_the_solid_face() {
		fn app() -> Element {
			rsx! {
				ConfirmDialog { open: true, title: rsx! { "Точно?" }, confirm_label: rsx! { "Да" }, on_confirm: |_| {} }
			}
		}
		let html = render(app);
		assert!(html.contains("bg-primary text-on-primary"), "{html}");
		assert!(!html.contains("bg-button-danger-bg"), "{html}");
		assert!(html.contains(">Cancel<"), "English default: {html}");
	}

	#[test]
	fn cancels_without_confirming() {
		static CONFIRMS: AtomicUsize = AtomicUsize::new(0);
		static CLOSES: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				ConfirmDialog {
					default_open: true,
					on_open_change: |open: bool| {
						if !open {
							CLOSES.fetch_add(1, Ordering::SeqCst);
						}
					},
					title: rsx! { "Точно?" },
					confirm_label: rsx! { "Да" },
					labels: ConfirmDialogLabels { cancel: Some("Отмена".into()), ..Default::default() },
					on_confirm: |_| {
						CONFIRMS.fetch_add(1, Ordering::SeqCst);
					},
				}
			}
		}
		let mut h = Harness::new(app);
		assert!(h.html().contains(">Отмена<"), "{}", h.html());
		let p = parts(&h);
		h.click(p.cancel);
		assert_eq!(CLOSES.load(Ordering::SeqCst), 1);
		assert_eq!(CONFIRMS.load(Ordering::SeqCst), 0);
		assert!(!open(&h.html()), "{}", h.html());
	}

	#[test]
	fn the_trigger_opens_it() {
		fn app() -> Element {
			rsx! {
				ConfirmDialog { trigger: rsx! { "Удалить" }, title: rsx! { "Точно?" }, confirm_label: rsx! { "Да" }, on_confirm: |_| {} }
			}
		}
		assert!(!open(&render(app)));
		assert!(open(&render_after_click(app)), "{}", render_after_click(app));
	}
}
