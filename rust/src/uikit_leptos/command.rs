use std::{
	collections::BTreeMap,
	sync::atomic::{AtomicUsize, Ordering},
};

use leptos::prelude::*;

use crate::{
	cn,
	uikit::{self, CommandKey, Row},
	uikit_leptos::{
		COMMAND_DIALOG_COMMAND, COMMAND_DIALOG_CONTENT, COMMAND_DIALOG_OVERLAY, COMMAND_EMPTY, COMMAND_GROUP, COMMAND_INPUT, COMMAND_INPUT_WRAPPER, COMMAND_ITEM, COMMAND_LIST, COMMAND_ROOT,
		COMMAND_SEPARATOR, COMMAND_SHORTCUT,
		primitives::{Controllable, stable_id, use_controllable, use_stable_id},
	},
};

// dep-light: inline positioning + backdrop; no portal/floating/drag — see README Limitations

#[component]
pub fn Command(
	#[prop(optional, into)] search: MaybeProp<String>,
	#[prop(optional, into)] default_search: String,
	#[prop(optional)] on_search_change: Option<Callback<String>>,
	/// `false` hands filtering to the caller (e.g. rows that are a server
	/// response to the query): every mounted item renders, in the caller's
	/// order, and `CommandEmpty` counts mounted items.
	#[prop(default = true.into(), into)]
	should_filter: Signal<bool>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let search = use_controllable(search, default_search, on_search_change);
	let state = RwSignal::new(uikit::Command::new(should_filter.get_untracked()));
	Effect::new(move |_| {
		let should_filter = should_filter.get();
		if state.with_untracked(|s| s.should_filter != should_filter) {
			state.update(|s| s.should_filter = should_filter);
		}
	});
	let layout = Trigger::new();
	let highlighted = Memo::new(move |_| {
		// Notified once the rows' DOM has settled, so a keyed re-sort is seen in its new order.
		layout.track();
		let search = search.get();
		state.with(|s| s.highlighted(&search, &Rows))
	});
	provide_context(CommandCtx {
		search,
		state,
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		layout,
		handlers: StoredValue::new(BTreeMap::new()),
		highlighted,
		list_id: Signal::stored(use_stable_id("command-list")),
	});
	let cls = cn!(COMMAND_ROOT, class);
	view! {
		<div class=cls data-slot="command">
			{children()}
		</div>
	}
}

#[component]
pub fn CommandDialog(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	/// See [`Command`]'s `should_filter`.
	#[prop(default = true.into(), into)]
	should_filter: Signal<bool>,
	#[prop(optional, into)] class: String,
	children: ChildrenFn,
) -> impl IntoView {
	let open = use_controllable(open, default_open, on_open_change);
	let cls = cn!(COMMAND_DIALOG_CONTENT, class);
	move || {
		open.get().then(|| {
			let children = children.clone();
			view! {
				<div class=COMMAND_DIALOG_OVERLAY on:click=move |_| open.set(false)></div>
				<div
					role="dialog"
					aria-modal="true"
					class=cls.clone()
					data-slot="command-dialog"
					on:keydown=move |e| {
						if e.key() == "Escape" {
							open.set(false);
						}
					}
				>
					<Command class=COMMAND_DIALOG_COMMAND should_filter>
						{children()}
					</Command>
				</div>
			}
		})
	}
}

#[component]
pub fn CommandInput(
	#[prop(optional, into)] placeholder: String,
	/// Marks the field `aria-invalid`, for a caller whose pick is missing.
	#[prop(optional, into)]
	invalid: Signal<bool>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let ctx = expect_context::<CommandCtx>();
	let cls = cn!(COMMAND_INPUT, class);
	view! {
		<div class=COMMAND_INPUT_WRAPPER data-slot="command-input-wrapper">
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="24"
				height="24"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="size-4 shrink-0 opacity-50"
				aria-hidden="true"
			>
				<circle cx="11" cy="11" r="8"></circle>
				<path d="m21 21-4.3-4.3"></path>
			</svg>
			// Focus stays here and points at the highlighted row (the ARIA
			// combobox pattern), so the rows never need to be tabbable.
			<input
				type="text"
				role="combobox"
				class=cls
				data-slot="command-input"
				aria-autocomplete="list"
				aria-expanded="true"
				aria-controls=ctx.list_id
				aria-activedescendant=move || ctx.highlighted_dom_id()
				aria-invalid=move || invalid.get().then_some("true")
				autocomplete="off"
				placeholder=placeholder
				value=ctx.search.get_untracked()
				prop:value=move || ctx.search.get()
				on:input=move |e| ctx.search.set(event_target_value(&e))
				on:keydown=move |e| ctx.on_input_key(&e)
			/>
		</div>
	}
}

