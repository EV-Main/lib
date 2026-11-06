/// What `FormControl` hands down to the control beneath it. Provided as a signal;
/// the kit's own controls consume it and stamp these onto themselves. Consume it
/// too if you wrap a bare element.
#[derive(Clone, Debug, PartialEq)]
pub struct FormControlContext {
	/// The id `FormLabel`'s `for` points at — put it on the control itself, or
	/// the label focuses nothing.
	pub id: String,
	/// The description id, plus the message id when invalid.
	pub described_by: String,
	/// Drives the control's own `aria-invalid:*` styling.
	pub invalid: bool,
}
