use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{
		COMMAND_PALETTE_CONTENT, COMMAND_PALETTE_HINT, COMMAND_PALETTE_HINTS, COMMAND_PALETTE_LABEL, COMMAND_PALETTE_LIST, COMMAND_PALETTE_TITLE, CommandDialog, CommandEmpty, CommandGroup,
		CommandInput, CommandItem, CommandList, CommandSeparator, CommandShortcut, DialogTitle, Kbd,
		primitives::{use_controllable, use_stable_id},
	},
};

/// One action of a [`CommandPalette`]. Mirrors the TS `CommandPaletteItem`.
#[derive(Clone, PartialEq)]
pub struct CommandPaletteItem {
	pub id: String,
	pub label: String,
	pub icon: Option<Element>,
	/// Shown at the row end, e.g. "⌘N".
	pub shortcut: Option<String>,
	/// Extra words the search matches besides the label.
	pub keywords: Vec<String>,
	pub disabled: bool,
	pub on_select: EventHandler<()>,
}

/// A run of [`CommandPaletteItem`]s under one heading. Mirrors the TS
/// `CommandPaletteGroup`.
#[derive(Clone, PartialEq)]
pub struct CommandPaletteGroup {
	pub id: String,
	pub heading: Option<String>,
	pub items: Vec<CommandPaletteItem>,
}

/// The palette's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CommandPaletteLabels {
	/// Names the dialog for assistive tech; "Command palette".
	pub title: Option<String>,
	/// The search field; "Type a command or search…".
	pub placeholder: Option<String>,
	/// Shown when the search matches nothing; "No results.".
	pub empty: Option<String>,
	/// The ↑↓ hint; "navigate".
	pub navigate: Option<String>,
	/// The ↵ hint; "select".
	pub select: Option<String>,
	/// The esc hint; "close".
	pub close: Option<String>,
}

/// Search-and-run over a list of actions. Mirrors the TS `CommandPalette`.
/// Choosing a row closes the palette, then runs the row's `on_select`. Pair it
/// with [`use_command_palette_shortcut`] for ⌘K / Ctrl+K.
///
/// Built on [`CommandDialog`], so it keeps that dialog's limits: inline
/// `position: fixed`, no portal, no focus trap, and no place on the layer
/// stack. Escape and a click on the scrim close it.
#[component]
pub fn CommandPalette(
	open: Option<bool>,
	#[props(default)] default_open: bool,
	on_open_change: Option<EventHandler<bool>>,
	groups: Vec<CommandPaletteGroup>,
	#[props(default)] labels: CommandPaletteLabels,
	#[props(default)] class: String,
) -> Element {
	let state = use_controllable(open, default_open, on_open_change);
	let title_id = use_stable_id("command-palette-title");
	let cls = cn!(COMMAND_PALETTE_CONTENT, class);
	let title = labels.title.unwrap_or_else(|| String::from("Command palette"));
	let placeholder = labels.placeholder.unwrap_or_else(|| String::from("Type a command or search…"));
	let empty = labels.empty.unwrap_or_else(|| String::from("No results."));
	let navigate = labels.navigate.unwrap_or_else(|| String::from("navigate"));
	let select = labels.select.unwrap_or_else(|| String::from("select"));
	let close = labels.close.unwrap_or_else(|| String::from("close"));
	rsx! {
		CommandDialog {
			open: state.get(),
			on_open_change: move |next| state.set(next),
			aria_labelledby: title_id.clone(),
			class: cls,
			DialogTitle { id: title_id, class: COMMAND_PALETTE_TITLE, {title} }
			CommandInput { placeholder }
			CommandList { class: COMMAND_PALETTE_LIST,
				CommandEmpty { {empty} }
				for (i, group) in groups.into_iter().enumerate() {
					Fragment { key: "{group.id}",
						if i > 0 {
							CommandSeparator {}
						}
						CommandGroup { heading: group.heading.unwrap_or_default(),
							for item in group.items {
								CommandItem {
									key: "{item.id}",
									value: std::iter::once(item.label.as_str()).chain(item.keywords.iter().map(String::as_str)).collect::<Vec<_>>().join(" "),
									disabled: item.disabled,
									on_select: move |_| {
										state.set(false);
										item.on_select.call(());
									},
									{item.icon}
									span { class: COMMAND_PALETTE_LABEL, {item.label} }
									if let Some(shortcut) = item.shortcut {
										CommandShortcut { {shortcut} }
									}
								}
							}
						}
					}
				}
			}
			div { "data-slot": "command-palette-hints", class: COMMAND_PALETTE_HINTS,
				span { class: COMMAND_PALETTE_HINT,
					Kbd { "↑↓" }
					{navigate}
				}
				span { class: COMMAND_PALETTE_HINT,
					Kbd { "↵" }
					{select}
				}
				span { class: COMMAND_PALETTE_HINT,
					Kbd { "esc" }
					{close}
				}
			}
		}
	}
}

