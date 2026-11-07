use crate::focus::filled_focus_ring;

/// The track paints `primary` when on, so it wears the offset ring rather than
/// the halo — see [`FILLED_FOCUS_RING`](crate::FILLED_FOCUS_RING).
pub const SWITCH_BASE: &str = concat!(
	"peer data-[state=checked]:bg-primary data-[state=unchecked]:bg-input focus-visible:border-ring \
	 inline-flex h-[calc(1em+4px)] w-[calc(2em+4px)] shrink-0 items-center rounded-full border border-transparent shadow-xs \
	 transition-all outline-none disabled:cursor-not-allowed disabled:opacity-(--disabled-opacity) ",
	filled_focus_ring!()
);

pub const SWITCH_THUMB: &str = "data-[state=checked]:bg-on-primary data-[state=unchecked]:bg-ink pointer-events-none \
                                block size-[calc(1em+2px)] rounded-full ring-0 transition-transform \
                                data-[state=checked]:translate-x-[calc(100%-2px)] data-[state=unchecked]:translate-x-0";
