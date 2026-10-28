use std::collections::BTreeMap;

use super::{KeyInput, Placement, sort_by_position};

/// One `Command`'s rows and highlight. The search text is the port's
/// controllable state, so every read takes it.
#[derive(Clone, Debug)]
pub(crate) struct Command {
	/// Every mounted item, by id in registration order — including the ones
	/// filtering themselves out, so the empty state can tell "nothing matched"
	/// from "nothing is here".
	rows: BTreeMap<usize, Row>,
	/// The row the user moved the highlight to, and the query it was moved under.
	moved: Option<(usize, String)>,
	/// `false` hands filtering to the caller (rows that are a server response to
	/// the query): every row stays, in the caller's order.
	pub should_filter: bool,
}
impl Command {
	pub(crate) fn new(should_filter: bool) -> Self {
		Self {
			rows: BTreeMap::new(),
			moved: None,
			should_filter,
		}
	}

	/// Registers or updates the row `id`.
	pub(crate) fn register(&mut self, id: usize, row: Row) {
		self.rows.insert(id, row);
	}

	pub(crate) fn unregister(&mut self, id: usize) {
		self.rows.remove(&id);
	}

	pub(crate) fn row(&self, id: usize) -> Option<&Row> {
		self.rows.get(&id)
	}

	/// A row's CSS `order` (`-score`, so the best match is on top), `None` when
	/// it is filtered out.
	pub(crate) fn order(&self, search: &str, value: &str) -> Option<i64> {
		let query = query_of(search);
		if !self.should_filter || query.is_empty() {
			return Some(0);
		}
		fuzzy_score(value, &query).map(|score| -score)
	}

	/// The empty state shows only while a search is under way and no row matches
	/// it — never next to results, and never before the user has typed.
	pub(crate) fn shows_empty(&self, search: &str) -> bool {
		!query_of(search).is_empty() && !self.rows.values().any(|r| self.order(search, &r.value).is_some())
	}

	/// The ids of the rows the keys can reach — rendered and not disabled — in the
	/// order on screen: each group's rows by their CSS `order`, then document order.
	pub(crate) fn reachable(&self, search: &str, placement: &impl Placement<usize>) -> Vec<usize> {
		let mut placed: Vec<_> = self
			.rows
			.iter()
			.filter(|(_, r)| !r.disabled)
			.filter_map(|(id, r)| self.order(search, &r.value).map(|rank| (*id, r.group, rank)))
			.collect();
		sort_by_position(&mut placed, |(id, ..)| placement.placed(*id), |(a, ..), (b, ..)| placement.precedes(*a, *b));
		let mut groups: Vec<Option<usize>> = Vec::new();
		for (_, group, _) in &placed {
			if !groups.contains(group) {
				groups.push(*group);
			}
		}
		placed.sort_by_key(|(_, group, rank)| (groups.iter().position(|g| g == group).expect("collected from `placed`"), *rank));
		placed.into_iter().map(|(id, ..)| id).collect()
	}

	/// The highlighted row: Enter's target and the input's `aria-activedescendant`.
	/// Until the user moves it, it tracks the top row, so results that arrive
	/// after the keystroke (or reorder under it) put Enter on the top hit. Once
	/// moved, it sticks while that row stays and the query is the one it was
	/// moved under.
	pub(crate) fn highlighted(&self, search: &str, placement: &impl Placement<usize>) -> Option<usize> {
		highlight_in(&self.reachable(search, placement), self.moved.as_ref(), &query_of(search))
	}

	/// The pointer resting on a row moves the highlight there.
	pub(crate) fn hover(&mut self, id: usize, search: &str) {
		self.moved = Some((id, query_of(search)));
	}

