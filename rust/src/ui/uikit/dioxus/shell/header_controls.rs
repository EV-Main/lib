use dioxus::prelude::*;
use ev_lib_classes::{Icon, copy};

pub use crate::ui::uikit::core::HeaderNotification;
use crate::{
	cn,
	ui::uikit::dioxus::{
		HEADER_BALANCE_BASE, HEADER_BALANCE_CHEVRON, HEADER_BALANCE_FIGURE, HEADER_BALANCE_INTERACTIVE, HEADER_BALANCE_LABEL, HEADER_BALANCE_SKELETON, HEADER_SEARCH_BASE,
		HEADER_SEARCH_PLACEHOLDER, ICON_BUTTON_BASE, IconButtonVariant, Kbd, NOTIFICATION_BELL_CONTENT, NOTIFICATION_BELL_EMPTY, NOTIFICATION_BELL_FOOTER, NOTIFICATION_BELL_ITEM,
		NOTIFICATION_BELL_ITEM_BODY, NOTIFICATION_BELL_ITEM_DESCRIPTION, NOTIFICATION_BELL_ITEM_DOT, NOTIFICATION_BELL_ITEM_DOT_READ, NOTIFICATION_BELL_ITEM_DOT_UNREAD,
		NOTIFICATION_BELL_ITEM_TIME, NOTIFICATION_BELL_ITEM_TITLE, NOTIFICATION_BELL_ITEM_TITLE_UNREAD, NOTIFICATION_BELL_LIST, NOTIFICATION_BELL_TRIGGER, NOTIFICATION_DOT, Popover,
		PopoverContent, PopoverTrigger, Size, Skeleton, Slot, Tone, button_classes, icon_button_size_class, primitives::icon_shapes,
	},
};

/// "Balance: $0.00" in the header; a button only when given `onclick`.
/// Mirrors the TS `HeaderBalance`.
///
/// `value` is the figure already formatted by the caller. The TS `amount`,
/// `format` and `locales` roll the figure through `AnimatedNumber`, which the
/// Dioxus kit does not have, and the TS `Settled` reveal is gone with it: the
/// figure simply replaces the skeleton.
#[component]
pub fn HeaderBalance(
	/// Before the figure; "Balance:".
	label: Option<Element>,
	/// The figure, already formatted: "$0.00".
	value: String,
	/// True until the balance is known; a skeleton stands in for the figure.
	#[props(default)]
	loading: bool,
	/// A glyph before the label.
	icon: Option<Element>,
	#[props(default)] tone: Tone,
	/// Makes the balance a button, with a chevron after the figure.
	onclick: Option<EventHandler<MouseEvent>>,
	/// Only meaningful with `onclick`.
	#[props(default)]
	disabled: bool,
	/// The TS port forwards any attribute; this one names the figure for a
	/// screen reader when the label alone is not enough.
	aria_label: Option<String>,
	#[props(default)] class: String,
) -> Element {
	let cls = cn!(HEADER_BALANCE_BASE, tone.as_class(), if onclick.is_some() { HEADER_BALANCE_INTERACTIVE } else { "" }, class);
	let label = label.unwrap_or_else(|| rsx! { {copy::HEADER_BALANCE} });
	let body = rsx! {
		{icon}
		span { class: HEADER_BALANCE_LABEL, {label} }
		if loading {
			Skeleton { class: HEADER_BALANCE_SKELETON }
		} else {
			span { class: HEADER_BALANCE_FIGURE, "data-slot": Slot::HeaderBalanceValue.as_str(), {value} }
		}
	};
	let Some(onclick) = onclick else {
		return rsx! {
			div { class: cls, "data-slot": Slot::HeaderBalance.as_str(), "aria-label": aria_label, {body} }
		};
	};
	rsx! {
		button {
			r#type: "button",
			class: cls,
			"data-slot": Slot::HeaderBalance.as_str(),
			"aria-label": aria_label,
			disabled,
			onclick: move |e| onclick.call(e),
			{body}
			Glyph { class: HEADER_BALANCE_CHEVRON,
				{icon_shapes(Icon::ChevronDown)}
			}
		}
	}
}

