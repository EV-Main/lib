use std::collections::BTreeMap;

use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit_leptos::{
		ACCOUNT_LIST_BASE, ACCOUNT_LIST_ITEM_ACTIONS, ACCOUNT_LIST_ITEM_AVATAR, ACCOUNT_LIST_ITEM_AVATAR_FALLBACK, ACCOUNT_LIST_ITEM_BADGES, ACCOUNT_LIST_ITEM_BASE, ACCOUNT_LIST_ITEM_CHECK,
		ACCOUNT_LIST_ITEM_CHECK_HIDDEN, ACCOUNT_LIST_ITEM_CHECKED, ACCOUNT_LIST_ITEM_DISABLED, ACCOUNT_LIST_ITEM_FIGURE, ACCOUNT_LIST_ITEM_HANDLE, ACCOUNT_LIST_ITEM_NAME,
		ACCOUNT_LIST_ITEM_RADIO, ACCOUNT_LIST_ITEM_TEXT, ACCOUNT_LIST_ITEM_UNCHECKED, Avatar, AvatarFallback,
		icon_button::Glyph,
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
	#[prop(into)]
	aria_label: String,
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let value = use_controllable(value, default_value, on_value_change);
	let roving = use_roving_focus(RovingOrientation::Both);
	let disabled = StoredValue::new(BTreeMap::new());
	let cls = cn!(ACCOUNT_LIST_BASE, class);
	view! {
		<div role="radiogroup" aria-label=aria_label data-slot="account-list" class=cls>
			<Provider value=AccountListCtx {
				value,
				roving,
				disabled,
			}>{children()}</Provider>
		</div>
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
	#[prop(into)] value: String,
	#[prop(into)] name: String,
	/// A line under the name: a handle or an email.
	#[prop(optional, into)]
	handle: Option<ViewFn>,
	#[prop(optional, into)] avatar: Option<ViewFn>,
	/// Role badges after the name block.
	#[prop(optional, into)]
	badges: Option<ViewFn>,
	/// A figure at the end, set in the serif face: a balance.
	#[prop(optional, into)]
	figure: Option<ViewFn>,
	/// Controls beside the row, such as a menu button.
	#[prop(optional, into)]
	actions: Option<ViewFn>,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let ctx = expect_context::<AccountListCtx>();
	let roving = ctx.roving;
	let node = NodeRef::<leptos::html::Button>::new();
	use_roving_item(roving, Signal::stored(value.clone()), node);
	ctx.disabled.update_value(|items| {
		items.insert(value.clone(), disabled);
	});
	let gone = value.clone();
	// `try_`: when the whole list unmounts, its registry may go first.
	on_cleanup(move || {
		ctx.disabled.try_update_value(|items| {
			items.remove(&gone);
		});
	});
	let key = StoredValue::new(value);
	let checked = Memo::new(move |_| key.with_value(|k| ctx.value.get() == *k));
	let cls = Signal::derive(move || {
		cn!(
			ACCOUNT_LIST_ITEM_BASE,
			if checked.get() { ACCOUNT_LIST_ITEM_CHECKED } else { ACCOUNT_LIST_ITEM_UNCHECKED },
			if disabled.get() { ACCOUNT_LIST_ITEM_DISABLED } else { "" },
			&class
		)
	});
	let check_cls = move || cn!(ACCOUNT_LIST_ITEM_CHECK, if checked.get() { "" } else { ACCOUNT_LIST_ITEM_CHECK_HIDDEN });
	let initial: String = name.trim().chars().next().map(|c| c.to_uppercase().collect()).unwrap_or_default();
	let avatar = match avatar {
		Some(avatar) => avatar.run(),
		None => view! {
			<Avatar class=ACCOUNT_LIST_ITEM_AVATAR>
				<AvatarFallback class=ACCOUNT_LIST_ITEM_AVATAR_FALLBACK>{initial}</AvatarFallback>
			</Avatar>
		}
		.into_any(),
	};
	view! {
		<div
			data-slot="account-list-item"
			data-state=move || if checked.get() { "checked" } else { "unchecked" }
			class=cls
		>
			<button
				type="button"
				role="radio"
				data-slot="account-list-item-radio"
				aria-checked=move || checked.get().to_string()
				disabled=disabled
				tabindex=move || if checked.get() || ctx.value.get().is_empty() { "0" } else { "-1" }
				class=ACCOUNT_LIST_ITEM_RADIO
				node_ref=node
				on:click=move |_| {
					if !disabled.get_untracked() {
						ctx.value.set(key.get_value());
					}
				}
				// The key is handled on the radio, not on the group as in TS:
				// the radio that hears it is the focused one, which TS has to
				// look up through `document.activeElement`.
				on:keydown=move |e| {
					if disabled.get_untracked()
						|| !matches!(e.key().as_str(), "ArrowDown" | "ArrowRight" | "ArrowUp" | "ArrowLeft")
					{
						return;
					}
					let skip = |k: &str| {
						ctx.disabled.with_value(|items| items.get(k).is_some_and(|d| d.get_untracked()))
					};
					if let Some(next) = roving.next_enabled(&e, &key.get_value(), skip) {
						e.prevent_default();
						roving.focus(&next);
						ctx.value.set(next);
					}
				}
			>
				{avatar}
				<span data-slot="account-list-item-text" class=ACCOUNT_LIST_ITEM_TEXT>
					<span class=ACCOUNT_LIST_ITEM_NAME>{name}</span>
					{handle.map(|h| view! { <span class=ACCOUNT_LIST_ITEM_HANDLE>{h.run()}</span> })}
				</span>
				{badges
					.map(|b| {
						view! {
							<span data-slot="account-list-item-badges" class=ACCOUNT_LIST_ITEM_BADGES>
								{b.run()}
							</span>
						}
					})}
				{figure
					.map(|f| {
						view! {
							<span data-slot="account-list-item-figure" class=ACCOUNT_LIST_ITEM_FIGURE>
								{f.run()}
							</span>
						}
					})}
				<span data-slot="account-list-item-check" class=check_cls aria-hidden="true">
					<Glyph class="size-4">
						<path d="M20 6 9 17l-5-5" />
					</Glyph>
				</span>
			</button>
			{actions
				.map(|a| {
					view! {
						<div data-slot="account-list-item-actions" class=ACCOUNT_LIST_ITEM_ACTIONS>
							{a.run()}
						</div>
					}
				})}
		</div>
	}
}

#[derive(Clone, Copy)]
struct AccountListCtx {
	value: Controllable<String>,
	roving: RovingFocus,
	/// Each account's `disabled`, by value, so the arrows can step over the
	/// disabled ones. Read only from key handlers, never to render.
	disabled: StoredValue<BTreeMap<String, Signal<bool>>>,
}
