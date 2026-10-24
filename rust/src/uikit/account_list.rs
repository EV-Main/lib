use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		ACCOUNT_LIST_BASE, ACCOUNT_LIST_ITEM_ACTIONS, ACCOUNT_LIST_ITEM_AVATAR, ACCOUNT_LIST_ITEM_AVATAR_FALLBACK, ACCOUNT_LIST_ITEM_BADGES, ACCOUNT_LIST_ITEM_BASE, ACCOUNT_LIST_ITEM_CHECK,
		ACCOUNT_LIST_ITEM_CHECK_HIDDEN, ACCOUNT_LIST_ITEM_CHECKED, ACCOUNT_LIST_ITEM_DISABLED, ACCOUNT_LIST_ITEM_FIGURE, ACCOUNT_LIST_ITEM_HANDLE, ACCOUNT_LIST_ITEM_NAME,
		ACCOUNT_LIST_ITEM_RADIO, ACCOUNT_LIST_ITEM_TEXT, ACCOUNT_LIST_ITEM_UNCHECKED, Avatar, AvatarFallback,
		primitives::{Controllable, RovingFocus, RovingOrientation, use_controllable, use_roving_focus, use_roving_item},
	},
};

/// Pick the account you act as. Mirrors the TS `AccountList`.
///
/// Radio-group semantics: one account is current, and the arrows move and
/// select in both axes, skipping disabled accounts and wrapping. The current
/// account is the only tab stop; while none is chosen (`""`), every account
/// is one, as in TS. Home and End do nothing, as in TS.
///
/// The TS `onKeyDown` and the rest of the `div` props are not ported: the kit
/// spreads no props.
#[component]
pub fn AccountList(
	/// The accessible name of the group, e.g. "Accounts".
	aria_label: String,
	value: Option<String>,
	#[props(default)] default_value: String,
	on_value_change: Option<EventHandler<String>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let value = use_controllable(value, default_value, on_value_change);
	let roving = use_roving_focus(RovingOrientation::Both);
	let disabled = use_hook(DisabledItems::default);
	use_context_provider(|| AccountListCtx { value, roving, disabled });
	let cls = cn!(ACCOUNT_LIST_BASE, class);
	rsx! {
		div { role: "radiogroup", "aria-label": aria_label, "data-slot": "account-list", class: cls, {children} }
	}
}

