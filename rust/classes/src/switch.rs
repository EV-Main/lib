use crate::focus::filled_focus_ring;

/// The track paints `primary` when on, so it wears the offset ring rather than
/// the halo — see [`FILLED_FOCUS_RING`](crate::FILLED_FOCUS_RING). Off, it is a
/// muted well edged by `--input`, the 3:1 control edge, where Figma draws the
/// fainter `border-strong`.
pub const SWITCH_BASE: &str = concat!(
	"peer data-[state=checked]:bg-primary data-[state=checked]:border-transparent data-[state=unchecked]:bg-muted \
	 data-[state=unchecked]:border-input focus-visible:border-ring \
	 inline-flex h-4 w-7 shrink-0 items-center rounded-full border \
	 transition-colors outline-none disabled:cursor-not-allowed disabled:opacity-40 ",
	filled_focus_ring!()
);

/// 12 px knob. The thumb's travel is the track's inner width less the knob and
/// the 2 px inset on each side, in px so it does not depend on the knob's size.
pub const SWITCH_THUMB: &str = "data-[state=checked]:bg-on-primary data-[state=unchecked]:bg-ink-mid pointer-events-none \
                                block size-3 rounded-full ring-0 transition-transform \
                                data-[state=checked]:translate-x-[13px] data-[state=unchecked]:translate-x-0.5";
