//! Host reads both ports make and the core may not: the clock. Neither framework
//! nor DOM, so one copy serves both.

use crate::uikit::CalendarDate;

/// Today on the host clock, or `None` where the build has no clock to read.
///
/// Web (`wasm32` + the `wasm` feature) reads the browser's local date, like the
/// TS port's `new Date()`. Native reads `SystemTime` and takes the **UTC** date:
/// the local zone needs tz data the dep-light kit does not carry, so near
/// midnight a native host can be a day off its wall clock. Bare `wasm32` without
/// `wasm` has no clock (`SystemTime::now` panics there).
///
/// Server-rendered and hydrated, the server's UTC date and the browser's local
/// one differ near midnight, so the opened month and `data-today` would not
/// match on hydration — the same as the TS `new Date()`. An SSR page passes
/// `default_month` and `today` explicitly, computed once.
pub(crate) fn today() -> Option<CalendarDate> {
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	{
		let now = web_sys::js_sys::Date::new_0();
		Some(CalendarDate::new(now.get_full_year() as i32, now.get_month() + 1, now.get_date()))
	}
	#[cfg(not(target_arch = "wasm32"))]
	{
		let secs = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
			Ok(after) => after.as_secs() as i64,
			Err(before) => -(before.duration().as_secs() as i64),
		};
		Some(CalendarDate::from_unix_days(secs.div_euclid(86_400)))
	}
	#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
	{
		None
	}
}

/// Milliseconds on some monotonic-enough clock, for the type-ahead window;
/// `None` on bare `wasm32` without `wasm`, where `SystemTime::now` panics.
pub(crate) fn now_ms() -> Option<f64> {
	#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
	{
		Some(web_sys::js_sys::Date::now())
	}
	#[cfg(not(target_arch = "wasm32"))]
	{
		Some(
			std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.expect("the host clock is past 1970")
				.as_secs_f64()
				* 1000.0,
		)
	}
	#[cfg(all(target_arch = "wasm32", not(feature = "wasm")))]
	{
		None
	}
}
