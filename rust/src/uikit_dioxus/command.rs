use std::{
	collections::BTreeMap,
	rc::Rc,
	sync::atomic::{AtomicUsize, Ordering},
};

use dioxus::{
	html::{ScrollBehavior, ScrollLogicalPosition, ScrollToOptions},
	prelude::*,
};

use crate::{
	cn,
	uikit::{self, CommandKey, Row},
	uikit_dioxus::{
		COMMAND_DIALOG_COMMAND, COMMAND_DIALOG_CONTENT, COMMAND_DIALOG_OVERLAY, COMMAND_EMPTY, COMMAND_GROUP, COMMAND_INPUT, COMMAND_INPUT_WRAPPER, COMMAND_ITEM, COMMAND_LIST, COMMAND_ROOT,
		COMMAND_SEPARATOR, COMMAND_SHORTCUT,
		primitives::{Controllable, use_controllable, use_stable_id},
	},
};

// dep-light: inline positioning + backdrop; no portal/floating/drag — see README Limitations

#[component]
pub fn Command(
	search: Option<String>,
	#[props(default)] default_search: String,
	on_search_change: Option<EventHandler<String>>,
	/// `false` hands filtering to the caller (e.g. rows that are a server
	/// response to the query): every mounted item renders, in the caller's
	/// order, and `CommandEmpty` counts mounted items.
	#[props(default = true)]
	should_filter: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let search = use_controllable(search, default_search, on_search_change);
	let mut state = use_signal(|| uikit::Command::new(should_filter));
	if state.peek().should_filter != should_filter {
		state.write().should_filter = should_filter;
	}
	let els = use_signal(BTreeMap::new);
	let layout = use_signal(|| 0_u64);
	let handlers = use_hook(|| CopyValue::new(BTreeMap::new()));
	let layout_pending = use_hook(|| CopyValue::new(None::<usize>));
	let highlighted = use_memo(move || {
		// Read for the dependency only: bumped once the rows' DOM has settled
		// after a render, so a keyed re-sort is seen in its new order.
		let _ = layout.read();
		state.read().highlighted(&search.get(), &*els.read())
	});
	let id = use_stable_id("command-list");
	let list_id = use_signal(move || id);
	use_context_provider(|| CommandCtx {
		search,
		state,
		els,
		layout,
		layout_pending,
		handlers,
		highlighted,
		list_id,
	});
	let cls = cn!(COMMAND_ROOT, class);
	rsx! {
		div { class: cls, "data-slot": "command", {children} }
	}
}
#[component]
pub fn CommandDialog(
	open: Option<bool>,
	#[props(default)] default_open: bool,
	on_open_change: Option<EventHandler<bool>>,
	/// See [`Command`]'s `should_filter`.
	#[props(default = true)]
	should_filter: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let open = use_controllable(open, default_open, on_open_change);
	if !open.get() {
		return rsx! {};
	}
	rsx! {
		div {
			class: COMMAND_DIALOG_OVERLAY,
			onclick: move |_| open.set(false),
		}
		div {
			role: "dialog",
			"aria-modal": "true",
			class: COMMAND_DIALOG_CONTENT,
			"data-slot": "command-dialog",
			onkeydown: move |e| {
				if e.key() == Key::Escape {
					open.set(false);
				}
			},
			Command { class: COMMAND_DIALOG_COMMAND, should_filter,
				{children}
			}
		}
	}
}
#[component]
pub fn CommandInput(
	#[props(default)] placeholder: String,
	/// Marks the field `aria-invalid`, for a caller whose pick is missing.
	#[props(default)]
	invalid: bool,
	#[props(default)] class: String,
) -> Element {
	let ctx = use_context::<CommandCtx>();
	let value = ctx.search.get();
	let cls = cn!(COMMAND_INPUT, class);
	rsx! {
		div {
			class: COMMAND_INPUT_WRAPPER,
			"data-slot": "command-input-wrapper",
			svg {
				xmlns: "http://www.w3.org/2000/svg",
				width: "24",
				height: "24",
				view_box: "0 0 24 24",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "2",
				stroke_linecap: "round",
				stroke_linejoin: "round",
				class: "size-4 shrink-0 opacity-50",
				"aria-hidden": "true",
				circle { cx: "11", cy: "11", r: "8" }
				path { d: "m21 21-4.3-4.3" }
			}
			input {
				r#type: "text",
				role: "combobox",
				class: cls,
				"data-slot": "command-input",
				// Focus stays here and points at the highlighted row (the ARIA
				// combobox pattern), so the rows never need to be tabbable.
				"aria-autocomplete": "list",
				"aria-expanded": "true",
				"aria-controls": ctx.list_id.read().clone(),
				"aria-activedescendant": ctx.highlighted_dom_id(),
				"aria-invalid": invalid.then_some("true"),
				autocomplete: "off",
				placeholder,
				value,
				oninput: move |e| ctx.search.set(e.value()),
				onkeydown: move |e| ctx.on_input_key(&e),
			}
		}
	}
}
#[component]
pub fn CommandList(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<CommandCtx>();
	let cls = cn!(COMMAND_LIST, class);
	rsx! {
		div {
			role: "listbox",
			id: ctx.list_id.read().clone(),
			class: cls,
			"data-slot": "command-list",
			{children}
		}
	}
}
/// Renders only when a search is under way and no item matched it — never next
/// to results, and never before the user has typed.
#[component]
pub fn CommandEmpty(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<CommandCtx>();
	if !ctx.state.read().shows_empty(&ctx.search.get()) {
		return rsx! {};
	}
	let cls = cn!(COMMAND_EMPTY, class);
	rsx! {
		div { class: cls, "data-slot": "command-empty", {children} }
	}
}
#[component]
pub fn CommandGroup(#[props(default)] heading: String, #[props(default)] class: String, children: Element) -> Element {
	use_context_provider(|| CommandGroupId(dioxus::dioxus_core::current_scope_id().0));
	let cls = cn!(COMMAND_GROUP, class);
	rsx! {
		div { role: "group", class: cls, "data-slot": "command-group",
			if !heading.is_empty() {
				div { "data-slot": "command-group-heading", {heading} }
			}
			{children}
		}
	}
}
/// Fuzzy-matched (fzf v1) by `value` against the parent `Command` search text
/// and ranked through CSS `order` (unless the `Command` has `should_filter:
/// false`); non-matching items render nothing.
#[component]
pub fn CommandItem(value: String, #[props(default)] disabled: bool, on_select: Option<EventHandler<String>>, #[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<CommandCtx>();
	let id = use_hook(|| NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed));
	let dom_id = use_stable_id("command-item");
	let group = try_use_context::<CommandGroupId>().map(|g| g.0);
	// Registered whether or not this item survives the filter below, so
	// `CommandEmpty` gates on the search, not on who happens to be mounted.
	// `use_reactive` re-runs it if the value or `disabled` prop changes.
	let registered_id = dom_id.clone();
	use_effect(use_reactive!(|value, disabled| {
		let mut state = ctx.state;
		state.write().register(
			id,
			Row {
				value,
				disabled,
				dom_id: registered_id.clone(),
				group,
			},
		);
	}));
	// Read at Enter, not frozen at registration: a handler the caller wires up
	// (or drops) after mount must be the one that fires.
	{
		let mut handlers = ctx.handlers;
		handlers.write().insert(id, on_select);
	}
	use_drop(move || {
		let mut state = ctx.state;
		state.write().unregister(id);
		let mut els = ctx.els;
		els.write().remove(&id);
		let mut handlers = ctx.handlers;
		handlers.write().remove(&id);
		ctx.row_dropped(id);
	});

	// A memo, so a row re-renders only when its own rank moves, not on every registration.
	let rank = use_memo(use_reactive!(|value| ctx.state.read().order(&ctx.search.get(), &value)));
	ctx.relayout_after_render(id);
	let Some(rank) = rank() else {
		// Unmounted by the filter: a stale element must not be placed in the document order.
		let mut els = ctx.els;
		if els.peek().contains_key(&id) {
			els.write().remove(&id);
		}
		return rsx! {};
	};
	let cls = cn!(COMMAND_ITEM, class);
	let select = {
		let value = value.clone();
		move |_| {
			if !disabled && let Some(h) = &on_select {
				h.call(value.clone());
			}
		}
	};
	let selected = ctx.highlighted.read().is_some_and(|h| h == id);
	rsx! {
		div {
			role: "option",
			id: dom_id,
			class: cls,
			"data-slot": "command-item",
			style: "order: {rank}",
			"data-disabled": if disabled { "true" } else { "false" },
			"data-selected": if selected { "true" } else { "false" },
			"aria-selected": if selected { "true" } else { "false" },
			"aria-disabled": disabled.then_some("true"),
			tabindex: "-1",
			onmounted: move |e| {
				let mut els = ctx.els;
				els.write().insert(id, e.data());
			},
			onpointermove: move |_| {
				if !disabled && !selected {
					ctx.hover(id);
				}
			},
			onclick: select,
			{children}
		}
	}
}
#[component]
pub fn CommandSeparator(#[props(default)] class: String) -> Element {
	let cls = cn!(COMMAND_SEPARATOR, class);
	rsx! {
		div { class: cls, "data-slot": "command-separator" }
	}
}
#[component]
pub fn CommandShortcut(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(COMMAND_SHORTCUT, class);
	rsx! {
		span { class: cls, "data-slot": "command-shortcut", {children} }
	}
}
#[derive(Clone, Copy)]
struct CommandCtx {
	search: Controllable<String>,
	/// Items register through an effect, so a row is in here one render after mount.
	state: Signal<uikit::Command>,
	/// The rendered rows' elements: their document order, and scrolling the highlight into view.
	els: Signal<BTreeMap<usize, Rc<MountedData>>>,
	/// Bumped after a render of the rows, once the DOM has their new order.
	layout: Signal<u64>,
	/// Every row's `on_select`, refreshed on each of its renders. Not a signal:
	/// only Enter reads it.
	handlers: CopyValue<BTreeMap<usize, Option<EventHandler<String>>>>,
	/// The row whose effect will bump `layout` after this render, if one is queued.
	/// An effect dies with its row, so a row unmounting with it queued clears this.
	layout_pending: CopyValue<Option<usize>>,
	highlighted: Memo<Option<usize>>,
	/// The listbox's id, for the input's `aria-controls`.
	list_id: Signal<String>,
}

