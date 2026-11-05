use super::KeyInput;

/// Orientation of a `Slider`; rendered as `data-orientation` so the landing
/// class selectors lay the track out horizontally or vertically.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr, strum::IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SliderOrientation {
	#[default]
	Horizontal,
	Vertical,
}

/// One slider's range, as its props give it on this render.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Slider {
	pub min: f64,
	pub max: f64,
	pub step: f64,
	pub orientation: SliderOrientation,
}
impl Slider {
	/// `value` clamped into the range and snapped to the nearest step.
	pub(crate) fn clamp(&self, value: f64) -> f64 {
		let clamped = value.clamp(self.min, self.max);
		if self.step <= 0.0 {
			return clamped;
		}
		let steps = ((clamped - self.min) / self.step).round();
		(self.min + steps * self.step).clamp(self.min, self.max)
	}

	/// The value a keydown on the thumb moves `current` to; `None` for a key the
	/// slider leaves alone.
	pub(crate) fn key(&self, input: &KeyInput, current: f64) -> Option<f64> {
		let next = match input.key.as_str() {
			"ArrowRight" | "ArrowUp" => current + self.step,
			"ArrowLeft" | "ArrowDown" => current - self.step,
			"Home" => self.min,
			"End" => self.max,
			_ => return None,
		};
		Some(self.clamp(next))
	}

	/// Maps a client coordinate along the active axis to a stepped value, given
	/// the track's origin and size on that axis. Vertical runs bottom-to-top.
	pub(crate) fn value_at(&self, client: f64, track: Track) -> f64 {
		let size = track.size.max(f64::EPSILON);
		let ratio = match self.orientation {
			SliderOrientation::Horizontal => (client - track.origin) / size,
			SliderOrientation::Vertical => 1.0 - (client - track.origin) / size,
		};
		self.clamp(self.min + ratio * (self.max - self.min))
	}

	/// The range's `style`, filling up to `value`.
	pub(crate) fn range_style(&self, value: f64) -> String {
		let p = self.percent(value);
		match self.orientation {
			SliderOrientation::Horizontal => format!("width: {p}%;"),
			SliderOrientation::Vertical => format!("height: {p}%;"),
		}
	}

	/// The thumb's `style`: absolutely positioned within the (relative) root and
	/// centred on `value` — without `position:absolute` the `%` offset is ignored
	/// and flow layout parks it at the end of the row.
	pub(crate) fn thumb_style(&self, value: f64) -> String {
		let p = self.percent(value);
		match self.orientation {
			SliderOrientation::Horizontal => format!("position: absolute; left: {p}%; top: 50%; transform: translate(-50%, -50%);"),
			SliderOrientation::Vertical => format!("position: absolute; bottom: {p}%; left: 50%; transform: translate(-50%, 50%);"),
		}
	}

	/// Position as a percentage string, ≤4 decimals with trailing zeros trimmed —
	/// matches the reference (Radix) serialization (`38.9474`, `25`), keeping the
	/// emitted `style` byte-stable instead of leaking f64 round-off.
	fn percent(&self, value: f64) -> String {
		let span = (self.max - self.min).max(f64::EPSILON);
		let s = format!("{:.4}", ((value - self.min) / span * 100.0).clamp(0.0, 100.0));
		s.trim_end_matches('0').trim_end_matches('.').to_owned()
	}
}

/// The track's extent on the slider's axis, as the port measured it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct Track {
	pub origin: f64,
	pub size: f64,
}

/// A pointer drag: the track as measured at the press, while one is open.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct Drag(Option<Track>);
impl Drag {
	pub(crate) fn press(&mut self, track: Track) {
		self.0 = Some(track);
	}

	pub(crate) fn release(&mut self) {
		self.0 = None;
	}

	/// The track a `pointermove` measures against, `None` when it moves nothing:
	/// no drag open (hover must not move the value), or disabled. A move with no
	/// button held closes the drag — the release happened where the root could
	/// not see it (no pointer capture off the web), and the bare cursor must not
	/// drive the value.
	pub(crate) fn moved(&mut self, disabled: bool, buttons_held: bool) -> Option<Track> {
		let track = self.0.filter(|_| !disabled)?;
		if !buttons_held {
			self.release();
			return None;
		}
		Some(track)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const PERCENT: Slider = Slider {
		min: 0.0,
		max: 100.0,
		step: 1.0,
		orientation: SliderOrientation::Horizontal,
	};
	const TRACK: Track = Track { origin: 10.0, size: 200.0 };

	#[test]
	fn values_clamp_into_the_range_and_snap_to_the_step() {
		assert_eq!(PERCENT.clamp(150.0), 100.0);
		assert_eq!(Slider { step: 5.0, ..PERCENT }.clamp(12.4), 10.0);
		assert_eq!(Slider { step: 0.0, ..PERCENT }.clamp(12.4), 12.4, "a zero step does not snap");
	}

	#[test]
	fn keys_step_and_jump_to_the_ends() {
		assert_eq!(PERCENT.key(&"ArrowRight".into(), 40.0), Some(41.0));
		assert_eq!(PERCENT.key(&"ArrowDown".into(), 0.0), Some(0.0), "stepping past min clamps");
		assert_eq!(PERCENT.key(&"End".into(), 40.0), Some(100.0));
		assert_eq!(PERCENT.key(&"Tab".into(), 40.0), None);
	}

	#[test]
	fn the_pointer_maps_along_the_axis_and_vertical_runs_upward() {
		assert_eq!(PERCENT.value_at(60.0, TRACK), 25.0);
		assert_eq!(PERCENT.value_at(-50.0, TRACK), 0.0);
		let vertical = Slider {
			orientation: SliderOrientation::Vertical,
			..PERCENT
		};
		assert_eq!(vertical.value_at(60.0, TRACK), 75.0);
	}

	#[test]
	fn styles_print_trimmed_percentages() {
		let odd = Slider { max: 19.0, step: 0.0, ..PERCENT };
		assert_eq!(odd.range_style(7.4), "width: 38.9474%;");
		assert_eq!(PERCENT.thumb_style(25.0), "position: absolute; left: 25%; top: 50%; transform: translate(-50%, -50%);");
	}

	#[test]
	fn hover_without_a_drag_never_moves_the_value() {
		let mut drag = Drag::default();
		assert_eq!(drag.moved(false, true), None);
		assert_eq!(drag.moved(false, false), None);
	}

	#[test]
	fn move_with_no_button_held_ends_a_stuck_drag() {
		let mut drag = Drag::default();
		drag.press(TRACK);
		assert_eq!(drag.moved(false, false), None);
		assert_eq!(drag, Drag::default(), "the drag is closed, not just skipped");
	}

	#[test]
	fn move_with_a_button_held_tracks() {
		let mut drag = Drag::default();
		drag.press(TRACK);
		assert_eq!(drag.moved(false, true), Some(TRACK));
	}

	#[test]
	fn disabled_ignores_moves_even_mid_drag() {
		let mut drag = Drag::default();
		drag.press(TRACK);
		assert_eq!(drag.moved(true, true), None);
		assert_eq!(drag.moved(false, true), Some(TRACK), "still open once re-enabled");
	}
}
