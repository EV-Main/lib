use std::collections::BTreeSet;

use super::KeyInput;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CarouselOrientation {
	#[default]
	Horizontal,
	Vertical,
}
impl CarouselOrientation {
	/// The track's `transform` showing slide `index`.
	pub(crate) fn transform(self, index: usize) -> String {
		match self {
			Self::Horizontal => format!("translate3d(-{}%, 0, 0)", index * 100),
			Self::Vertical => format!("translate3d(0, -{}%, 0)", index * 100),
		}
	}
}

/// One carousel's slides; the index is the port's controllable state, so every
/// read takes it.
#[derive(Clone, Debug, Default)]
pub(crate) struct Carousel {
	slides: BTreeSet<usize>,
}
impl Carousel {
	pub(crate) fn register(&mut self, id: usize) {
		self.slides.insert(id);
	}

	pub(crate) fn unregister(&mut self, id: usize) {
		self.slides.remove(&id);
	}

	pub(crate) fn can_prev(&self, index: usize) -> bool {
		index > 0
	}

	pub(crate) fn can_next(&self, index: usize) -> bool {
		index + 1 < self.slides.len()
	}

	pub(crate) fn prev(&self, index: usize) -> Option<usize> {
		self.can_prev(index).then(|| index - 1)
	}

	pub(crate) fn next(&self, index: usize) -> Option<usize> {
		self.can_next(index).then(|| index + 1)
	}

	/// The index a keydown on the region moves to: `Some` for the arrows it
	/// handles (consume the key), whose move may be no move at an end.
	pub(crate) fn key(&self, input: &KeyInput, index: usize) -> Option<Option<usize>> {
		match input.key.as_str() {
			"ArrowLeft" => Some(self.prev(index)),
			"ArrowRight" => Some(self.next(index)),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn two_slides() -> Carousel {
		let mut c = Carousel::default();
		c.register(7);
		c.register(9);
		c
	}

	#[test]
	fn moves_stop_at_both_ends() {
		let c = two_slides();
		assert_eq!(c.next(0), Some(1));
		assert_eq!(c.next(1), None);
		assert_eq!(c.prev(0), None);
		assert!(!c.can_prev(0) && c.can_next(0));
	}

	#[test]
	fn arrows_are_consumed_even_at_an_end() {
		let c = two_slides();
		assert_eq!(c.key(&"ArrowRight".into(), 1), Some(None));
		assert_eq!(c.key(&"ArrowLeft".into(), 1), Some(Some(0)));
		assert_eq!(c.key(&"ArrowDown".into(), 0), None);
	}

	#[test]
	fn an_unmounted_slide_stops_counting() {
		let mut c = two_slides();
		c.unregister(9);
		assert!(!c.can_next(0));
	}
}
