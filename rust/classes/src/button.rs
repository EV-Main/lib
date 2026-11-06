use tailwind_fuse::{AsTailwindClass, TwVariant};

use crate::Size;

/// Base classes shared by every button variant. Split out from the variant enum
/// so the TS codegen can emit it verbatim and `button_classes` fuses it last-wins.
pub const BUTTON_BASE: &str = "inline-flex h-(--control-box) items-center justify-center gap-2 whitespace-nowrap rounded-control \
                               font-medium transition-all cursor-pointer disabled:pointer-events-none disabled:opacity-40 \
                               [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-[calc(1em+2px)] shrink-0 [&_svg]:shrink-0 \
                               outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] \
                               aria-invalid:ring-accent-error/20 aria-invalid:border-accent-error";

/// Per-variant classes only; the base rides on [`BUTTON_BASE`]. `as_class()` yields
/// the variant string, the codegen key is `as_ref()` (kebab).
///
/// The variants that paint a fill end in [`FILLED_FOCUS_RING`](crate::FILLED_FOCUS_RING),
/// which overrides the base's halo; the rest sit on the surface and keep it.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ButtonVariant {
	#[tw(
		default,
		class = "bg-primary text-on-primary hover:bg-primary-hover focus-visible:ring-0 focus-visible:outline-solid focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
	)]
	Primary,
	#[tw(
		class = "bg-secondary text-on-secondary hover:bg-secondary/80 focus-visible:ring-0 focus-visible:outline-solid focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
	)]
	Secondary,
	/// The hover is a second layer over the raised face, not a replacement, so the
	/// button keeps its fill while it lightens.
	#[tw(class = "border border-border bg-secondary text-ink hover:bg-[image:linear-gradient(var(--hover),var(--hover))]")]
	Outline,
	#[tw(class = "hover:bg-hover hover:text-ink")]
	Ghost,
	/// The face is the theme's `--button-danger-*`: an outline in Dark and Light, a
	/// fill in Oldschool.
	#[tw(
		class = "border border-button-danger-border bg-button-danger-bg text-button-danger-ink hover:bg-button-danger-hover focus-visible:ring-0 focus-visible:outline-solid focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
	)]
	Destructive,
	#[tw(class = "text-primary-ink underline-offset-4 hover:underline")]
	Link,
}

/// A button holding a bare glyph: the square of its step's height, in place of
/// [`button_size_class`].
pub const BUTTON_ICON: &str = "aspect-square px-0";

/// Padding and gap per step; the height is the [`Size`]'s `--control-box`.
/// Mirrors the TS `buttonSizeClasses` table.
pub fn button_size_class(size: Size) -> &'static str {
	match size {
		Size::Xs => "gap-1 px-2 has-[>svg]:px-1.5",
		Size::Sm => "gap-1.5 px-3 has-[>svg]:px-2.5",
		Size::Md => "px-4 has-[>svg]:px-3",
		Size::Lg | Size::Touch => "px-6 has-[>svg]:px-4",
		// the CTA size: a consumer reshapes its call to action by writing the control tokens
		Size::Xl => "px-[var(--control-px)] text-[length:var(--control-text)]",
	}
}
