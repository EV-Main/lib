use dioxus::prelude::*;

pub use crate::uikit::CalendarDate;
use crate::{
	cn,
	uikit::{self, Day, NEXT_MONTH, NO_CLOCK_MONTH, PREVIOUS_MONTH, WEEKDAYS},
	uikit_dioxus::{
		CALENDAR_CAPTION, CALENDAR_DAY_CELL, CALENDAR_DAY_EMPTY, CALENDAR_GRID, CALENDAR_NAV, CALENDAR_ROOT, CALENDAR_WEEK, CALENDAR_WEEKDAY, CALENDAR_WEEKDAY_ROW,
		primitives::use_controllable,
	},
};

const CHEVRON_LEFT: &str = "m15 18-6-6 6-6";
const CHEVRON_RIGHT: &str = "m9 18 6-6-6-6";

/// A dep-light single-month, single-date picker. Mirrors the landing
/// `Calendar`'s class names while replacing `react-day-picker` with hand-rolled
/// month-grid math.
///
/// Simplifications versus the source: one month only (no multi-month), a single
/// selected date (no range/multi), and none of the locale/dropdown/caption
/// features — see the package README.
#[component]
pub fn Calendar(
	/// The currently selected day, if any.
	selected: Option<CalendarDate>,
	/// Fired with the day a user activates.
	on_select: Option<EventHandler<CalendarDate>>,
	/// Controlled displayed month (any day in it is fine).
	month: Option<CalendarDate>,
	/// Uncontrolled initial displayed month; the host's current month by default.
	default_month: Option<CalendarDate>,
	/// Fired when the displayed month changes via the nav buttons.
	on_month_change: Option<EventHandler<CalendarDate>>,
	/// "Today", highlighted in the grid; the host's current date by default
	/// (read where the port has a clock).
	today: Option<CalendarDate>,
	/// Earliest selectable day (inclusive); earlier days render disabled.
	min: Option<CalendarDate>,
	/// Latest selectable day (inclusive); later days render disabled.
	max: Option<CalendarDate>,
	/// Disables the whole grid and the nav buttons; every day renders `data-disabled`.
	#[props(default)]
	disabled: bool,
	/// `aria-label` of the previous-month button; "Previous month" by default.
	previous_month_label: Option<String>,
	/// `aria-label` of the next-month button; "Next month" by default.
	next_month_label: Option<String>,
	#[props(default)] class: String,
) -> Element {
	// Both default to the host clock independently, as in the TS port: an
	// explicit `today` does not move the opening month.
	let host_today = crate::uikit_host::today();
	let today = today.or(host_today);
	let view = use_controllable(month, default_month.or(host_today).unwrap_or(NO_CLOCK_MONTH), on_month_change);
	let current = view.get();

	let go = move |delta: i32| {
		view.set(current.add_months(delta));
	};
	let grid = uikit::Grid {
		month: current,
		selected,
		today,
		min,
		max,
		disabled,
	};
	let nav_class = grid.nav_class();
	let previous_label = previous_month_label.unwrap_or_else(|| PREVIOUS_MONTH.to_owned());
	let next_label = next_month_label.unwrap_or_else(|| NEXT_MONTH.to_owned());

	let root = cn!(CALENDAR_ROOT, class);

	rsx! {
		div { class: root, "data-slot": "calendar", role: "application",
			div { class: CALENDAR_NAV,
				button {
					r#type: "button",
					class: nav_class.clone(),
					"aria-label": previous_label,
					disabled,
					onclick: move |_| go(-1),
					Chevron { d: CHEVRON_LEFT }
				}
				div { class: CALENDAR_CAPTION, "data-slot": "calendar-caption", {grid.caption()} }
				button {
					r#type: "button",
					class: nav_class.clone(),
					"aria-label": next_label,
					disabled,
					onclick: move |_| go(1),
					Chevron { d: CHEVRON_RIGHT }
				}
			}
			table { class: CALENDAR_GRID, role: "grid",
				thead {
					tr { class: CALENDAR_WEEKDAY_ROW,
						for wd in WEEKDAYS {
							th {
								class: CALENDAR_WEEKDAY,
								scope: "col",
								{wd}
							}
						}
					}
				}
				tbody {
					for week in grid.weeks() {
						tr { class: CALENDAR_WEEK,
							for day in week {
								DayCell { day, on_select }
							}
						}
					}
				}
			}
		}
	}
}

