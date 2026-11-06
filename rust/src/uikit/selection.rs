//! The open set of an `Accordion` and the pressed set of a `ToggleGroup`: one
//! `Vec<String>` whichever type the group has — `Single` simply never holds
//! more than one (the mirror of the TS `toArray`).

pub(crate) const ACCORDION_CHEVRON: &str = "text-ink-soft pointer-events-none size-4 shrink-0 translate-y-0.5 transition-transform duration-200";
pub(crate) const TOGGLE_GROUP_ROOT: &str = "group/toggle-group flex w-fit items-center rounded-md data-[variant=outline]:shadow-xs";
/// Layered over `toggle_classes`: the group's adjacency utilities.
pub(crate) const TOGGLE_GROUP_ITEM: &str = "min-w-0 flex-auto shrink-0 rounded-none shadow-none first:rounded-l-md last:rounded-r-md focus:z-10 focus-visible:z-10 data-[variant=outline]:-ml-px data-[variant=outline]:first:ml-0 data-[state=on]:z-10";

/// `Single` keeps at most one item open; `Multiple` allows several.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum AccordionType {
	#[default]
	Single,
	Multiple,
}

/// `Single` keeps at most one item pressed; `Multiple` allows several.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ToggleGroupType {
	#[default]
	Single,
	Multiple,
}

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
