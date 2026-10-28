/// An order-book level's depth bar as a CSS percentage: the fraction is clamped
/// and rounded once here; a NaN from an empty book reads as no depth rather than
/// as `NaN%`.
pub(crate) fn depth_percent(depth: f64) -> u8 {
	if depth.is_nan() {
		return 0;
	}
	// Truncation is the point: the value is clamped to 0..=100 first.
	(depth.clamp(0.0, 1.0) * 100.0).round() as u8
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn depth_is_clamped_to_a_percentage() {
		assert_eq!(depth_percent(-0.5), 0);
		assert_eq!(depth_percent(0.0), 0);
		assert_eq!(depth_percent(0.5), 50);
		assert_eq!(depth_percent(1.7), 100);
		assert_eq!(depth_percent(f64::NAN), 0);
	}
}