impl CommandCtx {
	fn highlighted_dom_id(&self) -> Option<String> {
		(*self.highlighted.read()).and_then(|id| self.state.read().row(id).map(|r| r.dom_id.clone()))
	}

	/// Queues one re-read of the rows' order for after this render — a keyed
	/// re-sort moves rows without remounting them, so nothing else would tell
	/// the highlight. Effects run once the edits are in the DOM.
	///
	/// The effect is queued on the calling row, `row`; dioxus drops a scope's
	/// queued effects with the scope, so [`CommandCtx::row_dropped`] takes the
	/// job back when that row unmounts first.
	fn relayout_after_render(&self, row: usize) {
		let mut pending = self.layout_pending;
		if pending.peek().is_some() {
			return;
		}
		pending.set(Some(row));
		let layout = self.layout;
		dioxus::dioxus_core::queue_effect(move || {
			pending.set(None);
			bump(layout);
		});
	}

	/// A row unmounting: if the re-read it queued has not run, it never will,
	/// so re-read now — the rows that stay are already in the DOM — and free
	/// the next render to queue its own.
	fn row_dropped(&self, row: usize) {
		let mut pending = self.layout_pending;
		if *pending.peek() == Some(row) {
			pending.set(None);
			bump(self.layout);
		}
	}

