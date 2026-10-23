use tailwind_fuse::{AsTailwindClass, TwVariant};

pub const SCROLL_AREA_VIEWPORT: &str =
	"size-full rounded-[inherit] overflow-auto transition-colors outline-none focus-visible:outline-solid focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-ring";

pub const SCROLL_AREA_THUMB: &str = "bg-border relative flex-1 rounded-full";

pub const SCROLLBAR_BASE: &str = "flex touch-none p-px transition-colors select-none";

#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum ScrollBarOrientation {
	#[tw(default, class = "h-full w-2.5 border-l border-l-transparent")]
	Vertical,
	#[tw(class = "h-2.5 flex-col border-t border-t-transparent")]
	Horizontal,
}