/// Calls `on_trigger` on ⌘K (Mac) or Ctrl+K, and keeps the browser from taking
/// the key (Chrome focuses the address bar on Ctrl+K in some layouts). Mirrors
/// the TS `useCommandPaletteShortcut`.
///
/// It matches the physical key (`code`), so it also works on a Russian or other
/// non-Latin layout. A held key does not fire again, or it would toggle the
/// palette open and shut. The listener sits on `document` while `enabled` is
/// true and goes with the calling component.
///
/// Only the `wasm` feature on `wasm32` has a `document` to listen on; any other
/// build does nothing here, server rendering included.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub fn use_command_palette_shortcut(on_trigger: impl FnMut() + 'static, enabled: bool) {
	use std::{cell::RefCell, rc::Rc};

	type Trigger = Rc<RefCell<Option<Box<dyn FnMut()>>>>;
	// Refreshed on every render, so the listener always calls the latest
	// callback without being bound again.
	let latest: Trigger = use_hook(|| Rc::new(RefCell::new(None)));
	*latest.borrow_mut() = Some(Box::new(on_trigger));
	let listener = use_hook(|| Rc::new(RefCell::new(None::<listener::KeydownListener>)));
	{
		let listener = listener.clone();
		use_effect(use_reactive!(|enabled| {
			let mut slot = listener.borrow_mut();
			if !enabled {
				*slot = None;
				return;
			}
			if slot.is_none() {
				let latest = latest.clone();
				*slot = listener::KeydownListener::add(move || {
					// Taken out for the call: a trigger that re-renders the caller
					// would otherwise meet its own slot still borrowed.
					let taken = latest.borrow_mut().take();
					if let Some(mut f) = taken {
						f();
						let mut slot = latest.borrow_mut();
						if slot.is_none() {
							*slot = Some(f);
						}
					}
				});
			}
		}));
	}
	use_drop(move || {
		listener.borrow_mut().take();
	});
}

/// See the `wasm` build of this hook: without a `document` there is nothing to
/// listen on.
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
pub fn use_command_palette_shortcut(on_trigger: impl FnMut() + 'static, enabled: bool) {
	let _ = (on_trigger, enabled);
}

#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
mod listener {
	use web_sys::{
		KeyboardEvent,
		wasm_bindgen::{JsCast, closure::Closure},
	};

	use super::{ShortcutHit, ShortcutKey, shortcut_hit};

	/// A `keydown` listener on `document`, removed when dropped.
	pub(super) struct KeydownListener {
		document: web_sys::Document,
		callback: Closure<dyn FnMut(KeyboardEvent)>,
	}
	impl KeydownListener {
		/// `None` when there is no `document` or it refuses the listener; the
		/// shortcut then just does not exist, which is all a caller could do.
		pub(super) fn add(mut on_fire: impl FnMut() + 'static) -> Option<Self> {
			let document = web_sys::window()?.document()?;
			let callback = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
				let key = ShortcutKey {
					code: &e.code(),
					ctrl: e.ctrl_key(),
					meta: e.meta_key(),
					alt: e.alt_key(),
					shift: e.shift_key(),
					repeat: e.repeat(),
				};
				match shortcut_hit(&key) {
					ShortcutHit::Miss => {}
					ShortcutHit::Held => e.prevent_default(),
					ShortcutHit::Fire => {
						e.prevent_default();
						on_fire();
					}
				}
			});
			document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref()).ok()?;
			Some(Self { document, callback })
		}
	}
	impl Drop for KeydownListener {
		fn drop(&mut self) {
			// A listener the document no longer holds is already gone, so a
			// failed removal leaves nothing to clean up.
			let _ = self.document.remove_event_listener_with_callback("keydown", self.callback.as_ref().unchecked_ref());
		}
	}
}

