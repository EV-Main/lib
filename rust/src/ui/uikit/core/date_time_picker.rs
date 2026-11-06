use super::{CalendarDate, calendar::NO_CLOCK_MONTH};

/// A wall-clock moment with no zone: what the operator sees in the field.
///
/// Fields are declared in `date, hour, minute` order, so the derived ordering
/// is chronological.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct LocalDateTime {
	pub date: CalendarDate,
	pub hour: u32,
	pub minute: u32,
}

impl LocalDateTime {
	pub fn new(date: CalendarDate, hour: u32, minute: u32) -> Self {
		Self { date, hour, minute }
	}

	/// Inclusive clamp at minute precision.
	fn clamp_to(self, min: Option<Self>, max: Option<Self>) -> Self {
		match (min, max) {
			(Some(lo), _) if self < lo => lo,
			(_, Some(hi)) if self > hi => hi,
			_ => self,
		}
	}

	/// The default trigger label: `YYYY-MM-DD HH:MM`, 24-hour.
	pub(crate) fn label(&self) -> String {
		format!("{:04}-{:02}-{:02} {:02}:{:02}", self.date.year, self.date.month, self.date.day, self.hour, self.minute)
	}

	/// The hidden form value: `YYYY-MM-DDTHH:MM` (no zone on this side).
	pub(crate) fn form_value(&self) -> String {
		format!("{:04}-{:02}-{:02}T{:02}:{:02}", self.date.year, self.date.month, self.date.day, self.hour, self.minute)
	}
}

/// The picker's own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DateTimePickerLabels {
	/// `aria-label` of the calendar's previous-month button; "Previous month".
	pub previous_month: Option<String>,
	/// `aria-label` of the calendar's next-month button; "Next month".
	pub next_month: Option<String>,
	/// `aria-label` of the hours input; "Hours".
	pub hours: Option<String>,
	/// `aria-label` of the minutes input; "Minutes".
	pub minutes: Option<String>,
	/// Text of the clear button; "Clear".
	pub clear: Option<String>,
	/// `aria-label` of the close button; "Close".
	pub close: Option<String>,
	/// `aria-label` of the popover dialog; "Choose date and time".
	pub dialog: Option<String>,
}
impl DateTimePickerLabels {
	pub(crate) fn hours(&self) -> String {
		self.hours.clone().unwrap_or_else(|| "Hours".to_owned())
	}

	pub(crate) fn minutes(&self) -> String {
		self.minutes.clone().unwrap_or_else(|| "Minutes".to_owned())
	}

	pub(crate) fn clear(&self) -> String {
		self.clear.clone().unwrap_or_else(|| "Clear".to_owned())
	}

	pub(crate) fn close(&self) -> String {
		self.close.clone().unwrap_or_else(|| "Close".to_owned())
	}

	pub(crate) fn dialog(&self) -> String {
		self.dialog.clone().unwrap_or_else(|| "Choose date and time".to_owned())
	}
}

/// A `DateTimePicker`'s value and bounds, as of one render. The parent owns the
/// value; every edit comes back as the next value to emit.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Picker {
	pub value: Option<LocalDateTime>,
	pub min: Option<LocalDateTime>,
	pub max: Option<LocalDateTime>,
	pub today: Option<CalendarDate>,
}
impl Picker {
	/// The month the grid opens on. `min` before `today`: a bound in the future
	/// would otherwise open on a fully disabled grid.
	pub(crate) fn opening_month(&self) -> CalendarDate {
		self.value.map(|v| v.date).or(self.min.map(|m| m.date)).or(self.today).unwrap_or(NO_CLOCK_MONTH)
	}

	/// A day picked on the grid keeps the time already set.
	pub(crate) fn select(&self, day: CalendarDate) -> LocalDateTime {
		let (hour, minute) = self.value.map_or((0, 0), |v| (v.hour, v.minute));
		self.clamped(LocalDateTime::new(day, hour, minute))
	}

	/// What the hours field's `raw` text sets, `None` when nothing parses.
	/// `month` is the grid's, for a time typed before any day is chosen.
	pub(crate) fn hours(&self, raw: &str, month: CalendarDate) -> Option<LocalDateTime> {
		parse_field(raw, 23).map(|hour| self.clamped(LocalDateTime { hour, ..self.base(month) }))
	}

	pub(crate) fn minutes(&self, raw: &str, month: CalendarDate) -> Option<LocalDateTime> {
		parse_field(raw, 59).map(|minute| self.clamped(LocalDateTime { minute, ..self.base(month) }))
	}

	/// The time fields' text, midnight while empty.
	pub(crate) fn time_fields(&self) -> (String, String) {
		self.value
			.map_or_else(|| ("00".to_owned(), "00".to_owned()), |v| (format!("{:02}", v.hour), format!("{:02}", v.minute)))
	}

	fn base(&self, month: CalendarDate) -> LocalDateTime {
		self.value
			.unwrap_or_else(|| LocalDateTime::new(landing_day(self.today, self.min.map(|m| m.date), self.max.map(|m| m.date), month), 0, 0))
	}

	fn clamped(&self, at: LocalDateTime) -> LocalDateTime {
		at.clamp_to(self.min, self.max)
	}
}

/// The day a time typed before any day is chosen lands on: `today`, else
/// `min`'s day, else the first of the displayed `month` — then pulled inside
/// `[min, max]`. Clamping the day rather than the whole value keeps the typed
/// hour when `min` is in the future (or `max` in the past).
fn landing_day(today: Option<CalendarDate>, min: Option<CalendarDate>, max: Option<CalendarDate>, month: CalendarDate) -> CalendarDate {
	let day = today.or(min).unwrap_or_else(|| CalendarDate::new(month.year, month.month, 1));
	let day = min.map_or(day, |m| day.max(m));
	max.map_or(day, |m| day.min(m))
}

