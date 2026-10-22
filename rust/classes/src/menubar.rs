use crate::focus::option_focus_ring;

pub const MENUBAR_ROOT: &str = "bg-background flex h-8 items-center gap-0.5 rounded-lg border border-border p-0.5";
pub const MENUBAR_TRIGGER: &str = concat!(
	"focus:bg-hover focus:text-ink ",
	option_focus_ring!(),
	" data-[state=open]:bg-hover data-[state=open]:text-ink flex h-7 items-center rounded-[var(--control-radius)] px-2 text-body font-medium text-ink-mid outline-hidden select-none"
);
pub const MENUBAR_CONTENT: &str = "bg-popover text-ink data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 absolute z-50 min-w-[8rem] overflow-x-hidden overflow-y-auto flex flex-col gap-0.5 rounded-lg border border-border p-1.5 shadow-elevated";
pub const MENUBAR_SUB_CONTENT: &str = "bg-popover text-ink data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 data-[side=bottom]:slide-in-from-top-2 data-[side=left]:slide-in-from-right-2 data-[side=right]:slide-in-from-left-2 data-[side=top]:slide-in-from-bottom-2 absolute z-50 min-w-[8rem] overflow-hidden flex flex-col gap-0.5 rounded-lg border border-border p-1.5 shadow-elevated";
pub const MENUBAR_ITEM: &str = concat!(
	"focus:bg-hover focus:text-ink ",
	option_focus_ring!(),
	" data-[variant=danger]:text-accent-error-ink data-[variant=danger]:focus:bg-accent-error/10 data-[variant=danger]:focus:text-accent-error-ink data-[variant=danger]:*:[svg]:!text-accent-error-ink data-[inset]:pl-8 relative flex h-7 cursor-default items-center gap-2 rounded-[var(--control-radius)] px-2 text-body text-ink-mid outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-ink-mid"
);
pub const MENUBAR_CHECKBOX_ITEM: &str = concat!(
	"focus:bg-hover focus:text-ink ",
	option_focus_ring!(),
	" relative flex h-7 cursor-default items-center gap-2 rounded-[var(--control-radius)] pr-2 pl-8 text-body text-ink-mid outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4"
);
pub const MENUBAR_RADIO_ITEM: &str = concat!(
	"focus:bg-hover focus:text-ink ",
	option_focus_ring!(),
	" relative flex h-7 cursor-default items-center gap-2 rounded-[var(--control-radius)] pr-2 pl-8 text-body text-ink-mid outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 data-[state=checked]:bg-active data-[state=checked]:text-ink"
);
pub const MENUBAR_SUB_TRIGGER: &str = concat!(
	"focus:bg-hover focus:text-ink ",
	option_focus_ring!(),
	" data-[state=open]:bg-hover data-[state=open]:text-ink flex h-7 cursor-default items-center gap-2 rounded-[var(--control-radius)] px-2 text-body text-ink-mid outline-hidden select-none data-[inset]:pl-8 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 [&_svg:not([class*='text-'])]:text-ink-mid"
);
pub const MENUBAR_ITEM_INDICATOR: &str = "pointer-events-none absolute left-2 flex size-4 items-center justify-center";
pub const MENUBAR_LABEL: &str = "flex h-6 items-center px-2 text-body-s text-ink-soft data-[inset]:pl-8";
pub const MENUBAR_SEPARATOR: &str = "bg-border-subtle my-1 h-px";
pub const MENUBAR_SHORTCUT: &str = "text-ink-soft ml-auto text-caption tabular-nums";