/// What one `keydown` means to the shortcut.
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ShortcutHit {
	/// Not ⌘K / Ctrl+K: the key goes on as usual.
	Miss,
	/// ⌘K / Ctrl+K held down: the browser must still not take it, but the
	/// palette must not toggle again.
	Held,
	/// ⌘K / Ctrl+K pressed: take it from the browser and fire.
	Fire,
}

/// The parts of a `keydown` the shortcut reads. The layout's `key` is not one
/// of them: on a Russian layout the K key reports `key == "л"` and the same
/// `code`.
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
struct ShortcutKey<'a> {
	code: &'a str,
	ctrl: bool,
	meta: bool,
	alt: bool,
	shift: bool,
	repeat: bool,
}

/// Pulled out of the listener so a server-rendered test, which runs no
/// effects and has no `document`, can still check the decision.
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
fn shortcut_hit(k: &ShortcutKey<'_>) -> ShortcutHit {
	if !(k.meta || k.ctrl) || k.alt || k.shift || k.code != "KeyK" {
		return ShortcutHit::Miss;
	}
	if k.repeat { ShortcutHit::Held } else { ShortcutHit::Fire }
}

#[cfg(test)]
mod tests {
	use std::sync::atomic::{AtomicUsize, Ordering};

	use super::*;
	use crate::uikit::test_util::{Harness, render, render_with_effects};

	fn groups(on_new: EventHandler<()>, on_find: EventHandler<()>) -> Vec<CommandPaletteGroup> {
		vec![
			CommandPaletteGroup {
				id: "s".into(),
				heading: Some("Предложения".into()),
				items: vec![CommandPaletteItem {
					id: "new".into(),
					label: "Новое пространство".into(),
					icon: None,
					shortcut: Some("⌘N".into()),
					keywords: Vec::new(),
					disabled: false,
					on_select: on_new,
				}],
			},
			CommandPaletteGroup {
				id: "t".into(),
				heading: Some("Настройки".into()),
				items: vec![CommandPaletteItem {
					id: "find".into(),
					label: "Поиск символа".into(),
					icon: None,
					shortcut: None,
					keywords: vec!["тикер".into()],
					disabled: false,
					on_select: on_find,
				}],
			},
		]
	}

	fn plain() -> Element {
		rsx! {
			CommandPalette { open: true, groups: groups(EventHandler::new(|_| {}), EventHandler::new(|_| {})) }
		}
	}

