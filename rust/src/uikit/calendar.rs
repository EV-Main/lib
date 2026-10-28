use ev_lib_classes::{ButtonVariant, CALENDAR_DAY, CALENDAR_DAY_SELECTED, CALENDAR_DAY_TODAY, CALENDAR_NAV_BUTTON, Size};

use super::button_classes;
use crate::cn;

const MONTHS: [&str; 12] = [
	"January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December",
];
pub(crate) const WEEKDAYS: [&str; 7] = ["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"];
pub(crate) const PREVIOUS_MONTH: &str = "Previous month";
pub(crate) const NEXT_MONTH: &str = "Next month";
/// Always six weeks: a popover host must not jump in height when the month
/// changes, and six rows is the most any month needs with a Monday-first week
/// (31 days starting on a Sunday).
const GRID_WEEKS: usize = 6;
/// What an uncontrolled calendar opens on when the build has no clock to read
/// (see [`CalendarDate::today`]) and the caller passed no `default_month`.
pub(crate) const NO_CLOCK_MONTH: CalendarDate = CalendarDate { year: 1970, month: 1, day: 1 };

/// A calendar date as plain `(year, month 1-12, day 1-31)`. The kernel does its
/// own date math (no `chrono`/`jiff`): `wasm32`-safe and dependency-free.
///
/// Fields are declared in `year, month, day` order, so the derived ordering is
/// chronological.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CalendarDate {
	pub year: i32,
	pub month: u32,
	pub day: u32,
}

impl CalendarDate {
	pub fn new(year: i32, month: u32, day: u32) -> Self {
		Self { year, month, day }
	}

	fn is_leap(year: i32) -> bool {
		(year % 4 == 0 && year % 100 != 0) || year % 400 == 0
	}

	/// Days in `month` (1-12) of `year`, accounting for leap February.
	fn days_in_month(year: i32, month: u32) -> u32 {
		match month {
			1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
			4 | 6 | 9 | 11 => 30,
			2 if Self::is_leap(year) => 29,
			2 => 28,
			_ => unreachable!("month {month} is outside 1-12"),
		}
	}

	/// Weekday of the first day of the month, **Monday = 0 … Sunday = 6**, via
	/// Zeller's congruence.
	fn first_weekday_monday0(year: i32, month: u32) -> u32 {
		// Zeller treats Jan/Feb as months 13/14 of the previous year.
		let (m, y) = if month < 3 { (month + 12, year - 1) } else { (month, year) };
		let k = y.rem_euclid(100);
		let j = y.div_euclid(100);
		// h: 0 = Saturday, 1 = Sunday, 2 = Monday, …, 6 = Friday (q = 1, the first).
		let h = (1 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j).rem_euclid(7);
		((h + 5).rem_euclid(7)) as u32
	}

	/// Today on the host clock, or `None` where the build has no clock to read.
	///
	/// Web (`wasm32` + the `wasm` feature) reads the browser's local date, like
	/// the TS port's `new Date()`. Native reads `SystemTime` and takes the **UTC**
	/// date: the local zone needs tz data the dep-light kit does not carry, so
	/// near midnight a native host can be a day off its wall clock. Bare `wasm32`
	/// without `wasm` has no clock (`SystemTime::now` panics there) and yields
	/// `None`.
	///
	/// Server-rendered and hydrated: the server's UTC date and the browser's
	/// local one differ near midnight (and across a month boundary), so the
	/// opened month and `data-today` would not match on hydration — the same as
	/// the TS `new Date()`. An SSR page passes `default_month` and `today`
	/// explicitly, computed once.
	pub fn today() -> Option<Self> {
		#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
		{
			let now = web_sys::js_sys::Date::new_0();
			Some(Self::new(now.get_full_year() as i32, now.get_month() + 1, now.get_date()))
		}
		#[cfg(not(target_arch = "wasm32"))]
		{
			let secs = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
				Ok(after) => after.as_secs() as i64,
				Err(before) => -(before.duration().as_secs() as i64),
			};
			Some(Self::from_unix_days(secs.div_euclid(86_400)))
		}
		#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
		{
			None
		}
	}

	/// The civil date `days` after 1970-01-01 (Hinnant's `civil_from_days`).
	/// Only the `SystemTime` path needs it; the browser hands back a civil date.
	#[cfg(any(not(target_arch = "wasm32"), test))]
	fn from_unix_days(days: i64) -> Self {
		let z = days + 719_468;
		let era = z.div_euclid(146_097);
		let doe = z.rem_euclid(146_097);
		let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
		let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
		let mp = (5 * doy + 2) / 153;
		let day = (doy - (153 * mp + 2) / 5 + 1) as u32;
		let month = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
		let year = (yoe + era * 400 + i64::from(month <= 2)) as i32;
		Self { year, month, day }
	}

	/// Same day shifted by `delta` months, clamped to the target month's length
	/// (the prev/next nav).
	pub(crate) fn add_months(&self, delta: i32) -> Self {
		let zero = (self.year as i64) * 12 + (self.month as i64 - 1) + delta as i64;
		let year = zero.div_euclid(12) as i32;
		let month = (zero.rem_euclid(12) + 1) as u32;
		let day = self.day.min(Self::days_in_month(year, month));
		Self { year, month, day }
	}
}

