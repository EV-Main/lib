use std::collections::{BTreeMap, BTreeSet};

use super::{KeyInput, Placement, sort_by_position};

/// Keys typed within this long of each other build one type-ahead query.
const TYPEAHEAD_MS: f64 = 500.0;

/// One `Select`'s open list: its options, the one holding focus, the type-ahead.
/// The chosen value is the port's controllable state, so every read takes it.
#[derive(Clone, Debug, Default)]
pub(crate) struct Select {
	/// Every rendered option, by id in registration order.
	options: BTreeMap<usize, SelectOption>,
	/// The options whose element has mounted, so focus can land on them.
	mounted: BTreeSet<usize>,
	/// The option holding focus while the list is open; `None` before the list
	/// has landed on one, when the keys walk from where it would land.
	active: Option<String>,
	typed: Typeahead,
}
impl Select {
	pub(crate) fn register(&mut self, id: usize, option: SelectOption) {
		self.options.insert(id, option);
	}

	pub(crate) fn unregister(&mut self, id: usize) {
		self.options.remove(&id);
		self.mounted.remove(&id);
	}

	pub(crate) fn option(&self, id: usize) -> Option<&SelectOption> {
		self.options.get(&id)
	}

	pub(crate) fn active(&self) -> Option<&str> {
		self.active.as_deref()
	}

	/// The list closed: the next open lands afresh.
	pub(crate) fn close(&mut self) {
		self.active = None;
	}

	/// The option `value` took focus — a click on a disabled one too, so the
	/// arrows go on from it.
	pub(crate) fn focused(&mut self, value: String) {
		self.active = Some(value);
	}

	/// The element of option `id` mounted. Returns the option opening lands on,
	/// for the port to focus, once it can tell: the chosen option the moment it
	/// mounts, else the first on screen once every option has.
	///
	/// Decided without the document-order sort while the chosen option is
	/// reachable: one sort per mount would cost ~n² DOM calls on opening a long list.
	pub(crate) fn mounted(&mut self, id: usize, chosen: &str, placement: &impl Placement<usize>) -> Option<usize> {
		let value = self.options.get(&id)?.value.clone();
		self.mounted.insert(id);
		if self.active.is_some() {
			return None;
		}
		if self.options.values().any(|o| !o.disabled && o.value == chosen) {
			return (value == chosen).then(|| self.land(id, value));
		}
		if !self.options.keys().all(|id| self.mounted.contains(id)) {
			return None;
		}
		let (id, value) = self.landing(&self.on_screen(placement), chosen)?;
		Some(self.land(id, value))
	}

	/// Arrows (wrapping, past disabled options), Home, End, Enter, Space, Tab and
	/// type-ahead on the open list. `None` leaves the key to the page.
	pub(crate) fn key(&mut self, input: &KeyInput, chosen: &str, now_ms: Option<f64>, placement: &impl Placement<usize>) -> Option<SelectKey> {
		let all = self.on_screen(placement);
		let enabled: Vec<&Listed> = all.iter().filter(|o| !o.disabled).collect();
		let current = self.active.clone().or_else(|| self.landing(&all, chosen).map(|(_, value)| value));
		let at = current.as_ref().and_then(|c| enabled.iter().position(|o| o.value == *c));
		// On a focused disabled option: how many reachable options come before it,
		// so the arrows and the type-ahead go on from where it sits.
		let before = match (at, &current) {
			(None, Some(c)) => all.iter().position(|o| o.value == *c).map(|i| all[..i].iter().filter(|o| !o.disabled).count()),
			_ => None,
		};
		// Only an option the keys can reach is chosen — never a disabled one,
		// not even the previous active one behind it.
		let choose = || at.map_or(SelectKey::Held, |at| SelectKey::Choose(enabled[at].value.clone()));
		let last = enabled.len().checked_sub(1);
		let to = match (input.key.as_str(), last) {
			("ArrowDown", Some(last)) => match (at, before) {
				(Some(at), _) if at == last => 0,
				(Some(at), _) => at + 1,
				(None, Some(before)) if before > last => 0,
				(None, Some(before)) => before,
				(None, None) => 0,
			},
			("ArrowUp", Some(last)) => match (at, before) {
				(Some(0), _) => last,
				(Some(at), _) => at - 1,
				(None, Some(0)) => last,
				(None, Some(before)) => before - 1,
				(None, None) => last,
			},
			("Home", Some(_)) => 0,
			("End", Some(last)) => last,
			("Enter", _) => return Some(choose()),
			// Mid-query a space is part of the name ("united s"), as in Radix.
			(" ", _) if !self.typed.is_typing(now_ms) => return Some(choose()),
			("Tab", _) => return Some(SelectKey::Leave),
			(key, _) => {
				let mut chars = key.chars();
				let (Some(ch), None) = (chars.next(), chars.next()) else { return None };
				if input.ctrl || input.meta || input.alt {
					return None;
				}
				let labels: Vec<String> = enabled.iter().map(|o| o.label.to_lowercase()).collect();
				let from = at.or_else(|| before.and_then(|b| b.checked_sub(1)));
				let query = self.typed.push(ch, now_ms);
				let Some(to) = typeahead_match(&labels, from, &query) else {
					return Some(SelectKey::Held);
				};
				to
			}
		};
		let (id, value) = (enabled[to].id, enabled[to].value.clone());
		Some(SelectKey::Moved(self.land(id, value)))
	}