/// One account in an [`AccountList`]. Mirrors the TS `AccountListItem`.
///
/// `name` is text, not a node as in TS: the default avatar shows its first
/// letter, which a node cannot give. `avatar` replaces that default.
/// `actions` sit beside the radio, because a control cannot live inside one.
/// `class` lands on the outer row, as in TS.
#[component]
pub fn AccountListItem(
	value: String,
	name: String,
	/// A line under the name: a handle or an email.
	handle: Option<Element>,
	avatar: Option<Element>,
	/// Role badges after the name block.
	badges: Option<Element>,
	/// A figure at the end, set in the display face: a balance.
	figure: Option<Element>,
	/// Controls beside the row, such as a menu button.
	actions: Option<Element>,
	#[props(default)] disabled: bool,
	#[props(default)] class: String,
) -> Element {
	let ctx = use_context::<AccountListCtx>();
	let roving = ctx.roving;
	let id = use_roving_item(roving, value.clone());
	ctx.disabled.mark(id, &value, disabled);
	use_drop({
		let items = ctx.disabled.clone();
		move || items.forget(id)
	});
	let current = ctx.value.get();
	let checked = current == value;
	let tab_stop = checked || current.is_empty();
	let cls = cn!(
		ACCOUNT_LIST_ITEM_BASE,
		if checked { ACCOUNT_LIST_ITEM_CHECKED } else { ACCOUNT_LIST_ITEM_UNCHECKED },
		if disabled { ACCOUNT_LIST_ITEM_DISABLED } else { "" },
		class
	);
	let check_cls = cn!(ACCOUNT_LIST_ITEM_CHECK, if checked { "" } else { ACCOUNT_LIST_ITEM_CHECK_HIDDEN });
	let initial = name.trim().chars().next().map(|c| c.to_uppercase().collect::<String>()).unwrap_or_default();
	let avatar = avatar.unwrap_or_else(|| {
		rsx! {
			Avatar { class: ACCOUNT_LIST_ITEM_AVATAR,
				AvatarFallback { class: ACCOUNT_LIST_ITEM_AVATAR_FALLBACK, {initial} }
			}
		}
	});
	let click_value = value.clone();
	let skip = ctx.disabled.clone();
	rsx! {
		div { "data-slot": "account-list-item", "data-state": if checked { "checked" } else { "unchecked" }, class: cls,
			button {
				r#type: "button",
				role: "radio",
				"data-slot": "account-list-item-radio",
				"aria-checked": if checked { "true" } else { "false" },
				disabled,
				tabindex: if tab_stop { "0" } else { "-1" },
				class: ACCOUNT_LIST_ITEM_RADIO,
				onmounted: move |e: MountedEvent| roving.attach(id, e.data()),
				// The browser drops events on a disabled button, but a Dioxus
				// dispatch does not check `disabled`; the guards hold every
				// renderer to the browser's rule.
				onclick: move |_| {
					if !disabled {
						ctx.value.set(click_value.clone());
					}
				},
				// The key is handled on the radio, not on the group as in TS:
				// the radio that hears it is the focused one, which TS has to
				// look up through `document.activeElement`.
				onkeydown: move |e: KeyboardEvent| {
					if disabled || !matches!(e.key(), Key::ArrowDown | Key::ArrowRight | Key::ArrowUp | Key::ArrowLeft) {
						return;
					}
					if let Some(next) = skip.next_enabled(&roving, &e, &value) {
						e.prevent_default();
						roving.focus(&next);
						ctx.value.set(next);
					}
				},
				{avatar}
				span { "data-slot": "account-list-item-text", class: ACCOUNT_LIST_ITEM_TEXT,
					span { class: ACCOUNT_LIST_ITEM_NAME, {name} }
					if let Some(handle) = handle {
						span { class: ACCOUNT_LIST_ITEM_HANDLE, {handle} }
					}
				}
				if let Some(badges) = badges {
					span { "data-slot": "account-list-item-badges", class: ACCOUNT_LIST_ITEM_BADGES, {badges} }
				}
				if let Some(figure) = figure {
					span { "data-slot": "account-list-item-figure", class: ACCOUNT_LIST_ITEM_FIGURE, {figure} }
				}
				span { "data-slot": "account-list-item-check", class: check_cls, "aria-hidden": "true",
					svg {
						class: "size-4",
						view_box: "0 0 24 24",
						fill: "none",
						stroke: "currentColor",
						stroke_width: "1.75",
						stroke_linecap: "round",
						stroke_linejoin: "round",
						"aria-hidden": "true",
						path { d: "M20 6 9 17l-5-5" }
					}
				}
			}
			if let Some(actions) = actions {
				div { "data-slot": "account-list-item-actions", class: ACCOUNT_LIST_ITEM_ACTIONS, {actions} }
			}
		}
	}
}

#[derive(Clone)]
struct AccountListCtx {
	value: Controllable<String>,
	roving: RovingFocus,
	disabled: DisabledItems,
}

/// The disabled accounts, by roving item id, so the arrows can step over
/// them. Not a signal: it is read only from key handlers, never to render,
/// and each item writes its own entry while it renders.
#[derive(Clone, Default)]
struct DisabledItems(Rc<RefCell<BTreeMap<usize, String>>>);
impl DisabledItems {
	fn mark(&self, id: usize, key: &str, disabled: bool) {
		let mut items = self.0.borrow_mut();
		if disabled {
			items.insert(id, key.to_owned());
		} else {
			items.remove(&id);
		}
	}

	fn forget(&self, id: usize) {
		self.0.borrow_mut().remove(&id);
	}

