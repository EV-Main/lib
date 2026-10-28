use std::{
	collections::BTreeMap,
	rc::Rc,
	sync::atomic::{AtomicUsize, Ordering},
};

use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{self, SelectKey, SelectOption},
	uikit_dioxus::{
		SELECT_CONTENT_BOUNDS, SELECT_ITEM, Size,
		primitives::{Controllable, DismissReason, use_controllable, use_dismissable_layer, use_stable_id},
		select_trigger_size_class,
	},
};

// dep-light: inline positioning + backdrop; no portal/floating/drag — see README Limitations
#[component]
pub fn Select(
	value: Option<String>,
	#[props(default)] default_value: String,
	/// What a reset would put back: its option is marked, beneath the chosen one's mark.
	reset_value: Option<String>,
	on_value_change: Option<EventHandler<String>>,
	open: Option<bool>,
	#[props(default)] default_open: bool,
	on_open_change: Option<EventHandler<bool>>,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let value = use_controllable(value, default_value, on_value_change);
	let open = use_controllable(open, default_open, on_open_change);
	let mut reset = use_signal(|| reset_value.clone());
	if *reset.peek() != reset_value {
		reset.set(reset_value);
	}
	let id = use_stable_id("select-content");
	let content_id = use_signal(move || id);
	let trigger = use_signal(|| None);
	let state = use_signal(uikit::Select::default);
	let els = use_signal(BTreeMap::new);
	use_context_provider(|| SelectCtx {
		value,
		reset,
		open,
		content_id,
		trigger,
		state,
		els,
	});
	let cls = cn!("relative", class);
	rsx! {
		div { class: cls, "data-slot": "select", {children} }
	}
}
#[component]
pub fn SelectTrigger(#[props(default)] size: Size, #[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<SelectCtx>();
	let open = ctx.open.get();
	let data_state = if open { "open" } else { "closed" };
	// The placeholder mark is on `SelectValue`'s span, not on this button.
	let cls = cn!(
		"border-input [&_[data-placeholder]]:text-ink-soft [&_svg:not([class*='text-'])]:text-ink-soft \
		 focus-visible:border-ring focus-visible:ring-ring/50 aria-invalid:ring-accent-error/20 aria-invalid:border-accent-error \
		 flex w-fit items-center justify-between gap-2 rounded-[var(--control-radius)] border bg-transparent px-3 py-2 text-sm whitespace-nowrap shadow-xs \
		 transition-[color,box-shadow] outline-none focus-visible:ring-[3px] disabled:cursor-not-allowed disabled:opacity-50 \
		 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex \
		 *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 [&_svg]:pointer-events-none [&_svg]:shrink-0 \
		 [&_svg:not([class*='size-'])]:size-4",
		select_trigger_size_class(size),
		class
	);
	rsx! {
		button {
			r#type: "button",
			role: "combobox",
			class: cls,
			"data-slot": "select-trigger",
			"data-size": size.as_ref(),
			"data-state": data_state,
			"aria-haspopup": "listbox",
			"aria-expanded": if open { "true" } else { "false" },
			"aria-controls": open.then(|| ctx.content_id.read().clone()),
			onmounted: move |e| {
				let mut trigger = ctx.trigger;
				trigger.set(Some(e.data()));
			},
			onclick: move |_| if ctx.open.get() { ctx.close(true) } else { ctx.open.set(true) },
			onkeydown: move |e| {
				// A native select opens on the arrows too; Enter and Space already click the button.
				if !ctx.open.get() && matches!(e.key(), Key::ArrowDown | Key::ArrowUp) {
					e.prevent_default();
					ctx.open.set(true);
				}
			},
			{children}
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
				class: "size-4 opacity-50",
				path { d: "m6 9 6 6 6-6" }
			}
		}
	}
}
#[component]
pub fn SelectValue(#[props(default)] placeholder: String, #[props(default)] class: String) -> Element {
	let ctx = use_context::<SelectCtx>();
	let value = ctx.value.get();
	let is_empty = value.is_empty();
	let label = if is_empty { placeholder } else { value };
	rsx! {
		span {
			class,
			"data-slot": "select-value",
			"data-placeholder": if is_empty { Some("true") } else { None },
			{label}
		}
	}
}
#[component]
pub fn SelectContent(#[props(default)] class: String, children: Element) -> Element {
	let ctx = use_context::<SelectCtx>();
	let open = ctx.open.get();
	// Focus goes back to the trigger after a key, never after a click elsewhere,
	// where it would land on the trigger for a moment and fire its blur on the way out.
	let layer = use_dismissable_layer(open, move |reason| ctx.close(reason == DismissReason::Escape));
	if !open {
		// A caller's controlled `open: false` closes the list without
		// `SelectCtx::close`; an option left active would stop the next open
		// from landing, and focus would stay on the trigger.
		let mut state = ctx.state;
		if state.peek().active().is_some() {
			state.write().close();
		}
		return rsx! {};
	}
	let cls = cn!(
		"bg-popover text-ink absolute top-full left-0 z-50 mt-1 overflow-x-hidden overflow-y-auto rounded-md border border-border shadow-md",
		SELECT_CONTENT_BOUNDS,
		class
	);
	let backdrop_layer = layer.clone();
	rsx! {
		div {
			class: "fixed inset-0 z-40",
			onclick: move |_| backdrop_layer.on_outside(),
		}
		div {
			role: "listbox",
			id: ctx.content_id.read().clone(),
			class: cls,
			"data-slot": "select-content",
			"data-state": "open",
			tabindex: "-1",
			onkeydown: move |e| {
				if !layer.on_keydown(&e) {
					ctx.on_list_key(&e);
				}
			},
			div { class: "p-1", {children} }
		}
	}
}
#[component]
pub fn SelectItem(
	value: String,
	/// What the type-ahead matches — the text the option reads as, which the
	/// kit cannot read off `children`.
	text_value: String,
	#[props(default)] disabled: bool,
	#[props(default)] class: String,
	children: Element,
) -> Element {
	let ctx = use_context::<SelectCtx>();
	let selected = ctx.value.get() == value;
	let reset = ctx.reset.read().as_deref() == Some(value.as_str());
	let id = use_select_option(
		ctx,
		SelectOption {
			value: value.clone(),
			label: text_value,
			disabled,
		},
	);
	let cls = cn!(SELECT_ITEM, class);
	let choose = {
		let value = value.clone();
		move |_| {
			if !disabled {
				ctx.choose(value.clone());
			}
		}
	};
	let on_focus = {
		let value = value.clone();
		move |_| {
			// A disabled row takes focus from a click too: it becomes active, so
			// the arrows go on from it, and Enter or Space on it does nothing.
			let mut state = ctx.state;
			if state.peek().active() != Some(value.as_str()) {
				state.write().focused(value.clone());
			}
		}
	};
	rsx! {
		div {
			role: "option",
			class: cls,
			"data-slot": "select-item",
			"aria-selected": if selected { "true" } else { "false" },
			"data-reset": reset.then_some(""),
			"aria-disabled": disabled.then_some("true"),
			"data-disabled": disabled.then_some(""),
			tabindex: "-1",
			onmounted: move |e| ctx.mounted(id, e.data()),
			onfocus: on_focus,
			onclick: choose,
			if selected {
				span { class: "absolute right-2 flex size-3.5 items-center justify-center",
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
						class: "size-4",
						"aria-hidden": "true",
						path { d: "M20 6 9 17l-5-5" }
					}
				}
			}
			span { {children} }
		}
	}
}
#[component]
pub fn SelectGroup(#[props(default)] class: String, children: Element) -> Element {
	rsx! {
		div { role: "group", class, "data-slot": "select-group", {children} }
	}
}
#[component]
pub fn SelectLabel(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!("text-ink-soft px-2 py-1.5 text-xs", class);
	rsx! {
		div { class: cls, "data-slot": "select-label", {children} }
	}
}
#[component]
pub fn SelectSeparator(#[props(default)] class: String) -> Element {
	let cls = cn!("bg-border pointer-events-none -mx-1 my-1 h-px", class);
	rsx! {
		div { class: cls, "data-slot": "select-separator" }
	}
}
#[derive(Clone, Copy)]
struct SelectCtx {
	value: Controllable<String>,
	reset: Signal<Option<String>>,
	open: Controllable<bool>,
	/// The listbox's id, for the trigger's `aria-controls`.
	content_id: Signal<String>,
	/// Where focus goes back to once the list closes on a key or a choice.
	trigger: Signal<Option<Rc<MountedData>>>,
	/// Read from handlers only, so rendering never subscribes to it.
	state: Signal<uikit::Select>,
	/// The options' mounted elements: their document order, and where focus lands.
	els: Signal<BTreeMap<usize, Rc<MountedData>>>,
}
impl SelectCtx {
	fn choose(&self, value: String) {
		self.value.set(value);
		self.close(true);
	}

