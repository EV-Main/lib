/// Where the panel sits under its trigger for an `align`: `start` pins the left
/// edges, `end` the right, anything else centres it.
pub(crate) fn popover_placement(align: &str) -> &'static str {
	match align {
		"start" => "absolute top-full left-0 mt-1",
		"end" => "absolute top-full right-0 mt-1",
		_ => "absolute top-full left-1/2 -translate-x-1/2 mt-1",
	}
}