/// The bell's own strings; every one defaults to English.
#[derive(Clone, PartialEq)]
pub struct NotificationBellLabels {
	/// The bell's name.
	pub trigger: String,
	/// Spoken after the name when something is unread; gets the count.
	/// "{n} unread". A callback, not a template, so a language can pick its
	/// plural form by the count.
	pub unread: Option<Callback<usize, String>>,
	/// Shown in place of an empty list.
	pub empty: String,
}
impl Default for NotificationBellLabels {
	fn default() -> Self {
		Self {
			trigger: copy::NOTIFICATION_BELL_TRIGGER.to_owned(),
			unread: None,
			empty: copy::NOTIFICATION_BELL_EMPTY.to_owned(),
		}
	}
}

/// The bell with an unread dot; the list opens in a popover. Mirrors the TS
/// `NotificationBell`.
///
/// The unread count is in the bell's name, not only in the dot: the dot is
/// `aria-hidden`, so a screen reader hears "Notifications, 2 unread". The
/// popover keeps the kit's overlay limits: inline, no portal, no focus trap;
/// Escape and an outside click close it.
#[component]
pub fn NotificationBell(
	notifications: Vec<HeaderNotification>,
	on_select: Option<EventHandler<String>>,
	#[props(default)] labels: NotificationBellLabels,
	/// Under the list: "View all".
	footer: Option<Element>,
	#[props(default)] class: String,
) -> Element {
	let unread = notifications.iter().filter(|n| n.unread).count();
	let name = labels.trigger;
	let spoken = match (unread, labels.unread) {
		(0, _) => name,
		(n, Some(say)) => format!("{name}, {}", say.call(n)),
		(n, None) => format!("{name}, {n} unread"),
	};
	// The trigger is the popover's own button wearing the outline icon
	// button face: the kit has no `asChild` to hand the toggle to an `IconButton`.
	let trigger_cls = cn!(
		button_classes(&IconButtonVariant::Outline.button_variant(), Size::Md, true, None, ""),
		icon_button_size_class(Size::Md),
		ICON_BUTTON_BASE,
		NOTIFICATION_BELL_TRIGGER,
		class
	);
	let empty = labels.empty;
	rsx! {
		Popover {
			PopoverTrigger { class: trigger_cls, aria_label: spoken,
				Glyph { class: "size-4",
					{icon_shapes(Icon::Bell)}
				}
				if unread > 0 {
					span { class: NOTIFICATION_DOT, "data-slot": Slot::NotificationDot.as_str(), "aria-hidden": "true" }
				}
			}
			PopoverContent { align: "end", class: NOTIFICATION_BELL_CONTENT,
				if notifications.is_empty() {
					p { class: NOTIFICATION_BELL_EMPTY, {empty} }
				} else {
					ul { class: NOTIFICATION_BELL_LIST,
						for n in notifications {
							NotificationRow { key: "{n.id}", notification: n, on_select }
						}
					}
				}
				if let Some(footer) = footer {
					div { class: NOTIFICATION_BELL_FOOTER, {footer} }
				}
			}
		}
	}
}

/// A button dressed as the search field; it opens the palette rather than
/// taking input. Mirrors the TS `HeaderSearch`.
///
/// The hint reads "Ctrl K" on the server and on the first client render, so
/// the two agree; on Apple platforms the `wasm` build then switches it to "⌘K".
#[component]
pub fn HeaderSearch(
	/// The field text; "Search or jump to…".
	placeholder: Option<String>,
	/// The shortcut hint at the end. Default: "⌘K" on Apple platforms, "Ctrl K"
	/// elsewhere.
	shortcut: Option<String>,
	onclick: Option<EventHandler<MouseEvent>>,
	#[props(default)] class: String,
) -> Element {
	let platform = use_platform_shortcut();
	let cls = cn!(HEADER_SEARCH_BASE, class);
	let placeholder = placeholder.unwrap_or_else(|| copy::HEADER_SEARCH.to_owned());
	let hint = shortcut.unwrap_or_else(|| String::from(platform()));
	rsx! {
		button {
			r#type: "button",
			class: cls,
			"data-slot": Slot::HeaderSearch.as_str(),
			onclick: move |e| {
				if let Some(h) = onclick {
					h.call(e);
				}
			},
			Glyph { class: "size-4",
				{icon_shapes(Icon::Search)}
			}
			span { class: HEADER_SEARCH_PLACEHOLDER, {placeholder} }
			Kbd { {hint} }
		}
	}
}