	/// `restore_focus`: hand focus back to the trigger — after a key or a
	/// choice, never after a click elsewhere.
	fn close(&self, restore_focus: bool) {
		self.open.set(false);
		let mut state = self.state;
		state.write().close();
		if restore_focus && let Some(el) = self.trigger.peek().clone() {
			focus(el);
		}
	}

	fn mounted(&self, id: usize, el: Rc<MountedData>) {
		let mut els = self.els;
		els.write().insert(id, el);
		let mut state = self.state;
		let landed = state.write().mounted(id, &self.value.get(), &*self.els.peek());
		if let Some(id) = landed {
			self.focus_option(id);
		}
	}

	fn focus_option(&self, id: usize) {
		if let Some(el) = self.els.peek().get(&id).cloned() {
			focus(el);
		}
	}

	fn on_list_key(&self, e: &KeyboardEvent) {
		let mut state = self.state;
		let Some(effect) = state.write().key(&e.into(), &self.value.get(), now_ms(), &*self.els.peek()) else {
			return;
		};
		if effect == SelectKey::Leave {
			return self.close(false);
		}
		e.prevent_default();
		match effect {
			SelectKey::Moved(id) => self.focus_option(id),
			SelectKey::Choose(value) => self.choose(value),
			SelectKey::Held | SelectKey::Leave => {}
		}
	}
}

