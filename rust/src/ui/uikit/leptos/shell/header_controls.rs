use ev_lib_classes::{Icon, copy};
use leptos::prelude::*;

pub use crate::ui::uikit::core::HeaderNotification;
use crate::{
	cn,
	ui::uikit::leptos::{
		HEADER_BALANCE_BASE, HEADER_BALANCE_CHEVRON, HEADER_BALANCE_FIGURE, HEADER_BALANCE_INTERACTIVE, HEADER_BALANCE_LABEL, HEADER_BALANCE_SKELETON, HEADER_SEARCH_BASE,
		HEADER_SEARCH_PLACEHOLDER, ICON_BUTTON_BASE, IconButtonVariant, Kbd, NOTIFICATION_BELL_CONTENT, NOTIFICATION_BELL_EMPTY, NOTIFICATION_BELL_FOOTER, NOTIFICATION_BELL_ITEM,
		NOTIFICATION_BELL_ITEM_BODY, NOTIFICATION_BELL_ITEM_DESCRIPTION, NOTIFICATION_BELL_ITEM_DOT, NOTIFICATION_BELL_ITEM_DOT_READ, NOTIFICATION_BELL_ITEM_DOT_UNREAD,
		NOTIFICATION_BELL_ITEM_TIME, NOTIFICATION_BELL_ITEM_TITLE, NOTIFICATION_BELL_ITEM_TITLE_UNREAD, NOTIFICATION_BELL_LIST, NOTIFICATION_BELL_TRIGGER, NOTIFICATION_DOT, Popover,
		PopoverContent, PopoverTrigger, Size, Skeleton, Slot, Tone, button_classes, icon_button::Glyph, icon_button_size_class, primitives::icon_shapes,
	},
};

/// "Balance: $0.00" in the header; a button only when given `on_click`.
/// Mirrors the TS `HeaderBalance`.
///
/// `value` is the figure already formatted by the caller. The TS `amount`,
/// `format` and `locales` roll the figure through `AnimatedNumber`, which the
/// Rust kit does not have, and the TS `Settled` reveal is gone with it: the
/// figure simply replaces the skeleton.
#[component]
pub fn HeaderBalance(
	/// Before the figure; "Balance:".
	#[prop(optional, into)]
	label: Option<ViewFn>,
	/// The figure, already formatted: "$0.00".
	#[prop(into)]
	value: Signal<String>,
	/// True until the balance is known; a skeleton stands in for the figure.
	#[prop(optional, into)]
	loading: Signal<bool>,
	/// A glyph before the label.
	#[prop(optional, into)]
	icon: Option<ViewFn>,
	#[prop(optional)] tone: Tone,
	/// Makes the balance a button, with a chevron after the figure.
	#[prop(optional)]
	on_click: Option<Callback<leptos::ev::MouseEvent>>,
	/// Only meaningful with `on_click`.
	#[prop(optional, into)]
	disabled: Signal<bool>,
	/// The TS port forwards any attribute; this one names the figure for a
	/// screen reader when the label alone is not enough.
	#[prop(optional, into)]
	aria_label: Option<String>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let cls = cn!(HEADER_BALANCE_BASE, tone.as_class(), if on_click.is_some() { HEADER_BALANCE_INTERACTIVE } else { "" }, class);
	let body = view! {
		{icon.map(|i| i.run())}
		<span class=HEADER_BALANCE_LABEL>
			{match label {
				Some(label) => label.run(),
				None => copy::HEADER_BALANCE.into_any(),
			}}
		</span>
		{move || {
			if loading.get() {
				view! { <Skeleton class=HEADER_BALANCE_SKELETON /> }.into_any()
			} else {
				view! {
					<span class=HEADER_BALANCE_FIGURE data-slot=Slot::HeaderBalanceValue.as_str()>
						{value.get()}
					</span>
				}
					.into_any()
			}
		}}
	};
	let Some(on_click) = on_click else {
		return view! {
			<div class=cls data-slot=Slot::HeaderBalance.as_str() aria-label=aria_label>
				{body}
			</div>
		}
		.into_any();
	};
	view! {
		<button
			type="button"
			class=cls
			data-slot=Slot::HeaderBalance.as_str()
			aria-label=aria_label
			disabled=disabled
			on:click=move |e| on_click.run(e)
		>
			{body}
			<Glyph class=HEADER_BALANCE_CHEVRON>{icon_shapes(Icon::ChevronDown)}</Glyph>
		</button>
	}
	.into_any()
}

