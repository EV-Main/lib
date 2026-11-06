/// The `PopoverForm`'s own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PopoverFormLabels {
	/// The submit button; "Apply".
	pub submit: Option<String>,
	/// The reset button; "Reset".
	pub reset: Option<String>,
}