/// Registers the calling item with its list, in render order, returning the id
/// its `onmounted` hands back. Registration happens during render, so the keys
/// already know the order before anything has mounted.
fn use_select_option(ctx: SelectCtx, option: SelectOption) -> usize {
	let id = use_hook(|| NEXT_OPTION_ID.fetch_add(1, Ordering::Relaxed));
	let mut state = ctx.state;
	// A guarded write: a write on every render would dirty the signal for nothing.
	if state.peek().option(id) != Some(&option) {
		state.write().register(id, option);
	}
	use_drop(move || {
		state.write().unregister(id);
		let mut els = ctx.els;
		els.write().remove(&id);
	});
	id
}
static NEXT_OPTION_ID: AtomicUsize = AtomicUsize::new(0);

/// Best-effort focus: a detached element just leaves focus where it was, which
/// is why the result is dropped rather than surfaced.
fn focus(el: Rc<MountedData>) {
	spawn(async move {
		let _ = el.set_focus(true).await;
	});
}

/// Milliseconds on some monotonic-enough clock, for the type-ahead window;
/// `None` on bare `wasm32` without `wasm`, where `SystemTime::now` panics.
fn now_ms() -> Option<f64> {
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	{
		Some(web_sys::js_sys::Date::now())
	}
	#[cfg(not(target_arch = "wasm32"))]
	{
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).ok().map(|d| d.as_secs_f64() * 1000.0)
	}
	#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
	{
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::{
		Dialog, DialogContent,
		test_util::{Step, render, render_after_click, render_after_keydown, render_after_keys, render_after_steps},
	};

	#[test]
	fn closed_hides_content() {
		fn app() -> Element {
			rsx! {
				Select {
					SelectTrigger {
						SelectValue { placeholder: "Pick".to_string() }
					}
					SelectContent {
						SelectItem { value: "a", text_value: "Apple", "Apple" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("role=\"combobox\""), "{html}");
		assert!(html.contains("Pick"), "placeholder shown: {html}");
		assert!(!html.contains("Apple"), "options hidden while closed: {html}");
	}

	#[test]
	fn open_shows_listbox_and_options() {
		fn app() -> Element {
			rsx! {
				Select { default_open: true,
					SelectTrigger {
						SelectValue { placeholder: "Pick".to_string() }
					}
					SelectContent {
						SelectItem { value: "a", text_value: "Apple", "Apple" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("role=\"listbox\""), "{html}");
		assert!(html.contains("role=\"option\""), "{html}");
		assert!(html.contains("Apple"), "{html}");
	}

	#[test]
	fn open_content_overlays_and_caps_height() {
		fn app() -> Element {
			rsx! {
				Select { default_open: true,
					SelectTrigger {
						SelectValue { placeholder: "Pick".to_string() }
					}
					SelectContent {
						SelectItem { value: "a", text_value: "Apple", "Apple" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("absolute"), "content floats out of flow: {html}");
		assert!(html.contains("max-h-[min(24rem,calc(100dvh-2rem))]"), "height capped so overflow-y-auto engages: {html}");
		assert!(html.contains("max-w-[min(24rem,calc(100vw-2rem))]"), "width capped inside the viewport: {html}");
		assert!(html.contains("[overflow-wrap:anywhere]"), "a long option wraps: {html}");
		assert!(!html.contains("--radix-"), "no dead Radix vars: {html}");
	}

	#[test]
	fn reset_value_marks_its_option_whether_chosen_or_not() {
		fn app(chosen: &str) -> Element {
			rsx! {
				Select { default_value: chosen.to_string(), reset_value: "b".to_string(), default_open: true,
					SelectTrigger {
						SelectValue {}
					}
					SelectContent {
						SelectItem { value: "a", text_value: "Apple", "Apple" }
						SelectItem { value: "b", text_value: "Banana", "Banana" }
					}
				}
			}
		}
		fn chose_a() -> Element {
			app("a")
		}
		fn chose_b() -> Element {
			app("b")
		}
		let marked = |html: &str| html.matches("data-reset=").count();
		let html = render(chose_a);
		assert_eq!(marked(&html), 1, "{html}");
		assert!(html.contains("aria-selected:bg-accent-info/20"), "the chosen row is tinted: {html}");
		assert!(html.contains("data-[reset]:not-aria-selected:bg-accent-trace/20"), "the reset row, quieter: {html}");
		let html = render(chose_b);
		assert_eq!(marked(&html), 1, "chosen too; the tint is then the chosen one's: {html}");
	}

	#[test]
	fn selected_value_replaces_placeholder() {
		fn app() -> Element {
			rsx! {
				Select { default_value: "a".to_string(), default_open: true,
					SelectTrigger {
						SelectValue { placeholder: "Pick".to_string() }
					}
					SelectContent {
						SelectItem { value: "a", text_value: "Apple", "Apple" }
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("aria-selected=\"true\""), "{html}");
		// The attribute, not the substring: the trigger's class names `[data-placeholder]` too.
		assert!(!html.contains("data-placeholder="), "value replaces placeholder: {html}");
		assert!(!html.contains(">Pick<"), "value replaces placeholder: {html}");
	}

	#[test]
	fn trigger_size_sm() {
		fn app() -> Element {
			rsx! {
				Select {
					SelectTrigger { size: Size::Sm, "x" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("h-8"), "{html}");
	}

	#[test]
	fn trigger_size_lg_matches_the_large_input() {
		fn app() -> Element {
			rsx! {
				Select {
					SelectTrigger { size: Size::Lg, "x" }
				}
			}
		}
		let html = render(app);
		assert!(html.contains("h-12"), "{html}");
		assert!(html.contains("text-base"), "{html}");
		assert!(!html.contains("text-sm"), "lg replaces the md type: {html}");
		assert!(html.contains("rounded-[var(--control-radius)]"), "{html}");
	}

	/// The value the trigger shows, i.e. what the keys chose.
	fn shown(html: &str) -> &str {
		let from = html.find("data-slot=\"select-value\"").expect("a select value");
		let start = from + html[from..].find('>').expect("closed tag") + 1;
		let end = start + html[start..].find('<').expect("text end");
		&html[start..end]
	}

	fn fruit() -> Element {
		rsx! {
			Select {
				SelectTrigger {
					SelectValue { placeholder: "Pick".to_string() }
				}
				SelectContent {
					SelectItem { value: "apple", text_value: "Apple", "Apple" }
					SelectItem { value: "banana", text_value: "Banana", "Banana" }
					SelectItem { value: "blueberry", text_value: "Blueberry", "Blueberry" }
					SelectItem { value: "cherry", text_value: "Cherry", disabled: true, "Cherry" }
					SelectItem { value: "date", text_value: "Date", "Date" }
				}
			}
		}
	}

	fn fruit_open_on_banana() -> Element {
		rsx! {
			Select { default_value: "banana".to_string(), default_open: true,
				SelectTrigger {
					SelectValue { placeholder: "Pick".to_string() }
				}
				SelectContent {
					SelectItem { value: "apple", text_value: "Apple", "Apple" }
					SelectItem { value: "banana", text_value: "Banana", "Banana" }
					SelectItem { value: "blueberry", text_value: "Blueberry", "Blueberry" }
				}
			}
		}
	}

	#[test]
	fn trigger_announces_a_listbox_and_controls_it_while_open() {
		let closed = render(fruit);
		assert!(closed.contains("aria-haspopup=\"listbox\""), "{closed}");
		assert!(!closed.contains("aria-controls"), "nothing to control while closed: {closed}");
		let open = render(fruit_open_on_banana);
		let id_at = open.find("aria-controls=\"").expect("aria-controls while open") + "aria-controls=\"".len();
		let id = &open[id_at..id_at + open[id_at..].find('"').unwrap_or(0)];
		assert!(!id.is_empty(), "{open}");
		assert!(open.contains(&format!("role=\"listbox\" id=\"{id}\"")), "the listbox carries the controlled id: {open}");
	}

	#[test]
	fn arrows_on_the_trigger_open_the_list() {
		for key in [Key::ArrowDown, Key::ArrowUp] {
			let html = render_after_keydown(fruit, key);
			assert!(html.contains("role=\"listbox\""), "{html}");
			assert!(html.contains("aria-expanded=\"true\""), "{html}");
		}
		let html = render_after_keydown(fruit, Key::Escape);
		assert!(!html.contains("role=\"listbox\""), "only the arrows open it: {html}");
	}

	#[test]
	fn arrows_walk_the_options_and_enter_chooses() {
		// Opening lands on the first option; one more ArrowDown is the second.
		let html = render_after_keys(fruit, &[Key::ArrowDown, Key::ArrowDown, Key::Enter]);
		assert_eq!(shown(&html), "banana", "{html}");
		assert!(!html.contains("role=\"listbox\""), "a choice closes the list: {html}");
	}

	#[test]
	fn escape_and_tab_close_without_choosing() {
		for key in [Key::Escape, Key::Tab] {
			let html = render_after_keydown(fruit_open_on_banana, key);
			assert!(!html.contains("role=\"listbox\""), "{html}");
			assert_eq!(shown(&html), "banana", "{html}");
		}
	}

	#[test]
	fn disabled_option_is_marked_and_not_chosen_by_click() {
		fn app() -> Element {
			rsx! {
				Select { default_open: true,
					SelectTrigger {
						SelectValue { placeholder: "Pick".to_string() }
					}
					SelectContent {
						SelectItem { value: "cherry", text_value: "Cherry", disabled: true, "Cherry" }
					}
				}
			}
		}
		assert!(render(app).contains("aria-disabled=\"true\""));
		let html = render_after_click(app);
		assert_eq!(shown(&html), "Pick", "{html}");
	}

	fn select_in_dialog() -> Element {
		rsx! {
			Dialog { default_open: true,
				DialogContent { show_close_button: false,
					Select { default_open: true,
						SelectTrigger {
							SelectValue { placeholder: "Pick".to_string() }
						}
						SelectContent {
							SelectItem { value: "a", text_value: "A", "A" }
						}
					}
				}
			}
		}
	}

	#[test]
	fn escape_closes_only_the_top_layer() {
		let html = render_after_keydown(select_in_dialog, Key::Escape);
		assert!(!html.contains("role=\"listbox\""), "the list closes: {html}");
		assert!(html.contains("role=\"dialog\""), "the dialog below stays: {html}");
		let html = render_after_keys(select_in_dialog, &[Key::Escape, Key::Escape]);
		assert!(!html.contains("role=\"dialog\""), "the next Escape is the dialog's: {html}");
	}

	#[test]
	fn a_click_outside_closes_only_the_top_layer() {
		let html = render_after_click(select_in_dialog);
		assert!(!html.contains("role=\"listbox\""), "{html}");
		assert!(html.contains("role=\"dialog\""), "the dialog's scrim hands the click to the list: {html}");
	}

	#[test]
	fn closing_from_outside_lets_the_next_open_land_again() {
		fn app() -> Element {
			let mut open = use_signal(|| false);
			rsx! {
				// Stands in for the caller closing the list through its own state.
				div {
					onkeydown: move |e| {
						if e.key() == Key::F2 {
							open.set(false);
						}
					},
				}
				Select { open: open(), on_open_change: move |v| open.set(v),
					SelectTrigger {
						SelectValue { placeholder: "Pick".to_string() }
					}
					SelectContent {
						SelectItem { value: "apple", text_value: "Apple", "Apple" }
						SelectItem { value: "banana", text_value: "Banana", "Banana" }
					}
				}
			}
		}
		let steps = [Step::Key(Key::ArrowDown), Step::Focus, Step::Key(Key::F2), Step::Key(Key::ArrowDown), Step::Key(Key::Enter)];
		let html = render_after_steps(app, &steps);
		assert_eq!(shown(&html), "apple", "the reopened list lands on the first option, not the stale one: {html}");
	}
}