	/// The value of the `name` attribute on the element that carries `marker`.
	fn attr_on<'a>(html: &'a str, marker: &str, name: &str) -> &'a str {
		let at = html.find(marker).unwrap_or_else(|| panic!("no {marker}: {html}"));
		let tag = &html[html[..at].rfind('<').expect("tag start")..at + html[at..].find('>').expect("tag end")];
		let key = format!("{name}=\"");
		let start = tag.find(&key).unwrap_or_else(|| panic!("no {name} on {tag}")) + key.len();
		&tag[start..start + tag[start..].find('"').expect("attribute end")]
	}

	#[test]
	fn is_a_named_dialog_with_a_search_box_and_the_key_hints() {
		let html = render(plain);
		let named_by = attr_on(&html, "role=\"dialog\"", "aria-labelledby");
		let title_at = format!("id=\"{named_by}\"");
		assert!(attr_on(&html, &title_at, "class").contains("sr-only"), "{html}");
		let title = &html[html.find(&title_at).expect("found by attr_on")..];
		assert!(title[..title.find('<').expect("title end")].ends_with(">Command palette"), "{title}");
		assert_eq!(attr_on(&html, "role=\"combobox\"", "placeholder"), "Type a command or search…");
		assert!(html.contains("data-slot=\"command-palette-hints\""), "{html}");
		assert!(html.contains("navigate"), "{html}");
		assert!(html.contains(">esc<"), "{html}");
		assert!(html.contains("⌘N"), "{html}");
		assert!(html.contains("Предложения") && html.contains("data-slot=\"command-separator\""), "{html}");
	}

	#[test]
	fn the_labels_replace_every_default() {
		fn app() -> Element {
			rsx! {
				CommandPalette {
					open: true,
					groups: Vec::new(),
					labels: CommandPaletteLabels {
						title: Some("Палитра команд".into()),
						placeholder: Some("Команда или поиск…".into()),
						empty: Some("Ничего не найдено.".into()),
						navigate: Some("выбрать".into()),
						select: Some("запустить".into()),
						close: Some("закрыть".into()),
					},
				}
			}
		}
		let html = render(app);
		for text in ["Палитра команд", "Команда или поиск…", "выбрать", "запустить", "закрыть"] {
			assert!(html.contains(text), "{text}: {html}");
		}
		for text in ["Command palette", "Type a command", "navigate", ">select<", ">close<"] {
			assert!(!html.contains(text), "{text}: {html}");
		}
	}

	#[test]
	fn closed_renders_nothing() {
		fn app() -> Element {
			rsx! {
				CommandPalette { groups: groups(EventHandler::new(|_| {}), EventHandler::new(|_| {})) }
			}
		}
		assert!(!render(app).contains("role=\"dialog\""), "{}", render(app));
	}

	static NEW: AtomicUsize = AtomicUsize::new(0);
	static FIND: AtomicUsize = AtomicUsize::new(0);
	static CLOSED: AtomicUsize = AtomicUsize::new(0);

	/// Controlled from outside, as a caller holding the open state would.
	fn counted() -> Element {
		let mut open = use_signal(|| true);
		rsx! {
			CommandPalette {
				open: open(),
				on_open_change: move |next: bool| {
					if !next {
						CLOSED.fetch_add(1, Ordering::SeqCst);
					}
					open.set(next);
				},
				groups: groups(
					EventHandler::new(|_| {
						NEW.fetch_add(1, Ordering::SeqCst);
					}),
					EventHandler::new(|_| {
						FIND.fetch_add(1, Ordering::SeqCst);
					}),
				),
			}
		}
	}

	// One test, not two: both halves count on the same statics, and tests run in parallel.
	#[test]
	fn runs_the_chosen_action_and_closes_by_click_and_by_enter() {
		let reset = || [&NEW, &FIND, &CLOSED].iter().for_each(|c| c.store(0, Ordering::SeqCst));

		reset();
		let mut h = Harness::new(counted);
		// Rows are the only pointer-move listeners; the first is the first group's.
		let row = h.listening("pointermove")[0];
		h.click(row);
		assert_eq!(NEW.load(Ordering::SeqCst), 1);
		assert_eq!(FIND.load(Ordering::SeqCst), 0);
		assert_eq!(CLOSED.load(Ordering::SeqCst), 1);
		assert!(!h.html().contains("role=\"dialog\""), "the caller's state closed it: {}", h.html());

		reset();
		let mut h = Harness::new(counted);
		let input = h.listening("input")[0];
		h.key(input, Key::Enter);
		assert_eq!(NEW.load(Ordering::SeqCst), 1, "Enter runs the highlighted first row");
		assert_eq!(CLOSED.load(Ordering::SeqCst), 1);
	}

	#[test]
	fn filters_by_label_and_by_keywords_in_any_case_and_says_so_when_nothing_matches() {
		let mut h = Harness::new(plain);
		let input = h.listening("input")[0];
		h.input(input, "ТИКЕР");
		let html = h.html();
		assert!(html.contains("Поиск символа"), "the keyword matches: {html}");
		assert!(!html.contains("Новое пространство"), "{html}");
		assert!(!html.contains("No results."), "{html}");

		h.input(input, "НОВОЕ");
		let html = h.html();
		assert!(html.contains("Новое пространство") && !html.contains("Поиск символа"), "the label matches: {html}");

		h.input(input, "zzzz");
		let html = h.html();
		assert!(html.contains("No results."), "{html}");
		assert!(!html.contains("role=\"option\""), "{html}");
	}

	#[test]
	fn escape_asks_the_caller_to_close() {
		fn app() -> Element {
			let mut open = use_signal(|| true);
			rsx! {
				CommandPalette { open: open(), on_open_change: move |next| open.set(next), groups: Vec::new() }
			}
		}
		let mut h = Harness::new(app);
		let content = h.listening("keydown")[0];
		h.key(content, Key::Escape);
		assert!(!h.html().contains("role=\"dialog\""), "{}", h.html());
	}

	#[test]
	fn a_disabled_item_does_not_run() {
		static RAN: AtomicUsize = AtomicUsize::new(0);
		fn app() -> Element {
			let item = CommandPaletteItem {
				id: "off".into(),
				label: "Выключено".into(),
				icon: None,
				shortcut: None,
				keywords: Vec::new(),
				disabled: true,
				on_select: EventHandler::new(|_| {
					RAN.fetch_add(1, Ordering::SeqCst);
				}),
			};
			rsx! {
				CommandPalette {
					open: true,
					groups: vec![CommandPaletteGroup { id: "g".into(), heading: None, items: vec![item] }],
				}
			}
		}
		let mut h = Harness::new(app);
		let row = h.listening("pointermove")[0];
		h.click(row);
		assert_eq!(RAN.load(Ordering::SeqCst), 0);
		assert!(h.html().contains("aria-disabled=\"true\""), "{}", h.html());
	}

	fn key(code: &str) -> ShortcutKey<'_> {
		ShortcutKey {
			code,
			ctrl: false,
			meta: false,
			alt: false,
			shift: false,
			repeat: false,
		}
	}

	#[test]
	fn the_shortcut_fires_on_ctrl_k_and_cmd_k_on_any_layout() {
		assert_eq!(shortcut_hit(&ShortcutKey { ctrl: true, ..key("KeyK") }), ShortcutHit::Fire);
		// A Russian layout: `key` is "л", `code` stays "KeyK".
		assert_eq!(shortcut_hit(&ShortcutKey { meta: true, ..key("KeyK") }), ShortcutHit::Fire);
	}

	#[test]
	fn the_shortcut_ignores_a_held_key_but_still_keeps_it_from_the_browser() {
		assert_eq!(
			shortcut_hit(&ShortcutKey {
				ctrl: true,
				repeat: true,
				..key("KeyK")
			}),
			ShortcutHit::Held
		);
	}

	#[test]
	fn the_shortcut_ignores_a_bare_k_other_chords_and_other_keys() {
		assert_eq!(shortcut_hit(&key("KeyK")), ShortcutHit::Miss);
		assert_eq!(
			shortcut_hit(&ShortcutKey {
				ctrl: true,
				shift: true,
				..key("KeyK")
			}),
			ShortcutHit::Miss
		);
		assert_eq!(
			shortcut_hit(&ShortcutKey {
				meta: true,
				alt: true,
				..key("KeyK")
			}),
			ShortcutHit::Miss
		);
		// A Dvorak "k" sits on another physical key.
		assert_eq!(shortcut_hit(&ShortcutKey { ctrl: true, ..key("KeyV") }), ShortcutHit::Miss);
	}

	#[test]
	fn the_shortcut_hook_renders_and_does_nothing_without_a_document() {
		fn app() -> Element {
			let mut hits = use_signal(|| 0);
			use_command_palette_shortcut(move || hits += 1, true);
			rsx! { "hits:{hits}" }
		}
		assert!(render_with_effects(app).contains("hits:0"));
	}
}