#[component]
fn NotificationRow(notification: HeaderNotification, on_select: Option<EventHandler<String>>) -> Element {
	let HeaderNotification {
		id,
		title,
		description,
		time,
		unread,
	} = notification;
	let dot_cls = cn!(
		NOTIFICATION_BELL_ITEM_DOT,
		if unread { NOTIFICATION_BELL_ITEM_DOT_UNREAD } else { NOTIFICATION_BELL_ITEM_DOT_READ }
	);
	let title_cls = cn!(NOTIFICATION_BELL_ITEM_TITLE, if unread { NOTIFICATION_BELL_ITEM_TITLE_UNREAD } else { "" });
	rsx! {
		li {
			button {
				r#type: "button",
				class: NOTIFICATION_BELL_ITEM,
				"data-slot": Slot::Notification.as_str(),
				onclick: move |_| {
					if let Some(h) = on_select {
						h.call(id.clone());
					}
				},
				span { class: dot_cls, "aria-hidden": "true" }
				span { class: NOTIFICATION_BELL_ITEM_BODY,
					span { class: title_cls, {title} }
					if let Some(description) = description {
						span { class: NOTIFICATION_BELL_ITEM_DESCRIPTION, {description} }
					}
				}
				if let Some(time) = time {
					span { class: NOTIFICATION_BELL_ITEM_TIME, {time} }
				}
			}
		}
	}
}

/// Read after mount, so the server render and the first client render agree.
fn use_platform_shortcut() -> Signal<&'static str> {
	let shortcut = use_signal(|| copy::CTRL_K);
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	{
		let mut shortcut = shortcut;
		use_effect(move || {
			let next = crate::ui::uikit::host::platform_shortcut();
			if *shortcut.peek() != next {
				shortcut.set(next);
			}
		});
	}
	shortcut
}

