use tailwind_fuse::{AsTailwindClass, TwVariant};

use crate::{Size, focus::filled_focus_ring};

/// Base classes shared by every button variant. Split out from the variant enum
/// so the TS codegen can emit it verbatim and `button_classes` fuses it last-wins.
///
/// The focus indicator is the offset ring on every variant: the faces are flat
/// and mostly transparent, so a halo hugging the edge has nothing to read
/// against (lib#129).
pub const BUTTON_BASE: &str = concat!(
	"inline-flex items-center justify-center gap-1.5 whitespace-nowrap rounded-[var(--control-radius)] text-body \
	 font-medium transition-colors cursor-pointer disabled:pointer-events-none disabled:opacity-40 \
	 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 \
	 outline-none aria-invalid:border-accent-error ",
	filled_focus_ring!()
);

/// Per-variant classes only; the base rides on [`BUTTON_BASE`]. `as_class()` yields
/// the variant string, the codegen key is `as_ref()` (kebab).
///
/// `Primary` and `Danger` are outlines whose faces come from the `--button-*`
/// tokens, so a theme decides whether they stay outlined (Dark, Light) or fill
/// (Oldschool). `Solid` is the one filled call to action.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ButtonVariant {
	#[tw(default, class = "border border-button-primary-border bg-button-primary-bg text-button-primary-ink hover:bg-button-primary-hover")]
	Primary,
	/// The hover is a second layer over the raised face, not a replacement, so the
	/// button keeps its fill while it lightens.
	#[tw(class = "border border-border bg-secondary text-ink hover:bg-[image:linear-gradient(var(--hover),var(--hover))]")]
	Secondary,
	#[tw(class = "text-ink-mid hover:bg-hover hover:text-ink")]
	Ghost,
	#[tw(class = "bg-primary text-on-primary hover:bg-primary-hover")]
	Solid,
	#[tw(class = "border border-button-danger-border bg-button-danger-bg text-button-danger-ink hover:bg-button-danger-hover")]
	Danger,
	#[tw(class = "text-primary-ink underline-offset-4 hover:underline")]
	Link,
}

/// Size + shape dimensions. An `icon` button is a square (`h-N aspect-square px-0`);
/// otherwise per-size height + text padding. Mirrors the TS `buttonSizeClasses` table.
pub fn button_size_class(size: Size, icon: bool) -> &'static str {
	match (size, icon) {
		(Size::Xs, false) => "h-5 gap-1 px-1.5 text-caption has-[>svg]:px-1",
		(Size::Sm, false) => "h-6 px-2 text-body-s has-[>svg]:px-1.5",
		(Size::Md, false) => "h-7 px-3 text-body has-[>svg]:px-2.5",
		(Size::Lg, false) => "h-8 px-4 text-body-l has-[>svg]:px-3",
		// the CTA size: geometry entirely from the control tokens, so a consumer
		// reshapes its call to action by writing values
		(Size::Xl, false) => "px-[var(--control-px)] py-[var(--control-py)] text-[length:var(--control-text)]",
		// a floor, not a fixed height: the text sizing matches `Lg`, the box never
		// drops under 44 px
		(Size::Touch, false) => "min-h-11 px-6 text-body-l has-[>svg]:px-4",
		(Size::Xs, true) => "h-5 aspect-square px-0",
		(Size::Sm, true) => "h-6 aspect-square px-0",
		(Size::Md, true) => "h-7 aspect-square px-0",
		(Size::Lg, true) => "h-8 aspect-square px-0",
		(Size::Xl, true) => "aspect-square p-[var(--control-py)]",
		(Size::Touch, true) => "size-11 px-0",
	}
}
