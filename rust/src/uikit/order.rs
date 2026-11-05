/// Where a component's parts sit in the document, as only the port can tell: the
/// order the user sees, which a keyed re-sort or a part inserted above the others
/// makes differ from the order the parts registered in.
pub(crate) trait Placement<Id> {
	/// Whether `id` is in the document. Nothing is under SSR or before mount.
	fn placed(&self, id: Id) -> bool;
	/// Whether `a` comes before `b`. Only asked of placed ids.
	fn precedes(&self, a: Id, b: Id) -> bool;
}

/// Nothing placed: registration order stands.
#[cfg(test)]
pub(crate) struct Unplaced;
#[cfg(test)]
impl<Id> Placement<Id> for Unplaced {
	fn placed(&self, _: Id) -> bool {
		false
	}

	fn precedes(&self, _: Id, _: Id) -> bool {
		false
	}
}

/// Sorts `items` by `precedes` when every item is `placed`, leaving them as they
/// are otherwise — O(n) `placed` checks, then O(n) comparisons for a list already
/// in order and O(n log n) for one that is not. Each comparison is a call into
/// the DOM, so an n² pass over a long list would stall every key.
///
/// A merge sort of our own rather than `sort_by`: `sort_by` may panic on an
/// order that is not total, and a DOM comparison is one only while every node
/// is in the document — this one stays a permutation whatever `precedes` says.
pub(crate) fn sort_by_position<T: Clone>(items: &mut [T], placed: impl Fn(&T) -> bool, precedes: impl Fn(&T, &T) -> bool) {
	if !items.iter().all(&placed) {
		return;
	}
	if items.windows(2).all(|pair| !precedes(&pair[1], &pair[0])) {
		return;
	}
	merge_sort(items, &precedes);
}

fn merge_sort<T: Clone>(items: &mut [T], precedes: &impl Fn(&T, &T) -> bool) {
	if items.len() < 2 {
		return;
	}
	let mid = items.len() / 2;
	merge_sort(&mut items[..mid], precedes);
	merge_sort(&mut items[mid..], precedes);
	let mut merged = Vec::with_capacity(items.len());
	let (mut i, mut j) = (0, mid);
	while i < mid && j < items.len() {
		// Strictly before, so equals keep their order: the sort is stable.
		if precedes(&items[j], &items[i]) {
			merged.push(items[j].clone());
			j += 1;
		} else {
			merged.push(items[i].clone());
			i += 1;
		}
	}
	merged.extend_from_slice(&items[i..mid]);
	merged.extend_from_slice(&items[j..]);
	items.clone_from_slice(&merged);
}

#[cfg(test)]
mod tests {
	use super::*;

	/// `(registration id, place on screen)`; the sort sees only the place.
	fn by_place(items: &mut [(usize, Option<usize>)]) {
		sort_by_position(items, |i| i.1.is_some(), |a, b| a.1 < b.1);
	}

	#[test]
	fn a_keyed_resort_walks_in_document_order() {
		// Registered Apple (0), Apricot (1); re-sorted to Apricot above Apple.
		let mut items = [(0, Some(1)), (1, Some(0))];
		by_place(&mut items);
		assert_eq!(items.map(|i| i.0), [1, 0]);
	}

	#[test]
	fn a_row_inserted_above_the_mounted_ones_comes_first() {
		let mut items = [(0, Some(1)), (1, Some(2)), (2, Some(0))];
		by_place(&mut items);
		assert_eq!(items.map(|i| i.0), [2, 0, 1]);
	}

	#[test]
	fn an_unplaced_item_keeps_registration_order() {
		let mut items = [(0, Some(1)), (1, None), (2, Some(0))];
		by_place(&mut items);
		assert_eq!(items.map(|i| i.0), [0, 1, 2]);
	}

	#[test]
	fn the_document_sort_asks_n_log_n_comparisons_not_n_squared() {
		use std::cell::Cell;
		let n = 1000;
		let calls = Cell::new(0_usize);
		let count = |a: &usize, b: &usize| {
			calls.set(calls.get() + 1);
			a < b
		};
		let mut in_order: Vec<usize> = (0..n).collect();
		sort_by_position(&mut in_order, |_| true, count);
		assert_eq!(calls.get(), n - 1, "a list already in order costs one pass");
		calls.set(0);
		// A keyed re-sort that reverses the list: the worst case for the old insertion sort.
		let mut reversed: Vec<usize> = (0..n).rev().collect();
		sort_by_position(&mut reversed, |_| true, count);
		assert_eq!(reversed, (0..n).collect::<Vec<_>>());
		assert!(calls.get() < 2 * n * 10, "{} comparisons for {n} items", calls.get());
	}

	#[test]
	fn the_document_sort_is_stable_and_survives_an_inconsistent_order() {
		let mut items = [(0, 1), (1, 0), (2, 1), (3, 0)];
		sort_by_position(&mut items, |_| true, |a, b| a.1 < b.1);
		assert_eq!(items.map(|i| i.0), [1, 3, 0, 2]);
		// A comparison that is no order at all still leaves a permutation, never a panic.
		let mut items: Vec<usize> = (0..50).collect();
		sort_by_position(&mut items, |_| true, |a, b| (a * 7 + b * 3) % 5 == 0);
		items.sort_unstable();
		assert_eq!(items, (0..50).collect::<Vec<_>>());
	}
}
