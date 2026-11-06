//! The open set of an `Accordion` and the pressed set of a `ToggleGroup`: one
//! `Vec<String>` whichever type the group has — `Single` simply never holds
//! more than one (the mirror of the TS `toArray`).

/// The selection after the item `value` is pressed. In a single group the
/// selected item is released only when `collapsible`; a multiple group always
/// releases it.
pub(crate) fn toggled(selected: &[String], value: &str, multiple: bool, collapsible: bool) -> Vec<String> {
	let present = selected.iter().any(|v| v == value);
	match (multiple, present) {
		(true, true) => selected.iter().filter(|v| *v != value).cloned().collect(),
		(true, false) => selected.iter().cloned().chain([value.to_owned()]).collect(),
		(false, true) if collapsible => Vec::new(),
		(false, true) => selected.to_vec(),
		(false, false) => vec![value.to_owned()],
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn check(selected: &[&str], value: &str, multiple: bool, collapsible: bool, expect: &[&str]) {
		let selected: Vec<String> = selected.iter().map(|s| s.to_string()).collect();
		assert_eq!(toggled(&selected, value, multiple, collapsible), expect, "{value} on {selected:?}");
	}

	#[test]
	fn single_moves_and_releases_only_when_collapsible() {
		check(&[], "a", false, false, &["a"]);
		check(&["a"], "b", false, false, &["b"]);
		check(&["a"], "a", false, false, &["a"]);
		check(&["a"], "a", false, true, &[]);
	}

	#[test]
	fn multiple_adds_in_press_order_and_removes() {
		check(&["b"], "a", true, false, &["b", "a"]);
		check(&["a", "b"], "a", true, false, &["b"]);
	}
}
