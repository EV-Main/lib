use crate::focus::filled_focus_ring;

pub const RADIO_GROUP_ROOT: &str = "grid gap-3";
/// Unfilled, but the offset ring is the kit's indicator for a control with no
/// edge of its own to recolour that reads on every plane.
pub const RADIO_GROUP_ITEM: &str = concat!(
	"border-input text-primary-ink bg-muted aria-invalid:border-accent-error aspect-square size-4 \
	 shrink-0 rounded-full border transition-colors outline-none disabled:cursor-not-allowed disabled:opacity-40 ",
	filled_focus_ring!()
);
