use leptos::prelude::*;

pub use crate::uikit::CalendarDate;
use crate::{
	cn,
	uikit::{self, Day, NEXT_MONTH, NO_CLOCK_MONTH, PREVIOUS_MONTH, WEEKDAYS},
	uikit_leptos::{
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
	#[prop(optional, into)]
	selected: MaybeProp<CalendarDate>,
	/// Fired with the day a user activates.
	#[prop(optional)]
	on_select: Option<Callback<CalendarDate>>,
	/// Controlled displayed month (any day in it is fine).
	#[prop(optional, into)]
	month: MaybeProp<CalendarDate>,
	/// Uncontrolled initial displayed month; the host's current month by default.
	#[prop(optional, into)]
	default_month: Option<CalendarDate>,
	/// Fired when the displayed month changes via the nav buttons.
	#[prop(optional)]
	on_month_change: Option<Callback<CalendarDate>>,
	/// "Today", highlighted in the grid; the host's current date by default
	/// ([`CalendarDate::today`]).
	#[prop(optional, into)]
	today: MaybeProp<CalendarDate>,
	/// Earliest selectable day (inclusive); earlier days render disabled.
	#[prop(optional, into)]
	min: MaybeProp<CalendarDate>,
	/// Latest selectable day (inclusive); later days render disabled.
	#[prop(optional, into)]
	max: MaybeProp<CalendarDate>,
	/// Disables the whole grid and the nav buttons; every day renders `data-disabled`.
	#[prop(optional, into)]
	disabled: Signal<bool>,
	/// `aria-label` of the previous-month button; "Previous month" by default.
	#[prop(optional, into)]
	previous_month_label: MaybeProp<String>,
	/// `aria-label` of the next-month button; "Next month" by default.
	#[prop(optional, into)]
	next_month_label: MaybeProp<String>,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	// Both default to the host clock independently, as in the TS port: an
	// explicit `today` does not move the opening month.
	let host_today = CalendarDate::today();
	let view = use_controllable(month, default_month.or(host_today).unwrap_or(NO_CLOCK_MONTH), on_month_change);
	let grid = Memo::new(move |_| uikit::Grid {
		month: view.get(),
		selected: selected.get(),
		today: today.get().or(host_today),
		min: min.get(),
		max: max.get(),
		disabled: disabled.get(),
	});
	let go = move |delta: i32| view.set(view.get_untracked().add_months(delta));
	let nav_class = grid.with_untracked(uikit::Grid::nav_class);
	let previous_label = move || previous_month_label.get().unwrap_or_else(|| PREVIOUS_MONTH.to_owned());
	let next_label = move || next_month_label.get().unwrap_or_else(|| NEXT_MONTH.to_owned());
	let root = cn!(CALENDAR_ROOT, class);
	view! {
		<div class=root data-slot="calendar" role="application">
			<div class=CALENDAR_NAV>
				<button
					type="button"
					class=nav_class.clone()
					aria-label=previous_label
					disabled=disabled
					on:click=move |_| go(-1)
				>
					<Chevron d=CHEVRON_LEFT />
				</button>
				<div class=CALENDAR_CAPTION data-slot="calendar-caption">
					{move || grid.with(uikit::Grid::caption)}
				</div>
				<button
					type="button"
					class=nav_class
					aria-label=next_label
					disabled=disabled
					on:click=move |_| go(1)
				>
					<Chevron d=CHEVRON_RIGHT />
				</button>
			</div>
			<table class=CALENDAR_GRID role="grid">
				<thead>
					<tr class=CALENDAR_WEEKDAY_ROW>
						{WEEKDAYS
							.map(|wd| {
								view! {
									<th class=CALENDAR_WEEKDAY scope="col">
										{wd}
									</th>
								}
							})}
					</tr>
				</thead>
				<tbody>
					{move || {
						grid
							.with(uikit::Grid::weeks)
							.into_iter()
							.map(|week| {
								view! {
									<tr class=CALENDAR_WEEK>
										{week.map(|day| view! { <DayCell day=day on_select=on_select /> })}
									</tr>
								}
							})
							.collect_view()
					}}
				</tbody>
			</table>
		</div>
	}
}

#[component]
fn DayCell(day: Option<Day>, on_select: Option<Callback<CalendarDate>>) -> impl IntoView {
	let Some(day) = day else {
		return view! { <td class=CALENDAR_DAY_EMPTY></td> }.into_any();
	};
	let flag = |b: bool| if b { "true" } else { "false" };
	view! {
		<td class=CALENDAR_DAY_CELL role="gridcell" aria-selected=flag(day.selected)>
			<button
				type="button"
				class=day.class()
				data-slot="calendar-day"
				data-selected=flag(day.selected)
				data-today=flag(day.today)
				// Absent rather than "false", so the unbounded grid renders as before.
				data-disabled=day.disabled.then_some("true")
				disabled=day.disabled
				on:click=move |_| {
					if !day.disabled && let Some(h) = on_select {
						h.run(day.date);
					}
				}
			>
				{day.date.day}
			</button>
		</td>
	}
	.into_any()
}

#[component]
fn Chevron(d: &'static str) -> impl IntoView {
	view! {
		<svg
			class="size-4"
			xmlns="http://www.w3.org/2000/svg"
			width="24"
			height="24"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
		>
			<path d=d></path>
		</svg>
	}
}
