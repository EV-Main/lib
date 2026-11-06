/// The `ConfirmDialog`'s own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ConfirmDialogLabels {
	/// The cancel button; "Cancel".
	pub cancel: Option<String>,
	/// Shown when the work fails with an empty `error`; "Something went wrong".
	pub error: Option<String>,
}

/// A `ConfirmDialog`'s hold on the parent's work, which it reports through
/// `pending` and `error` (see the port's `ConfirmDialog`).
///
/// `awaiting` is set by a confirm until its work settles, so the dialog closes
/// on its own only for a confirm it started, and a press before the parent's
/// `pending` arrives cannot start the work twice.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct ConfirmFlow {
	awaiting: bool,
	show_error: bool,
}
impl ConfirmFlow {
	pub(crate) fn shows_error(&self) -> bool {
		self.show_error
	}

	/// The dialog (re)opened, which a controlled parent can do without
	/// [`ConfirmFlow::change`]: it starts without the last error.
	pub(crate) fn opened(&mut self) {
		self.show_error = false;
	}

	/// Whether an open-state change from cancel, Escape or the scrim goes
	/// through; while work runs, none does.
	pub(crate) fn change(&mut self, next: bool, pending: bool) -> bool {
		if pending || self.awaiting {
			return false;
		}
		if next {
			self.show_error = false;
		}
		true
	}

	/// Whether a press of confirm starts the work.
	pub(crate) fn confirm(&mut self, pending: bool) -> bool {
		if pending || self.awaiting {
			return false;
		}
		self.show_error = false;
		self.awaiting = true;
		true
	}

	/// Reads the parent's `pending` and whether it set `error`; `true` when the
	/// work this dialog started has succeeded and the dialog closes.
	pub(crate) fn settle(&mut self, pending: bool, failed: bool) -> bool {
		if !self.awaiting || pending {
			return false;
		}
		self.awaiting = false;
		self.show_error = failed;
		!failed
	}
}
