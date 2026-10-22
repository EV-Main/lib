pub const SLIDER_ROOT: &str = "relative flex w-full touch-none items-center select-none data-[disabled]:opacity-50 \
                               data-[orientation=vertical]:h-full data-[orientation=vertical]:min-h-44 \
                               data-[orientation=vertical]:w-auto data-[orientation=vertical]:flex-col";
/// 14 % of the text ink, composited on the plane: 1.42:1 against card on Dark and
/// 1.35:1 on Light, with the ink range at >= 4:1 on it. The flat u2 planes leave
/// `bg-muted` at 1.10:1 vs card, so the track is a tint of the text ink instead.
pub const SLIDER_TRACK: &str = "bg-ink/14 relative grow overflow-hidden rounded-full \
                                data-[orientation=horizontal]:h-1.5 data-[orientation=horizontal]:w-full \
                                data-[orientation=vertical]:h-full data-[orientation=vertical]:w-1.5";
/// The ink, not the fill: a range carries no label, so like the radio dot it is
/// identified by contrast alone, and the fill (`--primary`) is the dimmer of
/// the two on the track (docs/spec/accents.md).
pub const SLIDER_RANGE: &str = "bg-primary-ink absolute data-[orientation=horizontal]:h-full data-[orientation=vertical]:w-full";
/// An outline adjoining its own fill follows that fill — here the ink range the
/// thumb rides on.
pub const SLIDER_THUMB: &str = "border-primary-ink ring-ring/50 block size-4 shrink-0 rounded-full border bg-ink shadow-sm \
                                transition-[color,box-shadow] hover:ring-4 focus-visible:ring-4 focus-visible:outline-hidden \
                                disabled:pointer-events-none disabled:opacity-50";
