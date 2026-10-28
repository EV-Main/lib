/// Whether a drawer's panel is on screen. It outlives `open`: closing flips it
/// to `data-state="closed"`, and it unmounts once its exit transition ends.
/// Re-opening mid-exit flips it back, and the same transition carries it home.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Presence {
	Absent,
	Open,
	Closing,
}
impl Presence {
	pub(crate) fn new(open: bool) -> Self {
		if open { Self::Open } else { Self::Absent }
	}

	/// `open` changed.
	pub(crate) fn follow(self, open: bool) -> Self {
		match (open, self) {
			(true, _) => Self::Open,
			(false, Self::Open) => Self::Closing,
			(false, other) => other,
		}
	}

	/// The panel's own exit `transform` finished.
	pub(crate) fn exit_ended(self) -> Self {
		match self {
			Self::Closing => Self::Absent,
			other => other,
		}
	}

	/// The panel's `data-state`; `None` while it is not rendered.
	pub(crate) fn state(self) -> Option<&'static str> {
		match self {
			Self::Absent => None,
			Self::Open => Some("open"),
			Self::Closing => Some("closed"),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn the_panel_outlives_open_until_its_exit_ends() {
		let closing = Presence::new(true).follow(false);
		assert_eq!(closing.state(), Some("closed"));
		assert_eq!(closing.follow(false), closing, "a re-render while closing keeps exiting");
		assert_eq!(closing.exit_ended().state(), None);
		assert_eq!(closing.follow(true).state(), Some("open"), "re-opening mid-exit carries it home");
		assert_eq!(Presence::new(true).exit_ended(), Presence::Open, "an open panel's transitions are not its exit");
		assert_eq!(Presence::new(false).follow(false).state(), None);
	}
}