	fn hover(&self, id: usize) {
		let mut state = self.state;
		state.write().hover(id, &self.search.get());
	}

	/// The order is read from the DOM at the key, not from the memo: the memo
	/// may not have seen a re-sort that the screen already shows.
	fn on_input_key(&self, e: &KeyboardEvent) {
		let mut state = self.state;
		let Some(effect) = state.write().key(&e.into(), &self.search.get(), &*self.els.peek()) else {
			return;
		};
		e.prevent_default();
		match effect {
			CommandKey::Select(row, value) => {
				let on_select = self.handlers.peek().get(&row).copied().flatten();
				if let Some(handler) = on_select {
					handler.call(value);
				}
			}
			CommandKey::Moved(row) => {
				if let Some(el) = self.els.peek().get(&row).cloned() {
					// Best-effort, like focus: a row that unmounted meanwhile just
					// doesn't scroll, which is why the result is dropped.
					spawn(async move {
						let _ = el
							.scroll_to_with_options(ScrollToOptions {
								behavior: ScrollBehavior::Instant,
								vertical: ScrollLogicalPosition::Nearest,
								horizontal: ScrollLogicalPosition::Nearest,
							})
							.await;
					});
				}
			}
		}
	}
}

/// Provided by [`CommandGroup`]: `order` only reorders siblings, so keyboard order is per group.
#[derive(Clone, Copy)]
struct CommandGroupId(usize);