#[component]
fn DayCell(day: Option<Day>, on_select: Option<EventHandler<CalendarDate>>) -> Element {
	let Some(day) = day else {
		return rsx! {
			td { class: CALENDAR_DAY_EMPTY }
		};
	};
	rsx! {
		td {
			class: CALENDAR_DAY_CELL,
			role: "gridcell",
			"aria-selected": if day.selected { "true" } else { "false" },
			button {
				r#type: "button",
				class: day.class(),
				"data-slot": "calendar-day",
				"data-selected": if day.selected { "true" } else { "false" },
				"data-today": if day.today { "true" } else { "false" },
				// Absent rather than "false", so the unbounded grid renders as before.
				"data-disabled": if day.disabled { Some("true") } else { None },
				disabled: day.disabled,
				onclick: move |_| {
					if !day.disabled && let Some(h) = on_select {
						h.call(day.date);
					}
				},
				"{day.date.day}"
			}
		}
	}
}

#[component]
fn Chevron(d: &'static str) -> Element {
	rsx! {
		svg {
			class: "size-4",
			xmlns: "http://www.w3.org/2000/svg",
			width: "24",
			height: "24",
			view_box: "0 0 24 24",
			fill: "none",
			stroke: "currentColor",
			stroke_width: "2",
			stroke_linecap: "round",
			stroke_linejoin: "round",
			path { d }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit_dioxus::test_util::render;

	#[test]
	fn renders_month_grid_with_slots_and_weekdays() {
		fn app() -> Element {
			rsx! {
				Calendar { default_month: CalendarDate::new(2026, 6, 1) }
			}
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"calendar\""), "{html}");
		assert!(html.contains("June 2026"), "{html}");
		assert!(html.contains("Mo"));
		assert!(html.contains("role=\"gridcell\""), "{html}");
		// Every day of June is present.
		assert!(html.contains(">30<"), "{html}");
	}

	#[test]
	fn selected_and_today_get_their_classes() {
		fn app() -> Element {
			rsx! {
				Calendar {
					default_month: CalendarDate::new(2026, 6, 1),
					selected: CalendarDate::new(2026, 6, 10),
					today: CalendarDate::new(2026, 6, 15),
				}
			}
		}
		let html = render(app);
		assert!(html.contains("aria-selected=\"true\""), "{html}");
		assert!(html.contains("bg-primary"), "{html}");
		assert!(html.contains("bg-hover"), "{html}");
		assert!(html.contains("data-selected=\"true\""), "{html}");
	}

	#[test]
	fn unbounded_grid_has_no_disabled_days_and_english_nav_labels() {
		fn app() -> Element {
			rsx! {
				Calendar { default_month: CalendarDate::new(2026, 6, 1) }
			}
		}
		let html = render(app);
		assert!(!html.contains(" disabled=true"), "{html}");
		assert!(!html.contains("data-disabled"), "{html}");
		assert!(html.contains("aria-label=\"Previous month\""), "{html}");
		assert!(html.contains("aria-label=\"Next month\""), "{html}");
		assert!(html.contains("data-slot=\"calendar-caption\""), "{html}");
	}

	#[test]
	fn days_outside_min_max_render_disabled() {
		fn app() -> Element {
			rsx! {
				Calendar {
					default_month: CalendarDate::new(2026, 6, 1),
					min: CalendarDate::new(2026, 6, 10),
					max: CalendarDate::new(2026, 6, 20),
				}
			}
		}
		let html = render(app);
		// 9 days before `min` + 10 days after `max`.
		assert_eq!(html.matches("data-disabled=\"true\"").count(), 19, "{html}");
		assert_eq!(html.matches(" disabled=true").count(), 19, "{html}");
		let tenth = html.find(">10<").expect("day 10 rendered");
		let tag_start = html[..tenth].rfind("<button").expect("day 10 button");
		assert!(!html[tag_start..tenth].contains(" disabled=true"), "day at `min` stays enabled: {html}");
	}

	#[test]
	fn disabled_freezes_every_day_and_the_nav() {
		fn app() -> Element {
			rsx! {
				Calendar {
					default_month: CalendarDate::new(2026, 6, 1),
					disabled: true,
					min: CalendarDate::new(2026, 6, 10),
				}
			}
		}
		let html = render(app);
		// All 30 days of June, `min` notwithstanding.
		assert_eq!(html.matches("data-disabled=\"true\"").count(), 30, "{html}");
		// 30 days + the two nav buttons.
		assert_eq!(html.matches(" disabled=true").count(), 32, "{html}");
	}

	#[test]
	fn nav_labels_are_overridable() {
		fn app() -> Element {
			rsx! {
				Calendar {
					default_month: CalendarDate::new(2026, 6, 1),
					previous_month_label: "Предыдущий месяц",
					next_month_label: "Следующий месяц",
				}
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Предыдущий месяц\""), "{html}");
		assert!(html.contains("aria-label=\"Следующий месяц\""), "{html}");
		assert!(!html.contains("Previous month"), "{html}");
	}

	#[test]
	fn grid_always_renders_six_weeks() {
		fn short_month() -> Element {
			// 1 Feb 2027 is a Monday and the month has 28 days: exactly four
			// weeks before the grid was fixed at six.
			rsx! {
				Calendar { default_month: CalendarDate::new(2027, 2, 1) }
			}
		}
		fn long_month() -> Element {
			// 1 Aug 2026 is a Saturday and the month has 31 days: six weeks.
			rsx! {
				Calendar { default_month: CalendarDate::new(2026, 8, 1) }
			}
		}
		for (app, days) in [(short_month as fn() -> Element, 28usize), (long_month, 31)] {
			let html = render(app);
			assert_eq!(html.matches(CALENDAR_WEEK).count(), 6, "{html}");
			assert_eq!(html.matches("role=\"gridcell\"").count(), days, "{html}");
			assert_eq!(html.matches(CALENDAR_DAY_EMPTY).count(), 6 * 7 - days, "{html}");
		}
	}

	#[test]
	fn explicit_month_and_today_win_over_the_host_clock() {
		// A fixed month in the past and an explicit `today`: what the host clock
		// says cannot reach the markup, so this stays byte-stable forever.
		fn app() -> Element {
			rsx! {
				Calendar { default_month: CalendarDate::new(2026, 6, 1), today: CalendarDate::new(2026, 6, 10) }
			}
		}
		let html = render(app);
		assert!(html.contains("June 2026"), "{html}");
		assert_eq!(html.matches("data-today=\"true\"").count(), 1, "{html}");
	}

	#[test]
	fn uncontrolled_calendar_opens_on_the_host_month_and_marks_today() {
		fn app() -> Element {
			rsx! {
				Calendar {}
			}
		}
		// Sampled on both sides of the render: a month rollover between the two
		// reads must not turn into a flake.
		let before = crate::uikit_host::today().expect("native builds read SystemTime");
		let html = render(app);
		let after = crate::uikit_host::today().expect("native builds read SystemTime");
		let caption = |month| {
			uikit::Grid {
				month,
				selected: None,
				today: None,
				min: None,
				max: None,
				disabled: false,
			}
			.caption()
		};
		assert!(html.contains(&caption(before)) || html.contains(&caption(after)), "{html}");
		assert_eq!(html.matches("data-today=\"true\"").count(), 1, "{html}");
	}
}