	/// The search field drives the list: ArrowUp/ArrowDown move the highlight
	/// (stopping at the ends), Enter selects it. Home and End stay the caret's —
	/// the field is editable (ARIA APG editable combobox). `Some` means the key
	/// was the list's, and the port prevents its default.
	pub(crate) fn key(&mut self, input: &KeyInput, search: &str, placement: &impl Placement<usize>) -> Option<CommandKey> {
		if input.composing {
			return None;
		}
		let query = query_of(search);
		let reachable = self.reachable(search, placement);
		let last = reachable.len().checked_sub(1)?;
		let current = highlight_in(&reachable, self.moved.as_ref(), &query).and_then(|h| reachable.iter().position(|r| *r == h));
		let next = match input.key.as_str() {
			"ArrowDown" => current.map_or(0, |at| (at + 1).min(last)),
			"ArrowUp" => current.map_or(last, |at| at.saturating_sub(1)),
			"Enter" => {
				let row = reachable[current?];
				return Some(CommandKey::Select(row, self.rows[&row].value.clone()));
			}
			_ => return None,
		};
		let target = reachable[next];
		self.moved = Some((target, query));
		Some(CommandKey::Moved(target))
	}
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Row {
	pub value: String,
	pub disabled: bool,
	/// What the input's `aria-activedescendant` names while this row is highlighted.
	pub dom_id: String,
	/// The group it renders in; `order` only reorders siblings, so keyboard order is per group.
	pub group: Option<usize>,
}

/// What a key asks the port to do.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum CommandKey {
	/// The highlight moved onto this row: scroll it into view.
	Moved(usize),
	/// Enter on this row: fire its `on_select` with its value.
	Select(usize, String),
}

/// The active query: trimmed, so blank input is not a search, and lowercased
/// for the case-insensitive compare. Shared by the filter, the highlight and the
/// empty-state gate so they can never disagree.
fn query_of(search: &str) -> String {
	search.trim().to_lowercase()
}

/// The highlighted row among `reachable` (in screen order): the one the user
/// moved to, while it stays and the query is the one it was moved under; else
/// the top row.
fn highlight_in(reachable: &[usize], moved: Option<&(usize, String)>, query: &str) -> Option<usize> {
	match moved {
		Some((id, under)) if under == query && reachable.contains(id) => Some(*id),
		_ => reachable.first().copied(),
	}
}

