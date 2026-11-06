use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit_leptos::{
		WORKSPACE_TAB_BASE, WORKSPACE_TAB_CLOSE, WORKSPACE_TAB_LABEL, WORKSPACE_TAB_SELECTED, WORKSPACE_TAB_SPACER, WORKSPACE_TAB_TRIGGER, WORKSPACE_TABS_BASE,
		icon_button::Glyph,
		primitives::{Controllable, RovingFocus, RovingOrientation, use_controllable, use_roving_focus, use_roving_item},
	},
};

/// A row of open workspaces. Mirrors the TS `WorkspaceTabs`.
///
/// Arrow keys move focus (Home/End jump) and wrap; selecting is Enter, Space
/// or a click, so moving through the row never swaps the screen. That is
/// manual activation, unlike [`Tabs`](super::Tabs), which selects on arrow:
/// opening a workspace is too heavy to happen on every key press.
///
/// The TS `onKeyDown` and the rest of the `div` props are not ported: the kit
/// spreads no props.
#[component]
pub fn WorkspaceTabs(
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let value = use_controllable(value, default_value, on_value_change);
	let roving = use_roving_focus(RovingOrientation::Horizontal);
	let cls = cn!(WORKSPACE_TABS_BASE, class);
	view! {
		<div role="tablist" aria-orientation="horizontal" data-slot="workspace-tabs" class=cls>
			<Provider value=WorkspaceTabsCtx { value, roving }>{children()}</Provider>
		</div>
	}
}

/// One open workspace in a [`WorkspaceTabs`] row. Mirrors the TS `WorkspaceTab`.
///
/// `icon` replaces the default layers glyph. `on_close` adds a close button,
/// and Delete or Backspace on the focused tab closes it too. `class` lands on
/// the outer pill, as in TS. The TS `onClick`/`onKeyDown` and the rest of the
/// `button` props are not ported; `on_value_change` on the row reports clicks.
#[component]
pub fn WorkspaceTab(
	#[prop(into)] value: String,
	#[prop(optional, into)] icon: Option<ViewFn>,
	#[prop(optional)] on_close: Option<Callback<()>>,
	#[prop(default = String::from("Close tab"), into)] close_label: String,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let ctx = expect_context::<WorkspaceTabsCtx>();
	let roving = ctx.roving;
	let node = NodeRef::<leptos::html::Button>::new();
	use_roving_item(roving, Signal::stored(value.clone()), node);
	let key = StoredValue::new(value);
	let selected = Memo::new(move |_| key.with_value(|k| ctx.value.get() == *k));
	let cls = Signal::derive(move || cn!(WORKSPACE_TAB_BASE, if selected.get() { WORKSPACE_TAB_SELECTED } else { "" }, &class));
	let glyph = match icon {
		Some(icon) => icon.run(),
		None => view! {
			<Glyph class="size-4">
				<path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z" />
				<path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65M22 12.65l-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65" />
			</Glyph>
		}
		.into_any(),
	};
	let close = match on_close {
		Some(close) => view! {
			<button
				type="button"
				data-slot="workspace-tab-close"
				aria-label=close_label
				tabindex="-1"
				class=WORKSPACE_TAB_CLOSE
				on:click=move |_| close.run(())
			>
				<Glyph class="size-4">
					<path d="M18 6 6 18M6 6l12 12" />
				</Glyph>
			</button>
		}
		.into_any(),
		None => view! { <span class=WORKSPACE_TAB_SPACER aria-hidden="true"></span> }.into_any(),
	};
	view! {
		<div
			data-slot="workspace-tab"
			data-state=move || if selected.get() { "active" } else { "inactive" }
			class=cls
		>
			<button
				type="button"
				role="tab"
				data-slot="workspace-tab-trigger"
				aria-selected=move || selected.get().to_string()
				tabindex=move || {
					if key.with_value(|k| roving.is_tab_stop(k, &ctx.value.get())) { "0" } else { "-1" }
				}
				class=WORKSPACE_TAB_TRIGGER
				node_ref=node
				on:click=move |_| ctx.value.set(key.get_value())
				// The key is handled on the tab, not on the row as in TS: the
				// tab that hears it is the focused one, which TS has to look up
				// through `document.activeElement`.
				on:keydown=move |e| {
					let from = key.get_value();
					if let Some(close) = on_close
						.filter(|_| matches!(e.key().as_str(), "Delete" | "Backspace"))
					{
						e.prevent_default();
						if let Some(next) = roving.neighbour(&from) {
							roving.focus(&next);
						}
						close.run(());
					} else if let Some(next) = roving.next(&e, &from) {
						e.prevent_default();
						roving.focus(&next);
					}
				}
			>
				{glyph}
				<span class=WORKSPACE_TAB_LABEL>{children()}</span>
			</button>
			{close}
		</div>
	}
}

#[derive(Clone, Copy)]
struct WorkspaceTabsCtx {
	value: Controllable<String>,
	roving: RovingFocus,
}
