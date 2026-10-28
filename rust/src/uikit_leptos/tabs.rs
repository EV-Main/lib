use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{
		TABS_CONTENT, TABS_LIST, TABS_ROOT, TABS_TRIGGER, TabsOrientation,
		primitives::{Controllable, RovingFocus, use_controllable, use_roving_focus, use_roving_item},
	},
};

#[component]
pub fn Tabs(
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: String,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	#[prop(optional)] orientation: TabsOrientation,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let value = use_controllable(value, default_value, on_value_change);
	provide_context(TabsCtx { value, orientation });
	let cls = cn!(TABS_ROOT, class);
	view! {
		<div class=cls data-slot="tabs" data-orientation=orientation.as_ref().to_owned()>
			{children()}
		</div>
	}
}

/// Owns the group's roving focus, so the `tabindex="-1"` its triggers carry is
/// paid for: the arrows reach every tab the tab order no longer stops at.
#[component]
pub fn TabsList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<TabsCtx>();
	let roving = use_roving_focus(ctx.orientation.into());
	provide_context(roving);
	let cls = cn!(TABS_LIST, class);
	view! {
		<div
			role="tablist"
			class=cls
			data-slot="tabs-list"
			aria-orientation=ctx.orientation.as_ref().to_owned()
			// Automatic activation, the ARIA pattern for panels that render
			// instantly: arrowing to a tab selects it as well as focusing it,
			// which is also what carries the group's lone tab stop along.
			on:keydown=move |e| {
				if let Some(next) = roving.next(&e, &ctx.value.get_untracked()) {
					e.prevent_default();
					ctx.value.set(next.clone());
					roving.focus(&next);
				}
			}
		>
			{children()}
		</div>
	}
}

#[component]
pub fn TabsTrigger(#[prop(into)] value: String, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<TabsCtx>();
	let roving = expect_context::<RovingFocus>();
	let node = NodeRef::<leptos::html::Button>::new();
	use_roving_item(roving, Signal::stored(value.clone()), node);
	let key = StoredValue::new(value);
	let selected = Memo::new(move |_| key.with_value(|k| ctx.value.get() == *k));
	let cls = cn!(TABS_TRIGGER, class);
	view! {
		<button
			type="button"
			role="tab"
			class=cls
			data-slot="tabs-trigger"
			data-state=move || if selected.get() { "active" } else { "inactive" }
			aria-selected=move || selected.get().to_string()
			tabindex=move || {
				if key.with_value(|k| roving.is_tab_stop(k, &ctx.value.get())) { "0" } else { "-1" }
			}
			node_ref=node
			on:click=move |_| ctx.value.set(key.get_value())
		>
			{children()}
		</button>
	}
}

#[component]
pub fn TabsContent(#[prop(into)] value: String, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<TabsCtx>();
	let cls = cn!(TABS_CONTENT, class);
	move || {
		(ctx.value.get() == value).then(|| {
			view! {
				<div role="tabpanel" class=cls.clone() data-slot="tabs-content" data-state="active">
					{children()}
				</div>
			}
		})
	}
}

#[derive(Clone, Copy)]
struct TabsCtx {
	value: Controllable<String>,
	orientation: TabsOrientation,
}