/// The bell's own strings; every one defaults to English.
#[derive(Clone)]
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
	#[prop(into)] notifications: Signal<Vec<HeaderNotification>>,
	#[prop(optional)] on_select: Option<Callback<String>>,
	#[prop(optional)] labels: NotificationBellLabels,
	/// Under the list: "View all".
	#[prop(optional, into)]
	footer: Option<ViewFn>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let unread = Memo::new(move |_| notifications.with(|ns| ns.iter().filter(|n| n.unread).count()));
	let name = labels.trigger;
	let say = labels.unread;
	let spoken = move || match (unread.get(), say) {
		(0, _) => name.clone(),
		(n, Some(say)) => format!("{name}, {}", say.run(n)),
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
	view! {
		<Popover>
			<PopoverTrigger class=trigger_cls aria_label=Signal::derive(spoken)>
				<Glyph class="size-4">{icon_shapes(Icon::Bell)}</Glyph>
				{move || {
					(unread.get() > 0)
						.then(|| {
							view! {
								<span
									class=NOTIFICATION_DOT
									data-slot=Slot::NotificationDot.as_str()
									aria-hidden="true"
								></span>
							}
						})
				}}
			</PopoverTrigger>
			<PopoverContent align="end" class=NOTIFICATION_BELL_CONTENT>
				{
					let empty = empty.clone();
					move || {
						let ns = notifications.get();
						if ns.is_empty() {
							view! { <p class=NOTIFICATION_BELL_EMPTY>{empty.clone()}</p> }.into_any()
						} else {
							view! {
								<ul class=NOTIFICATION_BELL_LIST>
									{ns
										.into_iter()
										.map(|n| view! { <NotificationRow notification=n on_select /> })
										.collect_view()}
								</ul>
							}
								.into_any()
						}
					}
				}
				{footer.as_ref().map(|f| view! { <div class=NOTIFICATION_BELL_FOOTER>{f.run()}</div> })}
			</PopoverContent>
		</Popover>
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
	#[prop(optional, into)]
	placeholder: Option<String>,
	/// The shortcut hint at the end. Default: "⌘K" on Apple platforms, "Ctrl K"
	/// elsewhere.
	#[prop(optional, into)]
	shortcut: Option<String>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let platform = RwSignal::new(copy::CTRL_K);
	// Read after mount, so the server render and the first client render agree.
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	Effect::new(move |_| platform.set(crate::ui::uikit::host::platform_shortcut()));
	let cls = cn!(HEADER_SEARCH_BASE, class);
	let placeholder = placeholder.unwrap_or_else(|| copy::HEADER_SEARCH.to_owned());
	let hint = move || shortcut.clone().unwrap_or_else(|| String::from(platform.get()));
	view! {
		<button
			type="button"
			class=cls
			data-slot=Slot::HeaderSearch.as_str()
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			<Glyph class="size-4">{icon_shapes(Icon::Search)}</Glyph>
			<span class=HEADER_SEARCH_PLACEHOLDER>{placeholder}</span>
			<Kbd>{hint}</Kbd>
		</button>
	}
}

#[component]
fn NotificationRow(notification: HeaderNotification, on_select: Option<Callback<String>>) -> impl IntoView {
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
	view! {
		<li>
			<button
				type="button"
				class=NOTIFICATION_BELL_ITEM
				data-slot=Slot::Notification.as_str()
				on:click=move |_| {
					if let Some(h) = on_select {
						h.run(id.clone());
					}
				}
			>
				<span class=dot_cls aria-hidden="true"></span>
				<span class=NOTIFICATION_BELL_ITEM_BODY>
					<span class=title_cls>{title}</span>
					{description.map(|d| view! { <span class=NOTIFICATION_BELL_ITEM_DESCRIPTION>{d}</span> })}
				</span>
				{time.map(|t| view! { <span class=NOTIFICATION_BELL_ITEM_TIME>{t}</span> })}
			</button>
		</li>
	}
}