	fn land(&mut self, id: usize, value: String) -> usize {
		self.active = Some(value);
		id
	}

	/// Every option, disabled ones too, in the order on screen.
	fn on_screen(&self, placement: &impl Placement<usize>) -> Vec<Listed> {
		let mut all: Vec<(usize, &SelectOption)> = self.options.iter().map(|(id, o)| (*id, o)).collect();
		sort_by_position(&mut all, |(id, _)| placement.placed(*id), |(a, _), (b, _)| placement.precedes(*a, *b));
		all.into_iter()
			.map(|(id, o)| Listed {
				id,
				value: o.value.clone(),
				label: o.label.clone(),
				disabled: o.disabled,
			})
			.collect()
	}

	/// Where opening lands: the chosen option, else the first, as a native select does.
	fn landing(&self, all: &[Listed], chosen: &str) -> Option<(usize, String)> {
		let mut enabled = all.iter().filter(|o| !o.disabled);
		let first = enabled.clone().next();
		enabled.find(|o| o.value == chosen).or(first).map(|o| (o.id, o.value.clone()))
	}
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectOption {
	pub value: String,
	/// What the type-ahead matches, as the caller wrote it.
	pub label: String,
	pub disabled: bool,
}

/// What a key on the open list asks the port to do.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SelectKey {
	/// Focus moves onto this option.
	Moved(usize),
	/// Enter or Space chose this value: set it and close.
	Choose(String),
	/// Tab: close and let the browser move focus on — without a portal the list
	/// sits right after its trigger, so native order already goes on from there.
	Leave,
	/// The list's key, with nothing to do (Enter on a disabled option, a letter
	/// that matches nothing).
	Held,
}

#[derive(Clone, Debug)]
struct Listed {
	id: usize,
	value: String,
	label: String,
	disabled: bool,
}

#[derive(Clone, Debug, Default)]
struct Typeahead {
	text: String,
	at: Option<f64>,
}
impl Typeahead {
	/// Whether a query is under way: something typed, within the window.
	fn is_typing(&self, now: Option<f64>) -> bool {
		!self.text.is_empty() && matches!((now, self.at), (Some(now), Some(at)) if now - at <= TYPEAHEAD_MS)
	}

	/// Adds `ch` to the query, starting a new one after a pause — or on every
	/// key where the port has no clock, which still leaves a repeated letter cycling.
	fn push(&mut self, ch: char, now: Option<f64>) -> String {
		let fresh = match (now, self.at) {
			(Some(now), Some(at)) => now - at > TYPEAHEAD_MS,
			_ => true,
		};
		if fresh {
			self.text.clear();
		}
		self.text.extend(ch.to_lowercase());
		self.at = now;
		self.text.clone()
	}
}

