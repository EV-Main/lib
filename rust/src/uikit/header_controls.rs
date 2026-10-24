use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		HEADER_BALANCE_BASE, HEADER_BALANCE_CHEVRON, HEADER_BALANCE_FIGURE, HEADER_BALANCE_INTERACTIVE, HEADER_BALANCE_LABEL, HEADER_BALANCE_SKELETON, HEADER_SEARCH_BASE,
		HEADER_SEARCH_PLACEHOLDER, HeaderBalanceTone, ICON_BUTTON_BASE, IconButtonVariant, Kbd, NOTIFICATION_BELL_CONTENT, NOTIFICATION_BELL_EMPTY, NOTIFICATION_BELL_FOOTER,
		NOTIFICATION_BELL_ITEM, NOTIFICATION_BELL_ITEM_BODY, NOTIFICATION_BELL_ITEM_DESCRIPTION, NOTIFICATION_BELL_ITEM_DOT, NOTIFICATION_BELL_ITEM_DOT_READ,
		NOTIFICATION_BELL_ITEM_DOT_UNREAD, NOTIFICATION_BELL_ITEM_TIME, NOTIFICATION_BELL_ITEM_TITLE, NOTIFICATION_BELL_ITEM_TITLE_UNREAD, NOTIFICATION_BELL_LIST, NOTIFICATION_BELL_TRIGGER,
		NOTIFICATION_DOT, Popover, PopoverContent, PopoverTrigger, Size, Skeleton, button_classes,
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
	#[props(default)] tone: HeaderBalanceTone,
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
	let cls = cn!(HEADER_BALANCE_BASE, if onclick.is_some() { HEADER_BALANCE_INTERACTIVE } else { "" }, class);
	let label = label.unwrap_or_else(|| rsx! { "Balance:" });
	let figure_cls = cn!(HEADER_BALANCE_FIGURE, tone.as_class());
	let body = rsx! {
		{icon}
		span { class: HEADER_BALANCE_LABEL, {label} }
		if loading {
			Skeleton { class: HEADER_BALANCE_SKELETON }
		} else {
			span { class: figure_cls, "data-slot": "header-balance-value", {value} }
		}
	};
	let Some(onclick) = onclick else {
		return rsx! {
			div { class: cls, "data-slot": "header-balance", "aria-label": aria_label, {body} }
		};
	};
	rsx! {
		button {
			r#type: "button",
			class: cls,
			"data-slot": "header-balance",
			"aria-label": aria_label,
			disabled,
			onclick: move |e| onclick.call(e),
			{body}
			Glyph { class: HEADER_BALANCE_CHEVRON,
				path { d: "m6 9 6 6 6-6" }
			}
		}
	}
}

/// One entry of a [`NotificationBell`]. Mirrors the TS `HeaderNotification`;
/// the TS fields take any node, these take text, like `CommandPaletteItem`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeaderNotification {
	/// Reported to `on_select`.
	pub id: String,
	pub title: String,
	pub description: Option<String>,
	/// Already formatted: "2m ago".
	pub time: Option<String>,
	pub unread: bool,
}