fn bump(mut layout: Signal<u64>) {
	layout.with_mut(|n| *n = n.wrapping_add(1));
}

static NEXT_ITEM_ID: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::test_util::{render, render_after_keys_on_queued_effects, render_after_settled_keys, render_with_effects};

	#[test]
	fn renders_all_items_when_empty_search() {
		fn app() -> Element {
			rsx! {
				Command {
					CommandInput { placeholder: "Search".to_string() }
					CommandList {
						CommandItem { value: "Apple", "Apple" }
						CommandItem { value: "Banana", "Banana" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("role=\"combobox\""), "{html}");
		assert!(html.contains("Apple"), "{html}");
		assert!(html.contains("Banana"), "{html}");
	}

	#[test]
	fn a_match_inside_a_group_also_suppresses_the_empty_state() {
		// Items register through context, so nesting must not hide them from
		// `CommandEmpty` the way a sibling-only check would.
		fn app() -> Element {
			rsx! {
				Command { default_search: "set".to_string(),
					CommandList {
						CommandEmpty { "No results found." }
						CommandGroup { heading: "Pages",
							CommandItem { value: "settings", "Settings" }
						}
					}
				}
			}
		}
		let html = render_with_effects(app);
		assert!(html.contains("Settings"), "{html}");
		assert!(!html.contains("command-empty"), "a match nested in a group still counts: {html}");
	}

	#[test]
	fn group_renders_heading() {
		fn app() -> Element {
			rsx! {
				Command {
					CommandList {
						CommandGroup { heading: "Fruit".to_string(),
							CommandItem { value: "Apple", "Apple" }
						}
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"command-group-heading\""), "{html}");
		assert!(html.contains("Fruit"), "{html}");
	}

	#[test]
	fn dialog_hidden_until_open() {
		fn closed() -> Element {
			rsx! {
				CommandDialog {
					CommandInput { placeholder: "Search".to_string() }
				}
			}
		}
		assert!(!render(closed).contains("command-dialog"), "{}", render(closed));

		fn opened() -> Element {
			rsx! {
				CommandDialog { default_open: true,
					CommandInput { placeholder: "Search".to_string() }
				}
			}
		}
		let html = render(opened);
		assert!(html.contains("role=\"dialog\""), "{html}");
	}

	#[test]
	fn open_dialog_is_modal() {
		fn app() -> Element {
			rsx! {
				CommandDialog { default_open: true,
					CommandInput { placeholder: "Search".to_string() }
				}
			}
		}
		let html = render(app);
		let at = html.find("data-slot=\"command-dialog\"").expect("dialog rendered");
		let tag = &html[html[..at].rfind('<').expect("opening tag")..at];
		assert!(tag.contains("role=\"dialog\""), "{tag}");
		assert!(tag.contains("aria-modal=\"true\""), "{tag}");
	}

	/// The value of the row marked highlighted, read off its text.
	fn highlighted(html: &str) -> Vec<&str> {
		html.match_indices("data-selected=\"true\"")
			.map(|(at, _)| {
				let start = at + html[at..].find('>').expect("tag end") + 1;
				&html[start..start + html[start..].find('<').expect("text end")]
			})
			.collect()
	}

	fn picked(html: &str) -> &str {
		let start = html.find("picked:").expect("the picked marker") + "picked:".len();
		&html[start..start + html[start..].find('<').expect("marker end")]
	}

	fn fruit() -> Element {
		let mut picked = use_signal(String::new);
		rsx! {
			span { "picked:{picked}" }
			Command {
				CommandInput { placeholder: "Search".to_string() }
				CommandList {
					CommandItem { value: "Apple", on_select: move |v| picked.set(v), "Apple" }
					CommandItem { value: "Banana", disabled: true, on_select: move |v| picked.set(v), "Banana" }
					CommandItem { value: "Cherry", on_select: move |v| picked.set(v), "Cherry" }
					CommandItem { value: "Date", on_select: move |v| picked.set(v), "Date" }
				}
			}
		}
	}

	#[test]
	fn input_is_a_combobox_that_controls_the_list() {
		let html = render_with_effects(fruit);
		let at = html.find("aria-controls=\"").expect("aria-controls on the input") + "aria-controls=\"".len();
		let id = &html[at..at + html[at..].find('"').unwrap_or(0)];
		assert!(!id.is_empty(), "{html}");
		assert!(html.contains(&format!("role=\"listbox\" id=\"{id}\"")), "{html}");
		assert!(html.contains("aria-expanded=\"true\""), "{html}");
		assert!(html.contains("aria-autocomplete=\"list\""), "{html}");
	}

	#[test]
	fn the_first_row_is_highlighted_and_named_by_the_input() {
		let html = render_with_effects(fruit);
		assert_eq!(highlighted(&html), ["Apple"], "{html}");
		let at = html.find("aria-activedescendant=\"").expect("an active descendant") + "aria-activedescendant=\"".len();
		let id = &html[at..at + html[at..].find('"').unwrap_or(0)];
		assert!(html.contains(&format!("role=\"option\" id=\"{id}\"")), "it names a row: {html}");
		assert!(html.contains("aria-disabled=\"true\""), "{html}");
	}

	#[test]
	fn enter_selects_the_highlighted_row() {
		let html = render_after_settled_keys(fruit, &[Key::Enter]);
		assert_eq!(picked(&html), "Apple", "{html}");
		let html = render_after_settled_keys(fruit, &[Key::ArrowDown, Key::Enter]);
		assert_eq!(picked(&html), "Cherry", "{html}");
	}

	#[test]
	fn should_filter_false_leaves_every_row_to_the_caller() {
		fn app() -> Element {
			rsx! {
				Command { default_search: "zzz".to_string(), should_filter: false,
					CommandInput {}
					CommandList {
						CommandEmpty { "No results found." }
						CommandItem { value: "Apple", "Apple" }
						CommandItem { value: "Banana", "Banana" }
					}
				}
			}
		}
		let html = render_with_effects(app);
		assert!(html.contains(">Apple<") && html.contains(">Banana<"), "{html}");
		assert!(!html.contains("command-empty"), "mounted rows are the results: {html}");
		assert_eq!(highlighted(&html), ["Apple"], "{html}");

		fn nothing_mounted() -> Element {
			rsx! {
				Command { default_search: "zzz".to_string(), should_filter: false,
					CommandList {
						CommandEmpty { "No results found." }
					}
				}
			}
		}
		assert!(render_with_effects(nothing_mounted).contains("No results found."));
	}

	#[test]
	fn an_on_select_given_after_mount_is_what_enter_fires() {
		fn app() -> Element {
			let mut armed = use_signal(|| false);
			let mut picked = use_signal(String::new);
			let on_select = armed().then(|| EventHandler::new(move |v: String| picked.set(v)));
			rsx! {
				span { "picked:{picked}" }
				// Stands in for the caller wiring the handler up later.
				div {
					onkeydown: move |e| {
						if e.key() == Key::F2 {
							armed.set(true);
						}
					},
				}
				Command {
					CommandInput {}
					CommandList {
						CommandItem { value: "Apple", on_select, "Apple" }
					}
				}
			}
		}
		let html = render_after_settled_keys(app, &[Key::F2, Key::Enter]);
		assert_eq!(picked(&html), "Apple", "{html}");
	}

	#[test]
	fn a_row_unmounted_before_its_relayout_ran_does_not_freeze_the_order() {
		/// Shows whether a re-read of the rows' order is still queued, re-rendering on each re-read.
		#[component]
		fn Probe() -> Element {
			let ctx = use_context::<CommandCtx>();
			let _ = ctx.layout.read();
			let pending = ctx.layout_pending.peek().is_some();
			rsx! { "pending:{pending}" }
		}
		fn app() -> Element {
			let mut rows = use_signal(|| vec!["Apple", "Banana"]);
			rsx! {
				div {
					onkeydown: move |e| {
						if e.key() == Key::F2 {
							rows.set(vec!["Banana"]);
						}
					},
				}
				Command {
					CommandInput {}
					CommandList {
						for row in rows() {
							CommandItem { key: "{row}", value: row, {row} }
						}
					}
					Probe {}
				}
			}
		}
		// Apple queued the first re-read; F2 unmounts it before that ran, and the
		// highlight moving to Banana then asks for another.
		let html = render_after_keys_on_queued_effects(app, &[Key::F2, Key::ArrowDown]);
		assert!(html.contains("pending:false"), "{html}");
	}
}