/// One displayed month of a `Calendar`, with what marks its days. Monday-first.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Grid {
	/// Any day of the displayed month.
	pub month: CalendarDate,
	pub selected: Option<CalendarDate>,
	pub today: Option<CalendarDate>,
	pub min: Option<CalendarDate>,
	pub max: Option<CalendarDate>,
	pub disabled: bool,
}
impl Grid {
	/// The caption, e.g. "June 2026".
	pub(crate) fn caption(&self) -> String {
		format!("{} {}", MONTHS[(self.month.month - 1) as usize], self.month.year)
	}

	/// Six weeks of cells: leading blanks, the month's days, trailing blanks.
	pub(crate) fn weeks(&self) -> Vec<[Option<Day>; 7]> {
		let (year, month) = (self.month.year, self.month.month);
		let lead = CalendarDate::first_weekday_monday0(year, month) as usize;
		let total = CalendarDate::days_in_month(year, month) as usize;
		(0..GRID_WEEKS)
			.map(|week| {
				std::array::from_fn(|weekday| {
					let at = week * 7 + weekday;
					(lead..lead + total).contains(&at).then(|| self.day(CalendarDate::new(year, month, (at - lead + 1) as u32)))
				})
			})
			.collect()
	}

	/// The prev/next buttons' classes.
	pub(crate) fn nav_class(&self) -> String {
		button_classes(&ButtonVariant::Ghost, Size::Md, true, None, CALENDAR_NAV_BUTTON)
	}

	fn day(&self, date: CalendarDate) -> Day {
		Day {
			date,
			selected: self.selected == Some(date),
			today: self.today == Some(date),
			disabled: self.disabled || self.min.is_some_and(|lo| date < lo) || self.max.is_some_and(|hi| date > hi),
		}
	}
}

/// A day cell's date and marks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Day {
	pub date: CalendarDate,
	pub selected: bool,
	pub today: bool,
	pub disabled: bool,
}

