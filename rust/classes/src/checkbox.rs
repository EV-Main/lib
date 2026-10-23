use crate::focus::filled_focus_ring;

/// Checked, the box paints `primary`, so it wears the offset ring rather than
/// the halo — see [`FILLED_FOCUS_RING`](crate::FILLED_FOCUS_RING).
pub const CHECKBOX_BASE: &str = concat!(
	"peer border-input bg-muted data-[state=checked]:bg-primary data-[state=checked]:text-on-primary \
	 data-[state=checked]:border-primary focus-visible:border-ring \
	 aria-invalid:border-accent-error size-4 shrink-0 rounded-[var(--control-radius)] border transition-colors \
	 outline-none disabled:cursor-not-allowed disabled:opacity-40 ",
	filled_focus_ring!()
);

pub const CHECKBOX_INDICATOR: &str = "flex items-center justify-center text-current transition-none";
