use tailwind_fuse::{AsTailwindClass, TwVariant};

use crate::focus::filled_focus_ring;

/// On, the toggle paints the `primary` fill (lib#175): the `bg-hover` tint it
/// wore before measured ~1.2:1 against the surface and was the very colour an
/// outline toggle wore on hover, so a selected item could not be told from a
/// pointed-at one. A fill, it wears the offset ring rather than the halo — see
/// [`FILLED_FOCUS_RING`](crate::FILLED_FOCUS_RING). A tool, it packs at half a
/// button's inset.
pub const TOGGLE_BASE: &str = concat!(
	"inline-flex h-(--control-box) min-w-(--control-box) items-center justify-center rounded-control font-medium \
	 px-[calc(var(--control-pad)*0.5)] gap-[calc(var(--control-pad)*0.5)] \
	 hover:bg-hover hover:text-ink disabled:pointer-events-none disabled:opacity-(--disabled-opacity) \
	 data-[state=on]:bg-primary data-[state=on]:text-on-primary [&_svg]:pointer-events-none \
	 [&_svg:not([class*='size-'])]:size-[calc(1em+2px)] [&_svg]:shrink-0 focus-visible:border-ring outline-none \
	 transition-[color,box-shadow] aria-invalid:ring-accent-error/20 aria-invalid:border-accent-error \
	 whitespace-nowrap ",
	filled_focus_ring!()
);

#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ToggleVariant {
	#[tw(default, class = "bg-transparent")]
	Bare,
	#[tw(class = "border border-input bg-transparent shadow-xs hover:bg-hover hover:text-ink data-[state=on]:border-primary")]
	Outline,
}