#[component]
pub fn CommandList(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<CommandCtx>();
	let cls = cn!(COMMAND_LIST, class);
	let list = NodeRef::<leptos::html::Div>::new();
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	list.on_load(move |el| relayout_on_mutation(&el, ctx.layout));
	view! {
		<div role="listbox" id=ctx.list_id class=cls data-slot="command-list" node_ref=list>
			{children()}
		</div>
	}
}

/// Renders only when a search is under way and no item matched it — never next
/// to results, and never before the user has typed.
#[component]
pub fn CommandEmpty(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<CommandCtx>();
	let cls = cn!(COMMAND_EMPTY, class);
	let shows = Memo::new(move |_| {
		let search = ctx.search.get();
		ctx.state.with(|s| s.shows_empty(&search))
	});
	move || {
		shows.get().then(|| {
			view! {
				<div class=cls.clone() data-slot="command-empty">
					{children()}
				</div>
			}
		})
	}
}

#[component]
pub fn CommandGroup(#[prop(optional, into)] heading: String, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	static NEXT: AtomicUsize = AtomicUsize::new(0);
	provide_context(CommandGroupId(NEXT.fetch_add(1, Ordering::Relaxed)));
	let cls = cn!(COMMAND_GROUP, class);
	view! {
		<div role="group" class=cls data-slot="command-group">
			{(!heading.is_empty())
				.then(|| view! { <div data-slot="command-group-heading">{heading}</div> })}
			{children()}
		</div>
	}
}

/// Fuzzy-matched (fzf v1) by `value` against the parent `Command` search text
/// and ranked through CSS `order` (unless the `Command` has `should_filter:
/// false`); non-matching items render nothing.
#[component]
pub fn CommandItem(
	#[prop(into)] value: Signal<String>,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional)] on_select: Option<Callback<String>>,
	#[prop(optional, into)] class: String,
	children: ChildrenFn,
) -> impl IntoView {
	let ctx = expect_context::<CommandCtx>();
	let id = stable_id();
	let dom_id = row_dom_id(id);
	let group = use_context::<CommandGroupId>().map(|g| g.0);
	// Registered as it renders, whether or not it survives the filter, so
	// `CommandEmpty` gates on the search, not on who happens to be mounted.
	let row = {
		let dom_id = dom_id.clone();
		move || Row {
			value: value.get(),
			disabled: disabled.get(),
			dom_id: dom_id.clone(),
			group,
		}
	};
	ctx.state.update(|s| s.register(id, untrack(&row)));
	Effect::new(move |_| {
		let row = row();
		if ctx.state.with_untracked(|s| s.row(id) != Some(&row)) {
			ctx.state.update(|s| s.register(id, row));
		}
	});
	if let Some(on_select) = on_select {
		ctx.handlers.update_value(|h| {
			h.insert(id, on_select);
		});
	}
	on_cleanup(move || {
		ctx.state.update(|s| s.unregister(id));
		ctx.handlers.update_value(|h| {
			h.remove(&id);
		});
	});
	let rank = Memo::new(move |_| {
		let search = ctx.search.get();
		let value = value.get();
		ctx.state.with(|s| s.order(&search, &value))
	});
	let selected = Memo::new(move |_| ctx.highlighted.get() == Some(id));
	let cls = cn!(COMMAND_ITEM, class);
	move || {
		rank.get().map(|rank| {
			let children = children.clone();
			view! {
				<div
					role="option"
					id=dom_id.clone()
					class=cls.clone()
					data-slot="command-item"
					style=format!("order: {rank}")
					data-disabled=move || if disabled.get() { "true" } else { "false" }
					data-selected=move || if selected.get() { "true" } else { "false" }
					aria-selected=move || if selected.get() { "true" } else { "false" }
					aria-disabled=move || disabled.get().then_some("true")
					tabindex="-1"
					on:pointermove=move |_| {
						if !disabled.get_untracked() && !selected.get_untracked() {
							ctx.hover(id);
						}
					}
					on:click=move |_| {
						if !disabled.get_untracked() && let Some(h) = on_select {
							h.run(value.get_untracked());
						}
					}
				>
					{children()}
				</div>
			}
		})
	}
}

