/// Fused over the dialog content's own classes: it drops the palette into the top
/// sixth of the screen instead of centring it.
pub const COMMAND_PALETTE_CONTENT: &str = "top-1/6 max-w-[calc(100%-2rem)] translate-y-0 gap-0 overflow-hidden bg-popover p-0 sm:max-w-105";

pub const COMMAND_PALETTE_TITLE: &str = "sr-only";

pub const COMMAND_PALETTE_COMMAND: &str = "bg-transparent";

pub const COMMAND_PALETTE_LIST: &str = "max-h-72";

pub const COMMAND_PALETTE_LABEL: &str = "min-w-0 flex-1 truncate";

pub const COMMAND_PALETTE_HINTS: &str = "flex items-center gap-3 border-t border-border-subtle px-2 py-1 text-caption text-ink-soft";

pub const COMMAND_PALETTE_HINT: &str = "inline-flex items-center gap-1.5";