impl Day {
	/// Selected wins over today: the selected fill already marks the day.
	pub(crate) fn class(&self) -> String {
		let day = button_classes(&ButtonVariant::Ghost, Size::Md, true, None, CALENDAR_DAY);
		match (self.selected, self.today) {
			(true, _) => cn!(day, CALENDAR_DAY_SELECTED),
			(false, true) => cn!(day, CALENDAR_DAY_TODAY),
			(false, false) => day,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn june(day: u32) -> CalendarDate {
		CalendarDate::new(2026, 6, day)
	}

	fn grid(month: CalendarDate) -> Grid {
		Grid {
			month,
			selected: None,
			today: None,
			min: None,
			max: None,
			disabled: false,
		}
	}

	fn days(grid: &Grid) -> Vec<Day> {
		grid.weeks().into_iter().flatten().flatten().collect()
	}

	#[test]
	fn days_in_month_handles_leap_february() {
		assert_eq!(CalendarDate::days_in_month(2024, 2), 29);
		assert_eq!(CalendarDate::days_in_month(2023, 2), 28);
		assert_eq!(CalendarDate::days_in_month(1900, 2), 28);
		assert_eq!(CalendarDate::days_in_month(2000, 2), 29);
		assert_eq!(CalendarDate::days_in_month(2026, 4), 30);
		assert_eq!(CalendarDate::days_in_month(2026, 1), 31);
	}

	#[test]
	fn first_weekday_is_monday_zero() {
		assert_eq!(CalendarDate::first_weekday_monday0(2026, 6), 0, "1 June 2026 is a Monday");
		assert_eq!(CalendarDate::first_weekday_monday0(2026, 1), 3, "1 Jan 2026 is a Thursday");
		assert_eq!(CalendarDate::first_weekday_monday0(2026, 2), 6, "1 Feb 2026 is a Sunday");
	}

	#[test]
	fn add_months_wraps_year_and_clamps_day() {
		assert_eq!(CalendarDate::new(2026, 12, 15).add_months(1), CalendarDate::new(2027, 1, 15));
		assert_eq!(CalendarDate::new(2026, 1, 10).add_months(-1), CalendarDate::new(2025, 12, 10));
		assert_eq!(CalendarDate::new(2026, 1, 31).add_months(1), CalendarDate::new(2026, 2, 28), "31 Jan + 1 month clamps to 28 Feb");
	}

	#[test]
	fn unix_days_map_to_civil_dates() {
		assert_eq!(CalendarDate::from_unix_days(0), CalendarDate::new(1970, 1, 1));
		assert_eq!(CalendarDate::from_unix_days(-1), CalendarDate::new(1969, 12, 31));
		// 2000 is a leap year divisible by 400: 29 Feb exists, 1 Mar follows it.
		assert_eq!(CalendarDate::from_unix_days(11_016), CalendarDate::new(2000, 2, 29));
		assert_eq!(CalendarDate::from_unix_days(11_017), CalendarDate::new(2000, 3, 1));
		assert_eq!(CalendarDate::from_unix_days(19_723), CalendarDate::new(2024, 1, 1));
		assert_eq!(CalendarDate::from_unix_days(20_638), CalendarDate::new(2026, 7, 4));
	}

	#[test]
	fn calendar_date_orders_chronologically() {
		assert!(june(10) < june(11));
		assert!(june(30) < CalendarDate::new(2026, 7, 1));
		assert!(CalendarDate::new(2025, 12, 31) < CalendarDate::new(2026, 1, 1));
	}

	#[test]
	fn grid_is_always_six_weeks_of_the_month_s_days() {
		// 1 Feb 2027 is a Monday and the month has 28 days: four full weeks.
		// 1 Aug 2026 is a Saturday and the month has 31 days: six weeks.
		for (month, count, first_column) in [(CalendarDate::new(2027, 2, 1), 28, 0), (CalendarDate::new(2026, 8, 1), 31, 5)] {
			let weeks = grid(month).weeks();
			assert_eq!(weeks.len(), 6);
			assert_eq!(days(&grid(month)).len(), count);
			assert_eq!(weeks[0].iter().position(Option::is_some), Some(first_column));
		}
	}

	#[test]
	fn days_outside_min_max_or_a_disabled_grid_are_disabled() {
		let bounded = Grid {
			min: Some(june(10)),
			max: Some(june(20)),
			..grid(june(1))
		};
		let disabled: Vec<u32> = days(&bounded).into_iter().filter(|d| d.disabled).map(|d| d.date.day).collect();
		assert_eq!(disabled, (1..10).chain(21..=30).collect::<Vec<_>>(), "the bounds themselves stay enabled");
		let frozen = Grid { disabled: true, ..bounded };
		assert!(days(&frozen).iter().all(|d| d.disabled));
	}

	#[test]
	fn selected_and_today_mark_their_day_only() {
		let marked = Grid {
			selected: Some(june(10)),
			today: Some(june(15)),
			..grid(june(1))
		};
		let days = days(&marked);
		assert_eq!(days.iter().filter(|d| d.selected).map(|d| d.date).collect::<Vec<_>>(), [june(10)]);
		assert_eq!(days.iter().filter(|d| d.today).map(|d| d.date).collect::<Vec<_>>(), [june(15)]);
		assert_eq!(marked.caption(), "June 2026");
	}
}