/// The option the type-ahead `typed` lands on, from the option at `current`.
/// One letter repeated cycles through the options that start with it; a word
/// stays on the current option while it still matches. `labels` are lowercased.
fn typeahead_match(labels: &[String], current: Option<usize>, typed: &str) -> Option<usize> {
	let first = typed.chars().next()?;
	let repeated = typed.chars().all(|c| c == first);
	let query = if repeated { first.to_string() } else { typed.to_string() };
	let from = match current {
		Some(at) if repeated => at + 1,
		Some(at) => at,
		None => 0,
	};
	(0..labels.len()).map(|i| (from + i) % labels.len()).find(|&i| labels[i].trim_start().starts_with(&query))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::Unplaced;

	/// `(value, disabled)`, each labelled by its capitalised value.
	fn select(options: &[(&str, bool)]) -> Select {
		let mut s = Select::default();
		for (id, (value, disabled)) in options.iter().enumerate() {
			let mut label = value.to_string();
			label[..1].make_ascii_uppercase();
			s.register(
				id,
				SelectOption {
					value: value.to_string(),
					label,
					disabled: *disabled,
				},
			);
		}
		s
	}

	const FRUIT: &[(&str, bool)] = &[("apple", false), ("banana", false), ("blueberry", false), ("cherry", true), ("date", false)];

	/// What `keys` on the freshly opened list choose; every key lands within one type-ahead window.
	fn pick(s: &mut Select, chosen: &str, keys: &[&str]) -> Option<String> {
		keys.iter().find_map(|key| match s.key(&(*key).into(), chosen, Some(0.0), &Unplaced) {
			Some(SelectKey::Choose(value)) => Some(value),
			_ => None,
		})
	}

	fn picks(chosen: &str, keys: &[&str]) -> Option<String> {
		pick(&mut select(FRUIT), chosen, keys)
	}

	#[test]
	fn arrows_walk_from_where_opening_lands_and_enter_chooses() {
		assert_eq!(picks("", &["ArrowDown", "Enter"]).as_deref(), Some("banana"), "opening lands on the first option");
		assert_eq!(picks("banana", &["ArrowDown", "Enter"]).as_deref(), Some("blueberry"), "opening lands on the chosen option");
		assert_eq!(picks("", &["ArrowDown", " "]).as_deref(), Some("banana"), "Space chooses like Enter");
	}

	#[test]
	fn arrows_skip_disabled_options_and_wrap() {
		assert_eq!(picks("", &["End", "ArrowUp", "Enter"]).as_deref(), Some("blueberry"), "the disabled cherry is skipped");
		assert_eq!(picks("", &["ArrowUp", "Enter"]).as_deref(), Some("date"), "up from the first wraps to the last");
		assert_eq!(picks("", &["End", "ArrowDown", "Enter"]).as_deref(), Some("apple"), "down from the last wraps to the first");
		assert_eq!(picks("", &["End", "Home", "Enter"]).as_deref(), Some("apple"));
	}

	#[test]
	fn typeahead_finds_by_letters() {
		assert_eq!(picks("", &["b", "Enter"]).as_deref(), Some("banana"));
		assert_eq!(picks("", &["b", "b", "Enter"]).as_deref(), Some("blueberry"), "a repeated letter cycles");
		assert_eq!(picks("", &["b", "l", "Enter"]).as_deref(), Some("blueberry"), "letters build a word");
		assert_eq!(picks("", &["c", "Enter"]).as_deref(), Some("apple"), "a disabled option is not a match");
		let mut s = select(&[("apple", false), ("united kingdom", false), ("united states", false)]);
		let keys: Vec<String> = "united s".chars().map(String::from).chain(["Enter".to_owned()]).collect();
		let keys: Vec<&str> = keys.iter().map(String::as_str).collect();
		assert_eq!(pick(&mut s, "", &keys).as_deref(), Some("united states"), "mid-query a space is part of it");
	}

	#[test]
	fn a_modified_letter_and_a_named_key_are_the_page_s() {
		let mut s = select(FRUIT);
		assert_eq!(s.key(&KeyInput { ctrl: true, .."b".into() }, "", Some(0.0), &Unplaced), None);
		assert_eq!(s.key(&"F2".into(), "", Some(0.0), &Unplaced), None);
		assert_eq!(s.key(&"Tab".into(), "", Some(0.0), &Unplaced), Some(SelectKey::Leave));
	}

	#[test]
	fn a_focused_disabled_option_is_never_chosen_but_the_arrows_go_on_from_it() {
		let three = [("apple", false), ("cherry", false), ("banana", true)];
		for key in ["Enter", " "] {
			let mut s = select(&three);
			s.focused("banana".to_owned());
			assert_eq!(s.key(&key.into(), "", Some(0.0), &Unplaced), Some(SelectKey::Held), "not even the option focused before");
		}
		let mut s = select(&three);
		s.focused("banana".to_owned());
		assert_eq!(pick(&mut s, "", &["ArrowUp", "Enter"]).as_deref(), Some("cherry"));
		let mut s = select(&three);
		s.focused("banana".to_owned());
		assert_eq!(pick(&mut s, "", &["ArrowDown", "Enter"]).as_deref(), Some("apple"), "down from the last wraps");
	}

	#[test]
	fn closing_lets_the_next_open_land_again() {
		let mut s = select(FRUIT);
		s.focused("date".to_owned());
		s.close();
		assert_eq!(pick(&mut s, "", &["Enter"]).as_deref(), Some("apple"));
	}

	#[test]
	fn typeahead_query_restarts_after_a_pause() {
		let mut t = Typeahead::default();
		assert_eq!(t.push('B', Some(0.0)), "b");
		assert_eq!(t.push('l', Some(100.0)), "bl");
		assert_eq!(t.push('c', Some(700.0)), "c");
		assert_eq!(t.push('h', None), "h", "no clock: every key starts afresh");
	}

	#[test]
	fn typeahead_match_cycles_a_repeated_letter_and_holds_a_word() {
		let labels: Vec<String> = ["apple", "banana", "blueberry", "date"].map(String::from).to_vec();
		assert_eq!(typeahead_match(&labels, None, "b"), Some(1));
		assert_eq!(typeahead_match(&labels, Some(1), "bb"), Some(2));
		assert_eq!(typeahead_match(&labels, Some(2), "b"), Some(1), "wraps past the end");
		assert_eq!(typeahead_match(&labels, Some(1), "ba"), Some(1), "a word stays while it matches");
		assert_eq!(typeahead_match(&labels, Some(0), "z"), None);
	}

	/// `(registration id, place on screen)`.
	struct Screen(Vec<usize>);
	impl Placement<usize> for Screen {
		fn placed(&self, _: usize) -> bool {
			true
		}

		fn precedes(&self, a: usize, b: usize) -> bool {
			self.0[a] < self.0[b]
		}
	}

	#[test]
	fn opening_lands_on_the_chosen_option_as_it_mounts() {
		let mut s = select(FRUIT);
		assert_eq!(s.mounted(0, "date", &Unplaced), None);
		assert_eq!(s.mounted(4, "date", &Unplaced), Some(4));
		assert_eq!(s.mounted(1, "date", &Unplaced), None, "landed once");
	}

	#[test]
	fn with_nothing_chosen_opening_lands_on_the_first_on_screen_once_all_mounted() {
		// Registered banana (0), apple (1); a keyed re-sort put apple on top.
		let mut s = select(&[("banana", false), ("apple", false)]);
		let screen = Screen(vec![1, 0]);
		assert_eq!(s.mounted(0, "", &screen), None, "apple may still be coming");
		assert_eq!(s.mounted(1, "", &screen), Some(1));
		assert_eq!(pick(&mut s, "", &["Enter"]).as_deref(), Some("apple"));
		let mut s = select(&[("banana", false), ("apple", false)]);
		assert_eq!(s.key(&"ArrowDown".into(), "", Some(0.0), &screen), Some(SelectKey::Moved(0)), "the arrows walk the screen too");
	}
}