/// The bell's own strings; every one defaults to English.
#[derive(Clone, Default, PartialEq)]
pub struct NotificationBellLabels {
	/// The bell's name; "Notifications".
	pub trigger: Option<String>,
	/// Spoken after the name when something is unread; gets the count.
	/// "{n} unread". A callback, not a template, so a language can pick its
	/// plural form by the count.
	pub unread: Option<Callback<usize, String>>,
	/// Shown in place of an empty list; "No notifications".
	pub empty: Option<String>,
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
	let name = labels.trigger.unwrap_or_else(|| String::from("Notifications"));
	let spoken = match (unread, labels.unread) {
		(0, _) => name,
		(n, Some(say)) => format!("{name}, {}", say.call(n)),
		(n, None) => format!("{name}, {n} unread"),
	};
	// The trigger is the popover's own button wearing the secondary icon
	// button face: the kit has no `asChild` to hand the toggle to an `IconButton`.
	let trigger_cls = cn!(
		button_classes(&IconButtonVariant::Secondary.button_variant(), Size::Md, true, None, ""),
		ICON_BUTTON_BASE,
		NOTIFICATION_BELL_TRIGGER,
		class
	);
	let empty = labels.empty.unwrap_or_else(|| String::from("No notifications"));
	rsx! {
		Popover {
			PopoverTrigger { class: trigger_cls, aria_label: spoken,
				Glyph { class: "size-4",
					path { d: "M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9" }
					path { d: "M10.3 21a1.94 1.94 0 0 0 3.4 0" }
				}
				if unread > 0 {
					span { class: NOTIFICATION_DOT, "data-slot": "notification-dot", "aria-hidden": "true" }
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
				"data-slot": "notification",
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
	let placeholder = placeholder.unwrap_or_else(|| String::from("Search or jump to…"));
	let hint = shortcut.unwrap_or_else(|| String::from(platform()));
	rsx! {
		button {
			r#type: "button",
			class: cls,
			"data-slot": "header-search",
			onclick: move |e| {
				if let Some(h) = onclick {
					h.call(e);
				}
			},
			Glyph { class: "size-4",
				circle { cx: "11", cy: "11", r: "8" }
				path { d: "m21 21-4.3-4.3" }
			}
			span { class: HEADER_SEARCH_PLACEHOLDER, {placeholder} }
			Kbd { {hint} }
		}
	}
}

const CTRL_K: &str = "Ctrl K";
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
const COMMAND_K: &str = "⌘K";

/// The TS `APPLE` test: Mac, iPhone, iPad and iPod name their platform so.
/// Outside the effect so a server-rendered test can check it.
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
fn platform_shortcut(platform: &str) -> &'static str {
	if ["Mac", "iPhone", "iPad", "iPod"].iter().any(|p| platform.contains(p)) {
		COMMAND_K
	} else {
		CTRL_K
	}
}

/// Read after mount, so the server render and the first client render agree.
fn use_platform_shortcut() -> Signal<&'static str> {
	let shortcut = use_signal(|| CTRL_K);
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	{
		let mut shortcut = shortcut;
		use_effect(move || {
			// A browser that refuses to name its platform gets the Ctrl hint,
			// which is what the TS port shows for an empty platform too.
			let platform = web_sys::window().and_then(|w| w.navigator().platform().ok()).unwrap_or_default();
			let next = platform_shortcut(&platform);
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
	use std::sync::atomic::{AtomicUsize, Ordering};

	use super::*;
	use crate::uikit::test_util::{Harness, render, render_after_click};

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
	fn the_balance_colours_the_figure_for_the_warn_tone() {
		fn app() -> Element {
			rsx! { HeaderBalance { value: "$0.00", tone: HeaderBalanceTone::Warn } }
		}
		let html = render(app);
		let figure = html.split("data-slot=\"header-balance-value\"").next().and_then(|b| b.rsplit("<span").next()).expect("figure");
		assert!(figure.contains("text-accent-warn-ink") && !figure.contains("text-ink "), "{figure}");
	}

	#[test]
	fn the_balance_takes_a_translated_label_and_a_name() {
		fn app() -> Element {
			rsx! { HeaderBalance { label: rsx! { "Баланс:" }, value: "1 234,50 ₽", aria_label: "Баланс счёта" } }
		}
		let html = render(app);
		assert!(html.contains("Баланс:") && !html.contains("Balance:"), "{html}");
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
					labels: NotificationBellLabels { trigger: Some("Уведомления".into()), unread: Some(unread), empty: None },
				}
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Уведомления, 2 непрочитанных\""), "{html}");
	}

	#[test]
	fn the_bell_lists_notifications_in_a_popover_and_reports_the_pick() {
		static PICKED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
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
		assert!(html.contains("Заявка исполнена") && html.contains("За октябрь") && html.contains("2 мин назад"), "{html}");
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
					labels: NotificationBellLabels { empty: Some("Ничего нового".into()), ..Default::default() },
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
		assert!(
			trigger.contains("bg-secondary") && trigger.contains("aspect-square") && trigger.contains("relative") && trigger.contains("ml-1"),
			"{trigger}"
		);
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
		assert!(html.contains("data-slot=\"kbd\"") && html.contains(">Ctrl K</kbd>"), "{html}");
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
		assert!(text.contains("truncate") && text.contains("min-w-0"), "{text}");
		assert!(html.contains(">/</kbd>") && !html.contains("Ctrl K"), "{html}");
	}

	#[test]
	fn apple_platforms_get_the_command_key() {
		assert_eq!(platform_shortcut("MacIntel"), "⌘K");
		assert_eq!(platform_shortcut("iPhone"), "⌘K");
		assert_eq!(platform_shortcut("Win32"), "Ctrl K");
		assert_eq!(platform_shortcut("Linux x86_64"), "Ctrl K");
		assert_eq!(platform_shortcut(""), "Ctrl K");
	}
}
