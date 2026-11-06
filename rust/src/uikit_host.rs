//! Host access both ports need and the core may not have: the clock, the
//! clipboard, the platform and a document key listener. Framework-free, so one
//! copy serves both.

use crate::uikit::CalendarDate;

/// How long a copy's check mark and "Copied" announcement stay, in ms.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) const COPIED_MS: i32 = 2000;

/// Writes `text` to the clipboard, then calls `done` with whether it got
/// there. The mirror of the TS `writeClipboard` behind `KeyValue`.
///
/// The async clipboard API needs a secure context and a user gesture, so call
/// it from a press. An insecure origin, an old webview or a denied permission
/// falls back to a selection and `execCommand("copy")`.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub fn write_clipboard(text: &str, done: impl FnOnce(bool) + 'static) {
	use std::{cell::RefCell, rc::Rc};

	use web_sys::{
		js_sys::{Function, Reflect},
		wasm_bindgen::{JsCast, JsValue, closure::Closure},
	};

	let Some(window) = web_sys::window() else { return done(false) };
	let navigator = window.navigator();
	// `navigator.clipboard` is missing outside a secure context and in old
	// webviews; the binding does not return an `Option`, so ask first.
	let has_api = window.is_secure_context() && Reflect::get(&navigator, &JsValue::from_str("clipboard")).is_ok_and(|c| !c.is_undefined());
	if !has_api {
		return done(copy_by_selection(text));
	}
	let promise = navigator.clipboard().write_text(text);
	// One of the two handlers runs; both own the callback, so it sits in a
	// shared slot that the first to run empties.
	let slot = Rc::new(RefCell::new(Some(done)));
	let resolved = {
		let slot = Rc::clone(&slot);
		Closure::once_into_js(move |_: JsValue| {
			if let Some(done) = slot.borrow_mut().take() {
				done(true);
			}
		})
	};
	let text = text.to_owned();
	let rejected = {
		let slot = Rc::clone(&slot);
		Closure::once_into_js(move |_: JsValue| {
			if let Some(done) = slot.borrow_mut().take() {
				done(copy_by_selection(&text));
			}
		})
	};
	let then = Reflect::get(&promise, &JsValue::from_str("then")).ok().and_then(|f| f.dyn_into::<Function>().ok());
	let attached = then.is_some_and(|then| then.call2(&promise, &resolved, &rejected).is_ok());
	if !attached && let Some(done) = slot.borrow_mut().take() {
		done(false);
	}
}

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

/// The fallback for a browser without the async clipboard API.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
fn copy_by_selection(text: &str) -> bool {
	use web_sys::wasm_bindgen::JsCast;

	let Some(document) = web_sys::window().and_then(|w| w.document()) else { return false };
	// A modal pulls focus back into itself, which would drop a selection made
	// in `<body>`, so the helper lives inside the dialog the user is in.
	let dialog = document.active_element().and_then(|el| el.closest("[role=\"dialog\"]").ok().flatten());
	let Some(host) = dialog.or_else(|| document.body().map(Into::into)) else { return false };
	let Ok(area) = document.create_element("textarea") else { return false };
	let Ok(area) = area.dyn_into::<web_sys::HtmlTextAreaElement>() else { return false };
	area.set_value(text);
	// Attribute writes on a fresh element cannot fail in practice; a failure
	// only shows the helper for the instant it lives.
	let _ = area.set_attribute("readonly", "");
	let _ = area.set_attribute("style", "position: fixed; opacity: 0");
	if host.append_child(&area).is_err() {
		return false;
	}
	area.select();
	let copied = document.dyn_ref::<web_sys::HtmlDocument>().and_then(|d| d.exec_command("copy").ok()).unwrap_or(false);
	area.remove();
	copied
}

/// The `HeaderSearch` hint for the browser's platform: "⌘K" on Apple ones.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) fn platform_shortcut() -> &'static str {
	// A browser that refuses to name its platform gets the Ctrl hint, which is
	// what the TS port shows for an empty platform too.
	let platform = web_sys::window().and_then(|w| w.navigator().platform().ok()).unwrap_or_default();
	crate::uikit::platform_shortcut(&platform)
}

/// The ⌘K / Ctrl+K listener on `document` behind `use_command_palette_shortcut`,
/// removed when dropped.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) struct ShortcutListener {
	document: web_sys::Document,
	callback: web_sys::wasm_bindgen::closure::Closure<dyn FnMut(web_sys::KeyboardEvent)>,
}
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl ShortcutListener {
	/// `None` when there is no `document` or it refuses the listener; the
	/// shortcut then just does not exist, which is all a caller could do.
	pub(crate) fn add(mut on_fire: impl FnMut() + 'static) -> Option<Self> {
		use web_sys::wasm_bindgen::{JsCast, closure::Closure};

		use crate::uikit::{ShortcutHit, ShortcutKey, shortcut_hit};

		let document = web_sys::window()?.document()?;
		let callback = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(move |e: web_sys::KeyboardEvent| {
			let key = ShortcutKey {
				code: &e.code(),
				ctrl: e.ctrl_key(),
				meta: e.meta_key(),
				alt: e.alt_key(),
				shift: e.shift_key(),
				repeat: e.repeat(),
			};
			match shortcut_hit(&key) {
				ShortcutHit::Miss => {}
				ShortcutHit::Held => e.prevent_default(),
				ShortcutHit::Fire => {
					e.prevent_default();
					on_fire();
				}
			}
		});
		document.add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref()).ok()?;
		Some(Self { document, callback })
	}
}
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
impl Drop for ShortcutListener {
	fn drop(&mut self) {
		use web_sys::wasm_bindgen::JsCast;

		// A listener the document no longer holds is already gone, so a failed
		// removal leaves nothing to clean up.
		let _ = self.document.remove_event_listener_with_callback("keydown", self.callback.as_ref().unchecked_ref());
	}
}
