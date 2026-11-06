/// Fused on top of the outline `md` button: the field-shaped trigger, edged and
/// filled like an `Input` rather than the raised button face.
pub const DATE_TIME_PICKER_TRIGGER: &str = "border-input bg-transparent shadow-xs w-full justify-start gap-2 text-left font-normal data-[placeholder]:text-ink-soft [&>svg]:text-ink-soft";

/// Fused on top of the popover content: the calendar brings its own padding.
pub const DATE_TIME_PICKER_CONTENT: &str = "w-auto p-0";

pub const DATE_TIME_PICKER_TIME: &str = "flex items-center gap-2 border-t border-border px-3 py-2";

/// Fused on top of `INPUT_BASE` for the hours / minutes fields.
pub const DATE_TIME_PICKER_TIME_INPUT: &str = "w-14 text-center tabular-nums";

pub const DATE_TIME_PICKER_TIME_SEPARATOR: &str = "text-ink-soft select-none";

/// Fused on top of the ghost `sm` button.
pub const DATE_TIME_PICKER_CLEAR: &str = "ml-auto";

/// The `DateTimePicker`'s own strings. English by default; a caller translates by overriding fields.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(serde::Serialize), serde(rename_all = "camelCase"))]
pub struct DateTimePickerLabels {
	/// `aria-label` of the calendar's previous-month button.
	pub previous_month: String,
	/// `aria-label` of the calendar's next-month button.
	pub next_month: String,
	/// `aria-label` of the hours input.
	pub hours: String,
	/// `aria-label` of the minutes input.
	pub minutes: String,
	/// Text of the clear button.
	pub clear: String,
	/// `aria-label` of the close button.
	pub close: String,
	/// `aria-label` of the popover dialog.
	pub dialog: String,
}
impl Default for DateTimePickerLabels {
	fn default() -> Self {
		Self {
			previous_month: "Previous month".to_owned(),
			next_month: "Next month".to_owned(),
			hours: "Hours".to_owned(),
			minutes: "Minutes".to_owned(),
			clear: "Clear".to_owned(),
			close: "Close".to_owned(),
			dialog: "Choose date and time".to_owned(),
		}
	}
}