/// The TS `UiIcon` frame: a 24-unit lucide glyph drawn in the current ink.
#[component]
fn Glyph(class: &'static str, children: Element) -> Element {
	rsx! {
		svg {
			class,
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "1.75",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			"aria-hidden": "true",
			{children}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::sync::{
		Mutex,
		atomic::{AtomicUsize, Ordering},
	};

	use super::*;
	use crate::ui::uikit::dioxus::test_util::{Harness, render, render_after_click};

	#[test]
	fn the_balance_shows_a_skeleton_while_loading_then_the_figure() {
		fn loading() -> Element {
			rsx! { HeaderBalance { value: "12,00 ₽", loading: true } }
		}
		fn known() -> Element {
			rsx! { HeaderBalance { value: "12,00 ₽" } }
		}
		let html = render(loading);
		assert!(!html.contains("12,00 ₽"), "{html}");
		assert!(html.contains("data-slot=\"skeleton\""), "{html}");
		let html = render(known);
		assert!(html.contains("12,00 ₽"), "{html}");
		assert!(!html.contains("data-slot=\"skeleton\""), "{html}");
	}

	#[test]
	fn the_balance_is_a_button_only_when_it_does_something() {
		static CLICKS: AtomicUsize = AtomicUsize::new(0);
		fn still() -> Element {
			rsx! { HeaderBalance { value: "$0.00" } }
		}
		fn pressable() -> Element {
			rsx! { HeaderBalance { value: "$0.00", onclick: |_| { CLICKS.fetch_add(1, Ordering::SeqCst); } } }
		}
		let html = render(still);
		assert!(!html.contains("<button"), "{html}");
		assert!(!html.contains("cursor-pointer"), "{html}");

		let mut harness = Harness::new(pressable);
		let html = harness.html();
		assert!(html.starts_with("<button type=\"button\""), "{html}");
		assert!(html.contains("cursor-pointer"), "{html}");
		assert!(html.contains("d=\"m6 9 6 6 6-6\""), "a pressable balance shows its chevron: {html}");
		let clicks = harness.listening("click");
		assert_eq!(clicks.len(), 1, "{html}");
		harness.click(clicks[0]);
		assert_eq!(CLICKS.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn the_balance_inks_the_figure_in_its_tone() {
		fn app() -> Element {
			rsx! { HeaderBalance { value: "$0.00", tone: Tone::Warn } }
		}
		let html = render(app);
		assert!(html.contains(tailwind_fuse::AsTailwindClass::as_class(&Tone::Warn)), "{html}");
		let figure = html.split("data-slot=\"header-balance-value\"").next().and_then(|b| b.rsplit("<span").next()).expect("figure");
		assert!(figure.contains("text-(color:--tone-ink)"), "{figure}");
	}

	#[test]
	fn the_balance_takes_a_translated_label_and_a_name() {
		fn app() -> Element {
			rsx! { HeaderBalance { label: rsx! { "Баланс:" }, value: "1 234,50 ₽", aria_label: "Баланс счёта" } }
		}
		let html = render(app);
		assert!(html.contains("Баланс:"), "{html}");
		assert!(!html.contains("Balance:"), "{html}");
		assert!(html.contains("aria-label=\"Баланс счёта\""), "{html}");
	}

	#[test]
	fn a_disabled_balance_button_says_so() {
		fn app() -> Element {
			rsx! { HeaderBalance { value: "$0.00", disabled: true, onclick: |_| {} } }
		}
		let html = render(app);
		assert!(html.contains("disabled=true"), "{html}");
	}

	fn items() -> Vec<HeaderNotification> {
		vec![
			HeaderNotification {
				id: "a".into(),
				title: "Заявка исполнена".into(),
				unread: true,
				..Default::default()
			},
			HeaderNotification {
				id: "b".into(),
				title: "Выписка готова".into(),
				description: Some("За октябрь".into()),
				time: Some("2 мин назад".into()),
				..Default::default()
			},
		]
	}

	#[test]
	fn the_bell_is_named_with_the_unread_count_and_draws_the_dot() {
		fn app() -> Element {
			rsx! { NotificationBell { notifications: items() } }
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Notifications, 1 unread\""), "{html}");
		assert!(html.contains("data-slot=\"notification-dot\""), "{html}");
		assert!(html.contains("aria-expanded=\"false\""), "{html}");
	}

	#[test]
	fn the_bell_has_no_dot_and_a_plain_name_when_everything_is_read() {
		fn app() -> Element {
			rsx! { NotificationBell { notifications: vec![HeaderNotification { id: "b".into(), title: "Старое".into(), ..Default::default() }] } }
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Notifications\""), "{html}");
		assert!(!html.contains("notification-dot"), "{html}");
	}

	#[test]
	fn the_bell_speaks_the_count_in_the_callers_language() {
		fn app() -> Element {
			let unread = Callback::new(|n: usize| format!("{n} непрочитанных"));
			let mut list = items();
			list[1].unread = true;
			rsx! {
				NotificationBell {
					notifications: list,
					labels: NotificationBellLabels { trigger: "Уведомления".into(), unread: Some(unread), ..Default::default() },
				}
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Уведомления, 2 непрочитанных\""), "{html}");
	}

	#[test]
	fn the_bell_lists_notifications_in_a_popover_and_reports_the_pick() {
		static PICKED: Mutex<Vec<String>> = Mutex::new(Vec::new());
		fn app() -> Element {
			rsx! {
				NotificationBell {
					notifications: items(),
					on_select: |id: String| PICKED.lock().expect("picks").push(id),
					footer: rsx! { a { href: "#", "Все уведомления" } },
				}
			}
		}
		let mut harness = Harness::new(app);
		assert!(!harness.html().contains("Заявка исполнена"), "the list waits for the bell");
		let bell = harness.listening("click");
		assert_eq!(bell.len(), 1, "{}", harness.html());
		harness.click(bell[0]);
		let html = harness.html();
		assert!(html.contains("aria-expanded=\"true\""), "{html}");
		assert!(html.contains("data-align=\"end\""), "{html}");
		assert!(html.contains("Заявка исполнена"), "{html}");
		assert!(html.contains("За октябрь"), "{html}");
		assert!(html.contains("2 мин назад"), "{html}");
		assert!(html.contains("Все уведомления"), "{html}");
		let unread_title = html.split("Заявка исполнена").next().and_then(|b| b.rsplit("<span").next()).expect("title");
		assert!(unread_title.contains("font-medium"), "{unread_title}");
		let read_title = html.split("Выписка готова").next().and_then(|b| b.rsplit("<span").next()).expect("title");
		assert!(!read_title.contains("font-medium"), "{read_title}");

		// Created after the bell: the scrim, then one button per notification.
		let rows: Vec<_> = harness.listening("click").into_iter().filter(|id| !bell.contains(id)).collect();
		let first_row = *rows.get(1).unwrap_or_else(|| panic!("scrim and rows: {rows:?} {html}"));
		harness.click(first_row);
		assert_eq!(*PICKED.lock().expect("picks"), ["a"]);
	}

	#[test]
	fn the_bell_says_so_when_the_list_is_empty() {
		fn app() -> Element {
			rsx! {
				NotificationBell {
					notifications: Vec::new(),
					labels: NotificationBellLabels { empty: "Ничего нового".into(), ..Default::default() },
				}
			}
		}
		let html = render_after_click(app);
		assert!(html.contains("aria-label=\"Notifications\""), "{html}");
		assert!(html.contains("Ничего нового"), "{html}");
		assert!(!html.contains("<ul"), "{html}");
	}

	#[test]
	fn the_bell_wears_the_secondary_icon_button_face() {
		fn app() -> Element {
			rsx! { NotificationBell { notifications: Vec::new(), class: "ml-1" } }
		}
		let html = render(app);
		let trigger = html.split("data-slot=\"popover-trigger\"").next().and_then(|b| b.rsplit("<button").next()).expect("trigger");
		assert!(trigger.contains("bg-secondary"), "{trigger}");
		assert!(trigger.contains("aspect-square"), "{trigger}");
		assert!(trigger.contains("relative"), "{trigger}");
		assert!(trigger.contains("ml-1"), "{trigger}");
	}

	#[test]
	fn search_is_a_button_with_the_placeholder_and_the_shortcut() {
		static CLICKS: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			rsx! { HeaderSearch { onclick: |_| { CLICKS.fetch_add(1, Ordering::SeqCst); } } }
		}
		let mut harness = Harness::new(app);
		let html = harness.html();
		assert!(html.starts_with("<button type=\"button\""), "{html}");
		assert!(html.contains("Search or jump to…"), "{html}");
		assert!(html.contains("data-slot=\"kbd\""), "{html}");
		assert!(html.contains(">Ctrl K</kbd>"), "{html}");
		let clicks = harness.listening("click");
		assert_eq!(clicks.len(), 1, "{html}");
		harness.click(clicks[0]);
		assert_eq!(CLICKS.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn search_truncates_a_long_placeholder_and_keeps_the_hint() {
		fn app() -> Element {
			rsx! { HeaderSearch { placeholder: "Поиск по счетам, заявкам и контрагентам", shortcut: "/" } }
		}
		let html = render(app);
		let text = html.split("Поиск по счетам").next().and_then(|b| b.rsplit("<span").next()).expect("placeholder");
		assert!(text.contains("truncate"), "{text}");
		assert!(text.contains("min-w-0"), "{text}");
		assert!(html.contains(">/</kbd>"), "{html}");
		assert!(!html.contains("Ctrl K"), "{html}");
	}
}