	/// The first enabled key that `e` walks to from `from`. Bounded by the
	/// count of disabled items: past them the walk meets an enabled one, at
	/// worst `from` itself.
	fn next_enabled(&self, roving: &RovingFocus, e: &KeyboardEvent, from: &str) -> Option<String> {
		let items = self.0.borrow();
		let mut at = roving.next(e, from)?;
		for _ in 0..items.len() {
			if !items.values().any(|k| *k == at) {
				break;
			}
			at = roving.next(e, &at)?;
		}
		(!items.values().any(|k| *k == at)).then_some(at)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::{render, render_after_click, render_after_keydown};

	/// The opening tag of the radio whose name is `name`.
	fn radio<'a>(html: &'a str, name: &str) -> &'a str {
		let end = html.find(&format!(">{name}<")).unwrap_or_else(|| panic!("no account {name}: {html}"));
		let start = html[..end].rfind("<button").expect("radio button");
		let tag_end = start + html[start..].find('>').expect("tag end");
		&html[start..tag_end]
	}

	/// The TS `setup()`: three accounts, the last one disabled, and a log of
	/// every reported change rendered as text so a test can read it back.
	fn three_accounts(default_value: &'static str) -> Element {
		let mut log = use_signal(Vec::<String>::new);
		rsx! {
			AccountList { aria_label: "Счета", default_value, on_value_change: move |v| log.write().push(v),
				AccountListItem { value: "personal", name: "Яна Петрова", handle: rsx! { "yana.petrova" }, badges: rsx! { span { "Владелец" } } }
				AccountListItem { value: "team", name: "Акме", actions: rsx! { button { r#type: "button", "Ещё" } } }
				AccountListItem { value: "old", name: "Архив", disabled: true }
			}
			p { "log:{log.read().join(\",\")}" }
		}
	}

	fn chosen() -> Element {
		three_accounts("personal")
	}

	fn none_chosen() -> Element {
		three_accounts("")
	}

	#[test]
	fn is_a_named_radio_group_with_the_current_account_checked_and_tabbable() {
		let html = render(chosen);
		assert!(html.contains("role=\"radiogroup\""), "{html}");
		assert!(html.contains("aria-label=\"Счета\""), "{html}");
		assert_eq!(html.matches("role=\"radio\"").count(), 3, "{html}");
		assert!(radio(&html, "Яна Петрова").contains("aria-checked=\"true\""), "{html}");
		assert!(radio(&html, "Яна Петрова").contains("tabindex=\"0\""), "{html}");
		assert!(radio(&html, "Акме").contains("aria-checked=\"false\""), "{html}");
		assert!(radio(&html, "Акме").contains("tabindex=\"-1\""), "{html}");
		assert!(radio(&html, "Архив").contains("disabled"), "{html}");
		assert_eq!(html.matches("data-state=\"checked\"").count(), 1, "{html}");
	}

	#[test]
	fn makes_every_radio_tabbable_while_nothing_is_chosen() {
		let html = render(none_chosen);
		assert!(radio(&html, "Яна Петрова").contains("tabindex=\"0\""), "{html}");
		assert!(radio(&html, "Акме").contains("tabindex=\"0\""), "{html}");
		assert!(!html.contains("aria-checked=\"true\""), "{html}");
	}

	#[test]
	fn selects_on_click_and_ignores_a_disabled_account() {
		// Clicks reach every element in mount order: Акме is the last enabled
		// radio, and the disabled one after it must not take the selection.
		let html = render_after_click(chosen);
		assert!(html.contains("log:personal,team<"), "{html}");
		assert!(radio(&html, "Акме").contains("aria-checked=\"true\""), "{html}");
		assert!(radio(&html, "Яна Петрова").contains("aria-checked=\"false\""), "{html}");
		assert!(radio(&html, "Акме").contains("tabindex=\"0\""), "{html}");
	}

	#[test]
	fn reports_the_click_and_stays_put_when_controlled() {
		fn app() -> Element {
			let mut picked = use_signal(String::new);
			rsx! {
				AccountList { aria_label: "Счета", value: "personal", on_value_change: move |v| picked.set(v),
					AccountListItem { value: "personal", name: "Яна Петрова" }
					AccountListItem { value: "team", name: "Акме" }
				}
				p { "picked:{picked}" }
			}
		}
		let html = render_after_click(app);
		assert!(html.contains("picked:team"), "{html}");
		assert!(radio(&html, "Яна Петрова").contains("aria-checked=\"true\""), "the owner did not move the value: {html}");
	}

	/// Each enabled radio hears the key once, in mount order: Яна moves to
	/// Акме, and Акме steps over the disabled Архив and wraps back to Яна.
	#[test]
	fn arrows_forward_select_skip_disabled_and_wrap() {
		for key in [Key::ArrowDown, Key::ArrowRight] {
			let html = render_after_keydown(chosen, key.clone());
			assert!(html.contains("log:team,personal<"), "{key:?}: {html}");
		}
	}

	/// Backwards Яна wraps past the disabled Архив to Акме, and Акме moves
	/// back to Яна.
	#[test]
	fn arrows_back_select_skip_disabled_and_wrap() {
		for key in [Key::ArrowUp, Key::ArrowLeft] {
			let html = render_after_keydown(chosen, key.clone());
			assert!(html.contains("log:team,personal<"), "{key:?}: {html}");
		}
	}

	#[test]
	fn home_end_and_other_keys_select_nothing() {
		for key in [Key::Home, Key::End, Key::Enter, Key::Tab] {
			let html = render_after_keydown(chosen, key.clone());
			assert!(html.contains("log:<"), "{key:?}: {html}");
		}
	}

	#[test]
	fn keeps_row_actions_outside_the_radio() {
		let html = render(chosen);
		let start = html.find(radio(&html, "Акме")).expect("radio");
		let radio_body = &html[start..start + html[start..].find("</button>").expect("radio end")];
		assert!(!radio_body.contains("Ещё"), "{radio_body}");
		let actions = html.find("data-slot=\"account-list-item-actions\"").expect("actions");
		assert!(actions > start && html[actions..].contains(">Ещё</button>"), "{html}");
	}

	#[test]
	fn the_default_avatar_shows_the_upper_case_initial() {
		fn app() -> Element {
			rsx! {
				AccountList { aria_label: "Счета",
					AccountListItem { value: "a", name: "  яна" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"avatar-fallback\""), "{html}");
		assert!(html.contains(">Я</div>"), "{html}");
		// The square tile and the display face survive the fuse over the
		// avatar's own circle and sans initial.
		let tile = &html[..html.find("data-slot=\"avatar\"").expect("avatar")];
		let tile = &tile[tile.rfind("class=").expect("class")..];
		assert!(
			tile.contains("size-14") && tile.contains("rounded-(--control-radius)") && !tile.contains("rounded-full"),
			"{tile}"
		);
		assert!(html.contains("font-display text-h2 font-light text-ink\" data-slot=\"avatar-fallback\""), "{html}");
	}

	#[test]
	fn a_custom_avatar_replaces_the_initial() {
		fn app() -> Element {
			rsx! {
				AccountList { aria_label: "Счета",
					AccountListItem { value: "a", name: "Яна", avatar: rsx! { img { "data-testid": "own" } } }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-testid=\"own\""), "{html}");
		assert!(!html.contains("data-slot=\"avatar\""), "{html}");
	}

	#[test]
	fn hides_the_check_mark_of_an_unchecked_account_but_keeps_its_place() {
		let html = render(chosen);
		assert_eq!(html.matches("data-slot=\"account-list-item-check\"").count(), 3, "{html}");
		assert_eq!(html.matches("text-primary-ink invisible").count(), 2, "{html}");
		assert!(html.contains("M20 6 9 17l-5-5"), "{html}");
	}

	#[test]
	fn truncates_a_long_name_and_handle_instead_of_widening_the_row() {
		fn app() -> Element {
			rsx! {
				AccountList { aria_label: "Счета", default_value: "a",
					AccountListItem {
						value: "a",
						name: "Общество с ограниченной ответственностью «Северный ветер»",
						handle: rsx! { "бухгалтерия.северный.ветер@пример.рф" },
						figure: rsx! { "1 250 000 ₽" },
					}
				}
			}
		}
		let html = render(app);
		assert!(
			html.contains("<span class=\"truncate text-body-l text-ink\">Общество с ограниченной ответственностью «Северный ветер»</span>"),
			"{html}"
		);
		assert!(
			html.contains("<span class=\"truncate text-body text-ink-mid\">бухгалтерия.северный.ветер@пример.рф</span>"),
			"{html}"
		);
		assert!(html.contains("flex min-w-0 flex-1 items-center"), "{html}");
		assert!(html.contains("tabular-nums text-ink\">1 250 000 ₽</span>"), "{html}");
	}

	#[test]
	fn marks_the_disabled_row_and_lets_class_win_on_the_row() {
		fn app() -> Element {
			rsx! {
				AccountList { aria_label: "Счета", class: "gap-4",
					AccountListItem { value: "a", name: "Яна", disabled: true, class: "p-4" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("class=\"flex flex-col gap-4\""), "{html}");
		let row = &html[html.find("data-slot=\"account-list-item\"").expect("row")..];
		let row = &row[..row.find('>').expect("tag end")];
		assert!(row.contains("opacity-40") && row.contains("p-4") && !row.contains("p-2.5"), "{row}");
	}
}
