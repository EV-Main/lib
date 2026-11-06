pub const POPOVER_FORM_CONTENT: &str = "w-70 p-2";

pub const POPOVER_FORM_FORM: &str = "flex flex-col gap-2.5 px-0 py-1.5";

pub const POPOVER_FORM_TITLE: &str = "px-0 text-h3 font-medium text-ink";

pub const POPOVER_FORM_FIELDS: &str = "flex flex-col gap-2.5";

pub const POPOVER_FORM_ACTIONS: &str = "flex justify-end gap-1.5";

/// The `PopoverForm`'s own strings. English by default; a caller translates by overriding fields.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(serde::Serialize), serde(rename_all = "camelCase"))]
pub struct PopoverFormLabels {
	/// The submit button.
	pub submit: String,
	/// The reset button.
	pub reset: String,
}
impl Default for PopoverFormLabels {
	fn default() -> Self {
		Self {
			submit: "Apply".to_owned(),
			reset: "Reset".to_owned(),
		}
	}
}