/// Parses the digits an operator typed into an hours/minutes field. `None`
/// (nothing typed, or no digit at all) keeps the last value.
///
/// The LAST two digits win: the field is controlled, so what the browser hands
/// over is the old value plus the keystroke ("01" + "5" = "015"), and the
/// freshest digits are the ones the operator meant.
fn parse_field(raw: &str, max: u32) -> Option<u32> {
	let digits: Vec<u32> = raw.chars().filter_map(|c| c.to_digit(10)).collect();
	let tail = &digits[digits.len().saturating_sub(2)..];
	(!tail.is_empty()).then(|| tail.iter().fold(0, |n, d| n * 10 + d).min(max))
}

#[cfg(test)]
mod tests {
	use super::*;

	fn june(day: u32, hour: u32, minute: u32) -> LocalDateTime {
		LocalDateTime::new(CalendarDate::new(2026, 6, day), hour, minute)
	}

	fn picker(value: Option<LocalDateTime>) -> Picker {
		Picker {
			value,
			min: None,
			max: None,
			today: None,
		}
	}

	#[test]
	fn local_date_time_orders_and_clamps_at_minute_precision() {
		assert!(june(10, 9, 30) < june(10, 9, 31));
		assert!(june(10, 23, 59) < june(11, 0, 0));
		let lo = june(10, 9, 0);
		let hi = june(20, 18, 0);
		assert_eq!(june(10, 8, 59).clamp_to(Some(lo), Some(hi)), lo);
		assert_eq!(june(20, 18, 1).clamp_to(Some(lo), Some(hi)), hi);
		assert_eq!(june(15, 12, 0).clamp_to(Some(lo), Some(hi)), june(15, 12, 0));
		assert_eq!(june(1, 0, 0).clamp_to(None, None), june(1, 0, 0));
	}

	#[test]
	fn label_and_form_value_are_zero_padded() {
		assert_eq!(june(10, 9, 5).label(), "2026-06-10 09:05");
		assert_eq!(june(10, 9, 5).form_value(), "2026-06-10T09:05");
	}

	/// A time typed before a day is chosen keeps its hour when `min` is in the
	/// future: the day moves inside the bounds, the time is not clamped away.
	#[test]
	fn landing_day_stays_inside_the_bounds() {
		let d = |day| CalendarDate::new(2026, 6, day);
		let month = d(1);
		assert_eq!(landing_day(Some(d(15)), None, None, month), d(15));
		assert_eq!(landing_day(Some(d(15)), Some(d(22)), None, month), d(22), "min in the future");
		assert_eq!(landing_day(Some(d(15)), None, Some(d(8)), month), d(8), "max in the past");
		assert_eq!(landing_day(None, Some(d(22)), None, month), d(22));
		assert_eq!(landing_day(None, None, None, d(3)), d(1));
		let future = Picker {
			min: Some(LocalDateTime::new(d(22), 0, 0)),
			today: Some(d(15)),
			..picker(None)
		};
		assert_eq!(future.hours("9", month), Some(LocalDateTime::new(d(22), 9, 0)), "the hour survives a midnight `min`");
	}

	#[test]
	fn parse_field_clamps_and_keeps_last_on_empty() {
		assert_eq!(parse_field("7", 23), Some(7));
		assert_eq!(parse_field("09", 23), Some(9));
		assert_eq!(parse_field("31", 23), Some(23));
		assert_eq!(parse_field("75", 59), Some(59));
		// Sequential typing over a controlled field: "0" + "1", then "01" + "5".
		assert_eq!(parse_field("01", 23), Some(1));
		assert_eq!(parse_field("015", 23), Some(15));
		assert_eq!(parse_field("123", 59), Some(23));
		assert_eq!(parse_field("1a5", 59), Some(15));
		assert_eq!(parse_field("", 23), None);
		assert_eq!(parse_field("ab", 23), None);
	}

	#[test]
	fn a_picked_day_keeps_the_time_and_the_bounds() {
		assert_eq!(picker(Some(june(10, 9, 5))).select(CalendarDate::new(2026, 6, 12)), june(12, 9, 5));
		assert_eq!(picker(None).select(CalendarDate::new(2026, 6, 12)), june(12, 0, 0));
		let bounded = Picker {
			min: Some(june(12, 10, 0)),
			..picker(Some(june(10, 9, 5)))
		};
		assert_eq!(bounded.select(CalendarDate::new(2026, 6, 12)), june(12, 10, 0));
	}

	#[test]
	fn the_grid_opens_on_the_value_then_the_lower_bound_then_today() {
		let aug = LocalDateTime::new(CalendarDate::new(2026, 8, 1), 0, 0);
		let today = Some(CalendarDate::new(2026, 6, 15));
		assert_eq!(
			Picker {
				today,
				..picker(Some(june(3, 0, 0)))
			}
			.opening_month(),
			june(3, 0, 0).date
		);
		assert_eq!(
			Picker {
				today,
				min: Some(aug),
				..picker(None)
			}
			.opening_month(),
			aug.date
		);
		assert_eq!(Picker { today, ..picker(None) }.opening_month(), CalendarDate::new(2026, 6, 15));
		assert_eq!(picker(None).opening_month(), NO_CLOCK_MONTH);
	}
}