#[component]
pub fn CommandSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(COMMAND_SEPARATOR, class);
	view! { <div class=cls data-slot="command-separator"></div> }
}

#[component]
pub fn CommandShortcut(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(COMMAND_SHORTCUT, class);
	view! {
		<span class=cls data-slot="command-shortcut">
			{children()}
		</span>
	}
}

#[derive(Clone, Copy)]
struct CommandCtx {
	search: Controllable<String>,
	state: RwSignal<uikit::Command>,
	/// Notified when the rows' DOM changed under the list.
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	layout: Trigger,
	/// Every row's `on_select`. Not reactive: only Enter reads it.
	handlers: StoredValue<BTreeMap<usize, Callback<String>>>,
	highlighted: Memo<Option<usize>>,
	/// The listbox's id, for the input's `aria-controls`.
	list_id: Signal<String>,
}

impl CommandCtx {
	fn highlighted_dom_id(&self) -> Option<String> {
		let id = self.highlighted.get()?;
		self.state.with(|s| s.row(id).map(|r| r.dom_id.clone()))
	}

	fn hover(&self, id: usize) {
		let search = self.search.get_untracked();
		self.state.update(|s| s.hover(id, &search));
	}

	/// The order is read from the DOM at the key, not from the memo: the memo
	/// may not have seen a re-sort that the screen already shows.
	fn on_input_key(&self, e: &leptos::ev::KeyboardEvent) {
		let search = self.search.get_untracked();
		let mut effect = None;
		self.state.update(|s| effect = s.key(&e.into(), &search, &Rows));
		let Some(effect) = effect else { return };
		e.prevent_default();
		match effect {
			CommandKey::Select(row, value) =>
				if let Some(handler) = self.handlers.with_value(|h| h.get(&row).copied()) {
					handler.run(value);
				},
			CommandKey::Moved(row) => {
				#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
				if let Some(el) = dom::row(row) {
					let opts = web_sys::ScrollIntoViewOptions::new();
					opts.set_behavior(web_sys::ScrollBehavior::Instant);
					opts.set_block(web_sys::ScrollLogicalPosition::Nearest);
					opts.set_inline(web_sys::ScrollLogicalPosition::Nearest);
					el.scroll_into_view_with_scroll_into_view_options(&opts);
				}
				#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
				let _ = row; // no DOM to scroll
			}
		}
	}
}

/// Provided by [`CommandGroup`]: `order` only reorders siblings, so keyboard order is per group.
#[derive(Clone, Copy)]
struct CommandGroupId(usize);

fn row_dom_id(id: usize) -> String {
	format!("command-item-{id}")
}

/// The rows' elements, found by the DOM ids their registration ids mint. Nothing
/// is placed off the web.
struct Rows;
impl uikit::Placement<usize> for Rows {
	fn placed(&self, id: usize) -> bool {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		return dom::row(id).is_some_and(|el| el.is_connected());
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		{
			let _ = id; // no DOM
			false
		}
	}

	fn precedes(&self, a: usize, b: usize) -> bool {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		return match (dom::row(a), dom::row(b)) {
			(Some(a), Some(b)) => a.compare_document_position(&b) & web_sys::Node::DOCUMENT_POSITION_FOLLOWING != 0,
			_ => false,
		};
		#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
		{
			let _ = (a, b); // no DOM
			false
		}
	}
}

/// A keyed re-sort moves rows without re-running any of them, so the list's own
/// mutations are what tell the highlight.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn relayout_on_mutation(list: &web_sys::HtmlDivElement, layout: Trigger) {
	use leptos::wasm_bindgen::{JsCast, closure::Closure};
	let notify = Closure::<dyn Fn()>::new(move || layout.notify());
	let observer = web_sys::MutationObserver::new(notify.as_ref().unchecked_ref()).expect("MutationObserver is in every supported browser");
	let init = web_sys::MutationObserverInit::new();
	init.set_child_list(true);
	init.set_subtree(true);
	observer.observe_with_options(list, &init).expect("the list is an element");
	let kept = StoredValue::new_local((observer, notify));
	on_cleanup(move || kept.with_value(|(observer, _)| observer.disconnect()));
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
mod dom {
	pub(super) fn row(id: usize) -> Option<web_sys::Element> {
		web_sys::window()?.document()?.get_element_by_id(&super::row_dom_id(id))
	}
}
