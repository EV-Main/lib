use std::collections::BTreeMap;

use super::KeyInput;

/// Which edge of its anchor an overlay is placed against. Rendered as a
/// `data-side` attribute so CSS positions and animates the overlay; the kit
/// does not measure the viewport (the TS `useFloating` does).
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum Side {
	Top,
	Right,
	#[default]
	Bottom,
	Left,
}

/// What dismissed a layer — for a caller that treats keys and pointers apart (a
/// Select hands focus back to its trigger after Escape, never after a click
/// elsewhere). The mirror of the TS `DismissEvent`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DismissReason {
	Escape,
	Outside,
}

/// Which arrow keys walk a [`Roving`] group. `Home`/`End` always do.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum RovingOrientation {
	Horizontal,
	#[default]
	Vertical,
	Both,
}

/// Arrow-key roving focus over a group of items — the mirror of the TS
/// `useRovingFocus`, and what earns a group the right to take its items out of
/// the tab order: the group is one tab stop, and the arrows move within it.
///
/// Items are keyed by the same value the group selects on, so navigating is
/// "find the neighbour of the selected key". Movement wraps at both ends, as
/// `useRovingFocus`'s default `loop: true` does. Items walk in registration
/// order, which the ports make render order.
#[derive(Clone, Debug, Default)]
pub(crate) struct Roving {
	items: BTreeMap<usize, String>,
	orientation: RovingOrientation,
}
impl Roving {
	pub(crate) fn new(orientation: RovingOrientation) -> Self {
		Self {
			items: BTreeMap::new(),
			orientation,
		}
	}

	/// Registers or re-keys the item `id`.
	pub(crate) fn register(&mut self, id: usize, key: String) {
		self.items.insert(id, key);
	}

	pub(crate) fn unregister(&mut self, id: usize) {
		self.items.remove(&id);
	}

	/// The item keyed `key`, for the port to focus.
	pub(crate) fn id_of(&self, key: &str) -> Option<usize> {
		self.items.iter().find(|(_, k)| *k == key).map(|(id, _)| *id)
	}

	/// The key `input` walks to, starting from the item keyed `from`; `None` when
	/// `input` isn't a navigation key for this orientation, or the group is empty.
	pub(crate) fn next(&self, input: &KeyInput, from: &str) -> Option<String> {
		let keys: Vec<&str> = self.items.values().map(String::as_str).collect();
		let last = keys.len().checked_sub(1)?;
		// An unknown `from` (nothing selected yet) walks as if from the first
		// item, so the first arrow press lands on a real neighbour.
		let at = keys.iter().position(|k| *k == from).unwrap_or(0);
		let horizontal = matches!(self.orientation, RovingOrientation::Horizontal | RovingOrientation::Both);
		let vertical = matches!(self.orientation, RovingOrientation::Vertical | RovingOrientation::Both);
		let forward = |at: usize| if at == last { 0 } else { at + 1 };
		let back = |at: usize| if at == 0 { last } else { at - 1 };
		let to = match input.key.as_str() {
			"ArrowDown" if vertical => forward(at),
			"ArrowUp" if vertical => back(at),
			"ArrowRight" if horizontal => forward(at),
			"ArrowLeft" if horizontal => back(at),
			"Home" => 0,
			"End" => last,
			_ => return None,
		};
		Some(keys[to].to_owned())
	}

	/// Whether the item keyed `key` is the group's single tab stop: the selected
	/// item, or — while nothing is selected — the first item, so `Tab` always
	/// enters the group exactly once and never lands nowhere.
	pub(crate) fn is_tab_stop(&self, key: &str, selected: &str) -> bool {
		if key == selected {
			return true;
		}
		let mut keys = self.items.values().map(String::as_str);
		!keys.clone().any(|k| k == selected) && keys.next() == Some(key)
	}
}

