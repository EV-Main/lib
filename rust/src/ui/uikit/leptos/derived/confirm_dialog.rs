use leptos::prelude::*;

pub use crate::ui::uikit::core::ConfirmDialogLabels;
use crate::{
	cn,
	ui::uikit::{
		core::ConfirmFlow,
		leptos::{
			AlertDialog, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger, ButtonVariant, CONFIRM_DIALOG_CONTENT,
			CONFIRM_DIALOG_DESCRIPTION, CONFIRM_DIALOG_ERROR, CONFIRM_DIALOG_FOOTER, CONFIRM_DIALOG_HEADER, CONFIRM_DIALOG_ICON, CONFIRM_DIALOG_PENDING, CONFIRM_DIALOG_SPINNER,
			CONFIRM_DIALOG_TEXT, CONFIRM_DIALOG_TITLE, ConfirmDialogTone, Size, Spinner, button_classes,
			icon_button::Glyph,
			primitives::{use_controllable, use_stable_id},
		},
	},
};

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
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	#[prop(optional, into)] trigger: Option<ViewFn>,
	/// Fused over the secondary button face the trigger wears: the TS caller
	/// passes a `Button`, and Rust has no `asChild` to take its place.
	#[prop(optional, into)]
	trigger_class: String,
	/// The heading; it also names the dialog.
	#[prop(into)]
	title: ViewFn,
	#[prop(optional, into)] description: Option<ViewFn>,
	#[prop(optional)] tone: ConfirmDialogTone,
	#[prop(into)] confirm_label: ViewFn,
	on_confirm: Callback<()>,
	#[prop(optional, into)] pending: Signal<bool>,
	#[prop(optional, into)] error: Signal<Option<String>>,
	/// Replaces the tone's glyph.
	#[prop(optional, into)]
	icon: Option<ViewFn>,
	#[prop(optional)] labels: ConfirmDialogLabels,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let flow = RwSignal::new(ConfirmFlow::default());
	Effect::new(move |_| {
		if state.get() && flow.with_untracked(ConfirmFlow::shows_error) {
			flow.update(ConfirmFlow::opened);
		}
	});
	Effect::new(move |_| {
		// Subscribed to `flow`, so a confirm whose work never sets `pending` settles too.
		let current = flow.get();
		let mut next = current;
		let close = next.settle(pending.get(), error.with(Option::is_some));
		if next != current {
			flow.set(next);
		}
		if close {
			state.set(false);
		}
	});
	let change = Callback::new(move |next: bool| {
		let mut allowed = false;
		flow.update(|f| allowed = f.change(next, pending.get_untracked()));
		if allowed {
			state.set(next);
		}
	});
	let confirm = move |_| {
		let mut start = false;
		flow.update(|f| start = f.confirm(pending.get_untracked()));
		if start {
			on_confirm.run(());
		}
	};

	let title_id = use_stable_id("confirm-dialog-title");
	let description_id = use_stable_id("confirm-dialog-description");
	let described_by = description.as_ref().map(|_| description_id.clone());
	let error_label = labels.error.unwrap_or_else(|| String::from("Something went wrong"));
	let shown_error = move || {
		error.get().filter(|_| flow.with(ConfirmFlow::shows_error)).map(|e| {
			let message = if e.is_empty() { error_label.clone() } else { e };
			view! {
				<p role="alert" class=CONFIRM_DIALOG_ERROR data-slot="confirm-dialog-error">
					{message}
				</p>
			}
		})
	};
	let busy = move || pending.get().to_string();
	let danger = tone == ConfirmDialogTone::Danger;
	let confirm_variant = if danger { ButtonVariant::Destructive } else { ButtonVariant::Primary };
	let cancel_cls = button_classes(&ButtonVariant::Ghost, Size::Xs, false, None, CONFIRM_DIALOG_PENDING);
	let confirm_cls = button_classes(&confirm_variant, Size::Xs, false, None, CONFIRM_DIALOG_PENDING);
	let icon_cls = cn!(CONFIRM_DIALOG_ICON, tone.as_class());
	let cancel_label = labels.cancel.unwrap_or_else(|| String::from("Cancel"));
	let cls = cn!(CONFIRM_DIALOG_CONTENT, class);
	let trigger_cls = button_classes(&ButtonVariant::Outline, Size::Xs, false, None, &trigger_class);
	let labelled_by = title_id.clone();
	// Run on every opening, so it clones what the inner parts take by value.
	let body = move || {
		let (icon, title, description, confirm_label) = (icon.clone(), title.clone(), description.clone(), confirm_label.clone());
		let (title_id, description_id, cancel_label, shown_error) = (title_id.clone(), description_id.clone(), cancel_label.clone(), shown_error.clone());
		let (icon_cls, cancel_cls, confirm_cls) = (icon_cls.clone(), cancel_cls.clone(), confirm_cls.clone());
		let glyph = match icon {
			Some(icon) => icon.run(),
			None => view! {
				<Glyph class="size-4">
					<circle cx="12" cy="12" r="10" />
					<path d="M12 8v4M12 16h.01" />
				</Glyph>
			}
			.into_any(),
		};
		view! {
			<AlertDialogHeader class=CONFIRM_DIALOG_HEADER>
				<span class=icon_cls data-slot="confirm-dialog-icon" data-tone=tone.as_ref().to_owned()>
					{glyph}
				</span>
				<div class=CONFIRM_DIALOG_TEXT>
					<AlertDialogTitle id=title_id class=CONFIRM_DIALOG_TITLE>
						{title.run()}
					</AlertDialogTitle>
					{description
						.map(|d| {
							view! {
								<AlertDialogDescription id=description_id class=CONFIRM_DIALOG_DESCRIPTION>
									{d.run()}
								</AlertDialogDescription>
							}
						})}
					{shown_error}
				</div>
			</AlertDialogHeader>
			<AlertDialogFooter class=CONFIRM_DIALOG_FOOTER>
				<button
					type="button"
					class=cancel_cls
					data-slot="confirm-dialog-cancel"
					aria-disabled=busy
					on:click=move |_| change.run(false)
				>
					{cancel_label}
				</button>
				<button
					type="button"
					class=confirm_cls
					data-slot="confirm-dialog-confirm"
					aria-disabled=busy
					aria-busy=busy
					on:click=confirm
				>
					{move || pending.get().then(|| view! { <Spinner class=CONFIRM_DIALOG_SPINNER /> })}
					{confirm_label.run()}
				</button>
			</AlertDialogFooter>
		}
	};
	view! {
		<AlertDialog open=Signal::derive(move || state.get()) on_open_change=change>
			{trigger
				.map(|t| view! { <AlertDialogTrigger class=trigger_cls>{t.run()}</AlertDialogTrigger> })}
			<AlertDialogContent aria_labelledby=labelled_by aria_describedby=described_by class=cls>
				{body()}
			</AlertDialogContent>
		</AlertDialog>
	}
}
