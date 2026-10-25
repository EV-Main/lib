use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		Button, ButtonVariant, POPOVER_FORM_ACTIONS, POPOVER_FORM_CONTENT, POPOVER_FORM_FIELDS, POPOVER_FORM_FORM, POPOVER_FORM_TITLE, Popover, PopoverContent, PopoverTrigger, Size,
		button_classes,
		primitives::{use_controllable, use_stable_id},
	},
};

/// The form's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PopoverFormLabels {
	/// The submit button; "Apply".
	pub submit: Option<String>,
	/// The reset button; "Reset".
	pub reset: Option<String>,
}

/// A filter-style form in a popover: a title, fields, Reset and Apply.
/// Mirrors the TS `PopoverForm`.
///
/// `trigger` is the content of the opening button, not a button of its own:
/// the kit has no `asChild` slot, so [`PopoverTrigger`] renders the button.
///
/// It is a [`Popover`], so it keeps the kit's overlay limits: rendered inline,
/// no portal and no focus trap.
#[component]
pub fn PopoverForm(
	open: Option<bool>,
	#[props(default)] default_open: bool,
	on_open_change: Option<EventHandler<bool>>,
	trigger: Element,
	/// Fused over the secondary button face the trigger wears: the TS caller
	/// passes a `Button`, and Rust has no `asChild` to take its place.
	#[props(default)]
	trigger_class: String,
	/// The heading; it also names the form's dialog.
	title: Element,
	/// Called on submit; the popover closes afterwards.
	on_submit: Option<EventHandler<FormEvent>>,
	/// Called when Reset is pressed; the browser resets native fields first.
	on_reset: Option<EventHandler<()>>,
	#[props(default)] labels: PopoverFormLabels,
	#[props(default)] class: String,
	/// The fields: `Field`s with an `Input`, `Select` or `Checkbox`.
	children: Element,
) -> Element {
	// Owned here, not left to the `Popover`, because submitting has to close it
	// and the popover's state is private to its parts.
	let state = use_controllable(open, default_open, on_open_change);
	let title_id = use_stable_id("popover-form-title");
	let cls = cn!(POPOVER_FORM_CONTENT, class);
	let trigger_cls = button_classes(&ButtonVariant::Secondary, Size::Md, false, None, &trigger_class);
	let submit_label = labels.submit.unwrap_or_else(|| String::from("Apply"));
	let reset_label = labels.reset.unwrap_or_else(|| String::from("Reset"));
	rsx! {
		Popover { open: state.get(), on_open_change: move |next| state.set(next),
			PopoverTrigger { class: trigger_cls, {trigger} }
			PopoverContent { align: "start", role: "dialog", aria_labelledby: title_id.clone(), class: cls,
				form {
					class: POPOVER_FORM_FORM,
					"data-slot": "popover-form",
					onsubmit: move |e: FormEvent| {
						e.prevent_default();
						if let Some(h) = on_submit {
							h.call(e);
						}
						state.set(false);
					},
					onreset: move |_| {
						if let Some(h) = on_reset {
							h.call(());
						}
					},
					h3 { id: title_id, class: POPOVER_FORM_TITLE, "data-slot": "popover-form-title", {title} }
					div { class: POPOVER_FORM_FIELDS, "data-slot": "popover-form-fields", {children} }
					div { class: POPOVER_FORM_ACTIONS, "data-slot": "popover-form-actions",
						Button { r#type: "reset", variant: ButtonVariant::Ghost, size: Size::Sm, {reset_label} }
						Button { r#type: "submit", variant: ButtonVariant::Primary, size: Size::Sm, {submit_label} }
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicUsize, Ordering};

	use super::*;
	use crate::uikit::test_util::{Harness, render, render_after_click};

	#[test]
	fn the_trigger_wears_the_secondary_button_face() {
		fn app() -> Element {
			rsx! {
				PopoverForm { trigger: rsx! { "Фильтр" }, trigger_class: "w-40", title: rsx! { "Фильтр статуса" }, "поля" }
			}
		}
		let html = render(app);
		let trigger = html.split("data-slot=\"popover-trigger\"").next().expect("a trigger");
		assert!(trigger.contains("bg-secondary"), "{html}");
		assert!(trigger.contains("w-40"), "the override joins the face: {html}");
	}

	#[test]
	fn stays_closed_until_the_trigger_is_pressed() {
		fn app() -> Element {
			rsx! {
				PopoverForm { trigger: rsx! { "Фильтр" }, title: rsx! { "Фильтр статуса" },
					label { "Только избранное" input { r#type: "checkbox" } }
				}
			}
		}
		let closed = render(app);
		assert!(!closed.contains("role=\"dialog\""), "{closed}");
		assert!(closed.contains("aria-expanded=\"false\""), "{closed}");

		let html = render_after_click(app);
		assert!(html.contains("role=\"dialog\""), "{html}");
		let named_by = html.split("aria-labelledby=\"").nth(1).and_then(|rest| rest.split('"').next());
		let named_by = named_by.unwrap_or_else(|| panic!("the dialog is unnamed: {html}"));
		assert!(html.contains(&format!("<h3 id=\"{named_by}\"")), "the dialog is named by its title: {html}");
		assert!(html.contains("Фильтр статуса"), "{html}");
	}

	#[test]
	fn submits_reports_it_and_closes() {
		static SUBMITS: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				PopoverForm {
					default_open: true,
					trigger: rsx! { "Фильтр" },
					title: rsx! { "Фильтр статуса" },
					on_submit: |_| {
						SUBMITS.fetch_add(1, Ordering::SeqCst);
					},
					"поля"
				}
			}
		}
		let mut h = Harness::new(app);
		let form = h.listening("submit")[0];
		h.form("submit", form);
		assert_eq!(SUBMITS.load(Ordering::SeqCst), 1);
		let html = h.html();
		assert!(!html.contains("role=\"dialog\""), "the popover closes after a submit: {html}");
		assert!(html.contains("aria-expanded=\"false\""), "{html}");
	}

	#[test]
	fn resets_without_closing() {
		static RESETS: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! {
				PopoverForm {
					default_open: true,
					trigger: rsx! { "Фильтр" },
					title: rsx! { "Фильтр статуса" },
					on_reset: |_| {
						RESETS.fetch_add(1, Ordering::SeqCst);
					},
					"поля"
				}
			}
		}
		let mut h = Harness::new(app);
		let form = h.listening("reset")[0];
		h.form("reset", form);
		assert_eq!(RESETS.load(Ordering::SeqCst), 1);
		let html = h.html();
		assert!(html.contains("role=\"dialog\""), "a reset keeps the popover open: {html}");
	}

	#[test]
	fn the_buttons_reset_and_submit_the_form() {
		fn app() -> Element {
			rsx! {
				PopoverForm { default_open: true, trigger: rsx! { "Фильтр" }, title: rsx! { "Фильтр статуса" }, "поля" }
			}
		}
		let html = render(app);
		assert!(html.contains("type=\"reset\""), "{html}");
		assert!(html.contains("type=\"submit\""), "{html}");
		assert!(html.contains(">Reset<") && html.contains(">Apply<"), "English defaults: {html}");
	}

	#[test]
	fn takes_translated_button_labels() {
		fn app() -> Element {
			rsx! {
				PopoverForm {
					default_open: true,
					trigger: rsx! { "Фильтр" },
					title: rsx! { "Фильтр статуса" },
					labels: PopoverFormLabels {
						submit: Some("Применить".into()),
						reset: Some("Сбросить".into()),
					},
					"поля"
				}
			}
		}
		let html = render(app);
		assert!(html.contains(">Применить<"), "{html}");
		assert!(html.contains(">Сбросить<"), "{html}");
		assert!(!html.contains("Apply") && !html.contains("Reset<"), "{html}");
	}
}
