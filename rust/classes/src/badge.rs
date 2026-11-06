use crate::focus::filled_focus_ring;

/// A 20 px chip wearing its [`Tone`](crate::Tone) whole: fill, edge and label.
/// The fill and edge are the tone's tint, so a theme chooses between an outline
/// (Dark, Light) and a tinted pill (Oldschool) without the class table knowing.
///
/// A badge only takes focus as a link; it then wears the offset ring, and its
/// hover is a layer over the chip fill rather than a replacement for it.
pub const BADGE_BASE: &str = concat!(
	"inline-flex h-5 items-center justify-center rounded-chip border px-1.5 text-caption \
	 border-(color:--tone-edge) bg-(color:--tone-fill) text-(color:--tone-ink) \
	 font-medium w-fit whitespace-nowrap shrink-0 gap-1 overflow-hidden \
	 [&>svg]:size-3 [&>svg]:pointer-events-none transition-colors outline-none \
	 [a&]:hover:bg-[image:linear-gradient(var(--hover),var(--hover))] aria-invalid:border-accent-error ",
	filled_focus_ring!()
);
