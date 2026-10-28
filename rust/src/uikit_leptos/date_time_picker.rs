use leptos::{prelude::*, web_sys::HtmlInputElement};

pub use crate::uikit::{DateTimePickerLabels, LocalDateTime};
use crate::{
	cn,
	uikit::{self, CalendarDate},
	uikit_leptos::{
		ButtonVariant, DATE_TIME_PICKER_CLEAR, DATE_TIME_PICKER_CONTENT, DATE_TIME_PICKER_TIME, DATE_TIME_PICKER_TIME_INPUT, DATE_TIME_PICKER_TIME_SEPARATOR, DATE_TIME_PICKER_TRIGGER,
		INPUT_BASE, POPOVER_CONTENT, Size,
		button::button_classes,
		calendar::{Calendar, host_today},
		primitives::use_controllable,
	},
};

// lucide `calendar`, inlined per the kit's no-icon-dep convention.
const CALENDAR_ICON: [&str; 4] = ["M8 2v4", "M16 2v4", "M3 10h18", "M5 4h14a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2z"];
// lucide `x`.
const CLOSE_ICON: &str = "M18 6 6 18M6 6l12 12";

/// A date + time field over the kit's own bricks: an outline trigger opening a
/// popover with the [`Calendar`] and two numeric hours/minutes inputs — no
/// native `datetime-local`, so the browser's locale popup never appears.
///
/// The parent owns the value (like `Calendar`'s `selected`); every edit reports
/// through `on_change`. `today` defaults to the host date
/// (the host clock), like the TS port's `new Date()`; a time typed
/// before a day is chosen lands on it. Only where the build has no clock does
/// that fall back to `min`'s day, else the first of the displayed month. The
/// overlay is the inline Rust popover (see the README "Limitations").
#[component]
pub fn DateTimePicker(
	/// The current value; unset = empty.
	#[prop(optional, into)]
	value: MaybeProp<LocalDateTime>,
	/// Fired with the next value (`None` when cleared).
	#[prop(optional)]
	on_change: Option<Callback<Option<LocalDateTime>>>,
	/// Earliest allowed moment (inclusive, minute precision).
	#[prop(optional, into)]
	min: MaybeProp<LocalDateTime>,
	/// Latest allowed moment (inclusive, minute precision).
	#[prop(optional, into)]
	max: MaybeProp<LocalDateTime>,
	/// Overrides the trigger label; default `YYYY-MM-DD HH:MM`.
	#[prop(optional)]
	format: Option<Callback<LocalDateTime, String>>,
	/// Shown on the trigger while empty.
	#[prop(optional, into)]
	placeholder: String,
	#[prop(optional)] labels: DateTimePickerLabels,
	#[prop(optional, into)] disabled: Signal<bool>,
	/// Renders a hidden `<input name>` carrying `YYYY-MM-DDTHH:MM` (or "") for plain forms.
	#[prop(optional, into)]
	name: Option<String>,
	/// "Today" for the calendar highlight and as the day a typed time lands on
	/// while empty.
	#[prop(optional, into)]
	today: MaybeProp<CalendarDate>,
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	/// Fused onto the trigger.
	#[prop(optional, into)]
	class: String,
	/// Put on the trigger, so a `FieldLabel`'s `for` reaches it.
	#[prop(optional, into)]
	id: Option<String>,
) -> impl IntoView {
	let host_today = host_today();
	let open = use_controllable(open, default_open, on_open_change);
	let picker = Memo::new(move |_| uikit::Picker {
		value: value.get(),
		min: min.get(),
		max: max.get(),
		today: today.get().or(host_today),
	});
	let month = RwSignal::new(picker.with_untracked(uikit::Picker::opening_month));

	let emit = move |next: Option<LocalDateTime>| {
		if let Some(h) = on_change {
			h.run(next);
		}
	};
	let on_select = Callback::new(move |day: CalendarDate| emit(Some(picker.get_untracked().select(day))));
	// The time fields are controlled, so every keystroke must leave the field on
	// the canonical "HH"/"MM" (as React does). When the emitted value did not
	// change ("9" typed over "09") or nothing parsed (the field backspaced to
	// empty), no signal moves and `prop:value` would leave the raw keystrokes:
	// write the field back by hand.
	let time_input = move |hours: bool| {
		move |e: leptos::ev::Event| {
			let field = event_target::<HtmlInputElement>(&e);
			let p = picker.get_untracked();
			let next = if hours {
				p.hours(&field.value(), month.get_untracked())
			} else {
				p.minutes(&field.value(), month.get_untracked())
			};
			if let Some(next) = next {
				emit(Some(next));
			}
			let (h, m) = picker.get_untracked().time_fields();
			field.set_value(if hours { &h } else { &m });
		}
	};
	// A focused field is replaced, not appended to: with the whole value
	// selected, "1" then "5" reads as 15.
	let select_all = move |e: leptos::ev::FocusEvent| event_target::<HtmlInputElement>(&e).select();
	let toggle = move |_| {
		// Reopening lands on the value's month, not wherever the grid was left.
		if !open.get_untracked()
			&& let Some(v) = value.get_untracked()
		{
			month.set(v.date);
		}
		open.set(!open.get_untracked());
	};

	let label = move || match (value.get(), format) {
		(Some(v), Some(f)) => f.run(v),
		(Some(v), None) => v.label(),
		(None, _) => placeholder.clone(),
	};
	let trigger_class = button_classes(&ButtonVariant::Outline, Size::Md, false, None, &cn!(DATE_TIME_PICKER_TRIGGER, class));
	let content_class = cn!(POPOVER_CONTENT, cn!("absolute top-full left-0 mt-1", DATE_TIME_PICKER_CONTENT));
	let time_input_class = cn!(INPUT_BASE, DATE_TIME_PICKER_TIME_INPUT);
	let clear_class = button_classes(&ButtonVariant::Ghost, Size::Sm, false, None, DATE_TIME_PICKER_CLEAR);
	let close_class = button_classes(&ButtonVariant::Ghost, Size::Sm, true, None, "");

	let content = move || {
		open.get().then(|| {
			let (hours, minutes) = picker.with_untracked(uikit::Picker::time_fields);
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| open.set(false)></div>
				<div
					class=content_class.clone()
					role="dialog"
					aria-label=labels.dialog()
					data-slot="date-time-picker-content"
					data-state="open"
					data-side="bottom"
					data-align="start"
				>
					<Calendar
						selected=Signal::derive(move || value.get().map(|v| v.date))
						on_select=on_select
						month=Signal::derive(move || Some(month.get()))
						on_month_change=Callback::new(move |m| month.set(m))
						today=Signal::derive(move || picker.get().today)
						min=Signal::derive(move || min.get().map(|m| m.date))
						max=Signal::derive(move || max.get().map(|m| m.date))
						disabled=disabled
						previous_month_label=labels.previous_month.clone()
						next_month_label=labels.next_month.clone()
					/>
					<div class=DATE_TIME_PICKER_TIME data-slot="date-time-picker-time">
						<input
							type="text"
							inputmode="numeric"
							class=time_input_class.clone()
							data-slot="date-time-picker-hours"
							aria-label=labels.hours()
							value=hours
							prop:value=move || picker.get().time_fields().0
							disabled=disabled
							on:input=time_input(true)
							on:focus=select_all
						/>
						<span class=DATE_TIME_PICKER_TIME_SEPARATOR aria-hidden="true">
							":"
						</span>
						<input
							type="text"
							inputmode="numeric"
							class=time_input_class.clone()
							data-slot="date-time-picker-minutes"
							aria-label=labels.minutes()
							value=minutes
							prop:value=move || picker.get().time_fields().1
							disabled=disabled
							on:input=time_input(false)
							on:focus=select_all
						/>
						<button
							type="button"
							class=clear_class.clone()
							data-slot="date-time-picker-clear"
							disabled=disabled
							on:click=move |_| {
								emit(None);
								open.set(false);
							}
						>
							{labels.clear()}
						</button>
						<button
							type="button"
							class=close_class.clone()
							data-slot="date-time-picker-close"
							aria-label=labels.close()
							disabled=disabled
							on:click=move |_| open.set(false)
						>
							<CloseIcon />
						</button>
					</div>
				</div>
			}
			.into_any()
		})
	};

	view! {
		<div
			class="relative"
			data-slot="date-time-picker"
			on:keydown=move |e| {
				if e.key() == "Escape" {
					open.set(false);
				}
			}
		>
			<button
				type="button"
				id=id
				class=trigger_class
				data-slot="date-time-picker-trigger"
				data-placeholder=move || value.get().is_none().then_some("true")
				aria-haspopup="dialog"
				aria-expanded=move || if open.get() { "true" } else { "false" }
				disabled=disabled
				on:click=toggle
			>
				<CalendarIcon />
				<span>{label}</span>
			</button>
			{name
				.map(|name| {
					view! {
						<input
							type="hidden"
							name=name
							value=move || value.get().map(|v| v.form_value()).unwrap_or_default()
						/>
					}
				})}
			{content}
		</div>
	}
	.into_any()
}

#[component]
fn CalendarIcon() -> impl IntoView {
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
			aria-hidden="true"
		>
			{CALENDAR_ICON.map(|d| view! { <path d=d></path> })}
		</svg>
	}
}

#[component]
fn CloseIcon() -> impl IntoView {
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
			aria-hidden="true"
		>
			<path d=CLOSE_ICON></path>
		</svg>
	}
}