/// fzf v1, as the TS port's `fuzzyScore`: per whitespace-separated term, the
/// shortest window ending at the first full subsequence match, scored for word
/// starts and runs, charged for gaps. `None` when a term does not match.
fn fuzzy_score(value: &str, query: &str) -> Option<i64> {
	let hay: Vec<char> = value.to_lowercase().chars().collect();
	let mut total = 0;
	for term in query.split_whitespace() {
		let needle: Vec<char> = term.chars().collect();
		let mut k = 0;
		let end = hay.iter().position(|c| {
			if *c == needle[k] {
				k += 1;
			}
			k == needle.len()
		})?;
		let mut start = end;
		let mut j = needle.len();
		for i in (0..=end).rev() {
			if hay[i] == needle[j - 1] {
				start = i;
				j -= 1;
				if j == 0 {
					break;
				}
			}
		}
		let mut prev = None;
		let mut j = 0;
		for i in start..=end {
			if hay[i] != needle[j] {
				continue;
			}
			total += 16;
			if i == 0 || !hay[i - 1].is_alphanumeric() {
				total += 8;
			}
			if let Some(p) = prev {
				total += if i == p + 1 { 4 } else { -(3 + (i - p - 2) as i64) };
			}
			prev = Some(i);
			j += 1;
			if j == needle.len() {
				break;
			}
		}
	}
	Some(total)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::Unplaced;

	fn command(search_filter: bool, rows: &[(&str, bool, Option<usize>)]) -> Command {
		let mut c = Command::new(search_filter);
		for (id, (value, disabled, group)) in rows.iter().enumerate() {
			c.register(
				id,
				Row {
					value: value.to_string(),
					disabled: *disabled,
					dom_id: format!("row-{id}"),
					group: *group,
				},
			);
		}
		c
	}

	/// The values `query` leaves among `values`, in the order the arrows walk them.
	fn walk(values: &[&str], query: &str) -> Vec<String> {
		let c = command(true, &values.iter().map(|v| (*v, false, None)).collect::<Vec<_>>());
		c.reachable(query, &Unplaced).into_iter().map(|id| c.rows[&id].value.clone()).collect()
	}

	#[test]
	fn fuzzy_matching_ranked_as_fzf_ranks() {
		assert_eq!(walk(&["Apple", "Banana"], "bna"), ["Banana"]);
		assert_eq!(walk(&["Apple", "Banana"], "pe"), ["Apple"]);
		assert!(walk(&["Apple", "Banana"], "ea").is_empty());
		assert_eq!(
			walk(&["gary.lee.nnn@xa.com", "glennamonitti@gmail.com"], "glenna"),
			["glennamonitti@gmail.com", "gary.lee.nnn@xa.com"]
		);
		assert_eq!(walk(&["xmonx@a.com", "bob.mon@a.com"], "mon"), ["bob.mon@a.com", "xmonx@a.com"]);
		assert_eq!(walk(&["ann@gmail.com", "ann@proton.me", "bo@gmail.com"], "ann gm"), ["ann@gmail.com"]);
		assert_eq!(walk(&["b@a.com", "a@a.com"], "@a"), ["b@a.com", "a@a.com"]);
	}

	#[test]
	fn ranking_stays_within_each_group() {
		let c = command(true, &[("xmonx", false, Some(0)), ("xxmonxx", false, Some(1)), ("mon", false, Some(1))]);
		assert_eq!(c.reachable("mon", &Unplaced), [0, 2, 1]);
	}

	#[test]
	fn the_empty_state_waits_for_a_query_that_matches_nothing() {
		let c = command(true, &[("Apple", false, None), ("Banana", false, None)]);
		assert!(!c.shows_empty(""), "not next to the unfiltered list");
		assert!(!c.shows_empty("   "), "blank input is not a query (the TS port's `search.trim()`)");
		assert!(!c.shows_empty("ban"));
		assert!(c.shows_empty("zzz"));
		assert!(command(true, &[]).shows_empty("zzz"), "an itemless list is still 'no results'");
		assert!(!command(false, &[("Apple", false, None)]).shows_empty("zzz"), "unfiltered, the mounted rows are the results");
	}

	#[test]
	fn arrows_move_the_highlight_past_disabled_rows_without_wrapping() {
		let mut c = command(true, &[("Apple", false, None), ("Banana", true, None), ("Cherry", false, None), ("Date", false, None)]);
		assert_eq!(c.highlighted("", &Unplaced), Some(0));
		assert_eq!(c.key(&"ArrowDown".into(), "", &Unplaced), Some(CommandKey::Moved(2)), "the disabled Banana is skipped");
		c.key(&"ArrowDown".into(), "", &Unplaced);
		assert_eq!(c.key(&"ArrowDown".into(), "", &Unplaced), Some(CommandKey::Moved(3)), "the last row holds");
		assert_eq!(c.key(&"Home".into(), "", &Unplaced), None, "Home stays with the caret");
		assert_eq!(c.key(&"Enter".into(), "", &Unplaced), Some(CommandKey::Select(3, "Date".to_owned())));
		let mut c = command(true, &[("Apple", false, None), ("Cherry", false, None)]);
		assert_eq!(c.key(&"ArrowUp".into(), "", &Unplaced), Some(CommandKey::Moved(0)), "the first row holds");
	}

	#[test]
	fn a_composing_key_is_the_ime_s() {
		let mut c = command(true, &[("Apple", false, None)]);
		let input = KeyInput { composing: true, .."Enter".into() };
		assert_eq!(c.key(&input, "", &Unplaced), None);
	}

	#[test]
	fn the_highlight_follows_the_filter() {
		let mut c = command(true, &[("Apple", false, None), ("Banana", false, None), ("Mango", false, None)]);
		c.key(&"ArrowDown".into(), "an", &Unplaced);
		assert_eq!(c.highlighted("an", &Unplaced), Some(2), "only the rows the filter kept");
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
	fn a_keyed_resort_moves_the_default_highlight_and_the_arrows() {
		// Registered Apple (0), Apricot (1); the server re-sorts Apricot on top.
		let mut c = command(false, &[("Apple", false, None), ("Apricot", false, None)]);
		let screen = Screen(vec![1, 0]);
		assert_eq!(c.reachable("ap", &screen), [1, 0]);
		assert_eq!(c.highlighted("ap", &screen), Some(1), "Enter goes to the top row on screen");
		c.hover(0, "ap");
		assert_eq!(c.highlighted("ap", &screen), Some(0), "a moved highlight stays on its row");
	}

	#[test]
	fn a_moved_highlight_resets_with_the_query_or_its_row() {
		let mut c = command(false, &[("a", false, None), ("b", false, None)]);
		c.hover(1, "a");
		assert_eq!(c.highlighted("ab", &Unplaced), Some(0), "a new query starts at the top");
		c.unregister(1);
		assert_eq!(c.highlighted("a", &Unplaced), Some(0), "a gone row hands back to the top");
	}
}
