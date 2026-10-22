/// 14 % of the text ink, composited on the plane: 1.42:1 against card on Dark and
/// 1.35:1 on Light, with the ink indicator at >= 4:1 on it. The flat u2 planes
/// leave `bg-muted` at 1.10:1 vs card, and a tint of the indicator's own ink
/// trades one floor for the other.
pub const PROGRESS_TRACK: &str = "bg-ink/14 relative h-2 w-full overflow-hidden rounded-full";
/// The ink, not the fill: an indicator carries no label, so like the radio dot
/// it is identified by contrast alone (docs/spec/accents.md).
pub const PROGRESS_INDICATOR: &str = "bg-primary-ink h-full w-full flex-1 transition-all";