/// The open layers of one page, oldest first — the Rust side of the TS
/// `useDismissableLayer`.
///
/// Escape or a click outside only ever dismisses the topmost layer: a Select's
/// list inside a Dialog closes the list, not the Dialog. The kit renders
/// overlays inline, so a key or a click meant for the top layer can reach a
/// lower one — the Escape from a field of the Dialog while its Popover is open,
/// a click on the Dialog's own scrim below the list. A lower layer that hears
/// one hands it to the top instead of acting on it itself.
///
/// The stack follows renders, not handlers: a layer that closes itself stays
/// on top until it re-renders closed, so the Escape that closed it cannot also
/// close the layer below on its way up. Ids only — the port keeps each layer's
/// `on_dismiss`.
#[derive(Clone, Debug, Default)]
pub(crate) struct LayerStack(Vec<usize>);
impl LayerStack {
	/// Called on each render of layer `id`. Re-rendering an open layer keeps its
	/// place: re-pushing would move it above the layers opened after it.
	pub(crate) fn sync(&mut self, id: usize, open: bool) {
		let at = self.0.iter().position(|l| *l == id);
		match (open, at) {
			(true, None) => self.0.push(id),
			(false, Some(at)) => {
				self.0.remove(at);
			}
			(true, Some(_)) | (false, None) => {}
		}
	}

	pub(crate) fn remove(&mut self, id: usize) {
		self.0.retain(|l| *l != id);
	}

	pub(crate) fn top(&self) -> Option<usize> {
		self.0.last().copied()
	}

	/// The layer an Escape heard by layer `id` dismisses: the topmost, when `id`
	/// is open. A closed layer leaves the key alone, for the layer around it,
	/// and the port lets it propagate.
	pub(crate) fn escape(&self, id: usize, input: &KeyInput) -> Option<usize> {
		(input.key == "Escape" && self.0.contains(&id)).then(|| self.top()).flatten()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn escape_goes_to_the_top_layer_even_from_a_lower_one() {
		let mut stack = LayerStack::default();
		stack.sync(0, true);
		stack.sync(1, true);
		assert_eq!(stack.escape(0, &"Escape".into()), Some(1));
		stack.sync(0, true);
		assert_eq!(stack.top(), Some(1), "a lower layer re-rendering keeps its place");
		stack.sync(1, false);
		assert_eq!(stack.escape(0, &"Escape".into()), Some(0));
	}

	#[test]
	fn a_closed_layer_leaves_escape_to_the_one_around_it() {
		let mut stack = LayerStack::default();
		stack.sync(0, true);
		stack.sync(1, false);
		assert_eq!(stack.escape(1, &"Escape".into()), None);
		assert_eq!(stack.escape(0, &"Enter".into()), None);
	}

	#[test]
	fn arrows_wrap_and_follow_the_orientation() {
		let mut roving = Roving::new(RovingOrientation::Horizontal);
		for (id, key) in ["a", "b", "c"].into_iter().enumerate() {
			roving.register(id, key.to_owned());
		}
		assert_eq!(roving.next(&"ArrowRight".into(), "c").as_deref(), Some("a"));
		assert_eq!(roving.next(&"ArrowLeft".into(), "a").as_deref(), Some("c"));
		assert_eq!(roving.next(&"ArrowDown".into(), "a"), None, "vertical arrows belong to the page");
		assert_eq!(roving.next(&"End".into(), "").as_deref(), Some("c"));
		assert_eq!(roving.next(&"ArrowRight".into(), "").as_deref(), Some("b"), "nothing selected walks from the first");
	}

	#[test]
	fn the_tab_stop_is_the_selected_item_or_else_the_first() {
		let mut roving = Roving::new(RovingOrientation::Vertical);
		roving.register(0, "a".to_owned());
		roving.register(1, "b".to_owned());
		assert!(roving.is_tab_stop("b", "b") && !roving.is_tab_stop("a", "b"));
		assert!(roving.is_tab_stop("a", "") && !roving.is_tab_stop("b", ""));
	}
}
