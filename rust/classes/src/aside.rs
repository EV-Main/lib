use tailwind_fuse::{AsTailwindClass, TwVariant};

/// The 44 px title row.
pub const ASIDE_HEADER_BASE: &str = "flex h-11 shrink-0 items-center gap-1.5 border-b border-border bg-card pl-3 pr-2";

/// `min-w-0` with `truncate`: a long title ends in an ellipsis and the buttons keep their room.
pub const ASIDE_HEADER_TITLE: &str = "min-w-0 flex-1 truncate text-body font-medium text-ink";

/// The back button reuses the chevron-right glyph, turned to point back.
pub const ASIDE_HEADER_BACK_ICON: &str = "size-4 rotate-180";

pub const ASIDE_HEADER_CLOSE_ICON: &str = "size-4";

pub const ASIDE_RAIL_BASE: &str = "flex w-11 shrink-0 flex-col items-center gap-1 border-l border-border bg-card px-1.5 py-2";

/// Pushes the end items to the bottom of the rail.
pub const ASIDE_RAIL_SPACER: &str = "flex-1";

pub const ASIDE_BODY: &str = "min-h-0 flex-1 overflow-y-auto";

pub const ASIDE_FOOTER: &str = "flex shrink-0 items-center justify-end gap-2 border-t border-border bg-card px-4 py-3";

pub const ASIDE_PANEL_DOCKED: &str = "flex h-full w-95 shrink-0 flex-col bg-card";

/// The overlay below `md`. The Sheet close button is hidden because the
/// `AsideHeader` carries its own.
pub const ASIDE_PANEL_DRAWER: &str = "flex max-h-[85dvh] flex-col bg-card";

pub const ASIDE_PANEL_SHEET: &str = "w-full gap-0 border-border-strong bg-card p-0 sm:max-w-105 [&>[data-slot=sheet-close]]:hidden";

/// Dioxus only: the Dioxus port has no `matchMedia`, so it renders the wide
/// and the narrow form together and lets the `md` breakpoint show one of them.
pub const ASIDE_PANEL_WIDE_ONLY: &str = "hidden md:flex";

/// Dioxus only, see [`ASIDE_PANEL_WIDE_ONLY`]. `contents`, so the wrapper adds no box.
pub const ASIDE_PANEL_WIDE_ONLY_WRAPPER: &str = "hidden md:contents";

/// Dioxus only, see [`ASIDE_PANEL_WIDE_ONLY`].
pub const ASIDE_PANEL_NARROW_ONLY: &str = "contents md:hidden";

/// The edge of the screen the panel hugs. Only the edge that faces the content
/// has a border.
#[derive(Debug, PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum AsidePanelSide {
	#[tw(class = "border-r border-border")]
	Left,
	#[tw(default, class = "border-l border-border")]
	Right,
}
