pub const COMMAND_ROOT: &str = "bg-popover text-ink flex h-full w-full flex-col overflow-hidden rounded-panel";

pub const COMMAND_DIALOG_OVERLAY: &str = "fixed inset-0 z-50 bg-scrim";

pub const COMMAND_DIALOG_CONTENT: &str = "fixed top-1/2 left-1/2 z-50 w-full max-w-lg -translate-x-1/2 -translate-y-1/2 overflow-hidden rounded-panel border border-border p-0 shadow-lg";

pub const COMMAND_DIALOG_COMMAND: &str = "[&_[data-slot=command-input-wrapper]]:h-(--control-h-xl) [&_[data-slot=command-input]]:h-(--control-h-xl)";

pub const COMMAND_INPUT_WRAPPER: &str = "flex h-(--control-h-md) items-center gap-2 border-b border-border px-3 has-[[aria-invalid=true]]:border-accent-error";

pub const COMMAND_INPUT: &str =
	"placeholder:text-ink-soft flex h-(--control-h-lg) w-full rounded-control bg-transparent py-3 text-sm outline-hidden disabled:cursor-not-allowed disabled:opacity-(--disabled-opacity)";

/// A column, not a grid: rows carry their rank as CSS `order`, which needs a flex or
/// grid container, but a grid item with `overflow-hidden` (a group) has a minimum
/// height of 0 and the grid squeezed every group to fit `max-h` instead of letting
/// the list scroll. `shrink-0` keeps each child its own height.
pub const COMMAND_LIST: &str = "flex max-h-[300px] scroll-py-1 flex-col overflow-x-hidden overflow-y-auto [&>*]:shrink-0";

pub const COMMAND_EMPTY: &str = "py-6 text-center text-sm";

pub const COMMAND_GROUP: &str = "grid text-ink [&_[data-slot=command-group-heading]]:text-ink-soft overflow-hidden p-1 [&>[data-slot=command-group-heading]]:order-first [&_[data-slot=command-group-heading]]:px-2 [&_[data-slot=command-group-heading]]:py-1.5 [&_[data-slot=command-group-heading]]:text-xs [&_[data-slot=command-group-heading]]:font-medium";

pub const COMMAND_ITEM: &str = "data-[selected=true]:bg-hover data-[selected=true]:text-ink [&_svg:not([class*='text-'])]:text-ink-soft relative flex cursor-default items-center gap-2 rounded-row px-2 py-1.5 text-sm outline-hidden select-none data-[disabled=true]:pointer-events-none data-[disabled=true]:opacity-(--disabled-opacity) [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";

pub const COMMAND_SEPARATOR: &str = "bg-border -mx-1 h-px";

pub const COMMAND_SHORTCUT: &str = "text-ink-soft ml-auto text-xs tracking-widest";
pub const COMMAND_INPUT_ICON: &str = "size-4 shrink-0 opacity-50";
