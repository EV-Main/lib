pub const SIDEBAR_MENU_BUTTON_BASE: &str = "peer/menu-button flex w-full items-center gap-2 overflow-hidden rounded-[var(--control-radius)] p-2 text-left text-body \
                                            outline-hidden ring-ring transition-[width,height,padding] hover:bg-hover \
                                            hover:text-ink focus-visible:ring-2 active:bg-active \
                                            active:text-ink disabled:pointer-events-none disabled:opacity-40 \
                                            group-has-data-[sidebar=menu-action]/menu-item:pr-8 aria-disabled:pointer-events-none \
                                            aria-disabled:opacity-40 data-[active=true]:bg-active data-[active=true]:font-medium \
                                            data-[active=true]:text-ink data-[state=open]:hover:bg-hover \
                                            data-[state=open]:hover:text-ink group-data-[collapsible=icon]:size-8! \
                                            group-data-[collapsible=icon]:p-2! [&>span:last-child]:truncate [&>svg]:size-4 [&>svg]:shrink-0";

pub const SIDEBAR_WRAPPER: &str = "group/sidebar-wrapper has-data-[variant=inset]:bg-secondary flex min-h-svh w-full";

pub const SIDEBAR_FLAT: &str = "bg-secondary text-ink flex h-full w-(--sidebar-width) flex-col";

pub const SIDEBAR_INNER: &str = "bg-secondary group-data-[variant=floating]:border-border flex h-full w-full flex-col group-data-[variant=floating]:rounded-lg group-data-[variant=floating]:border group-data-[variant=floating]:shadow-card";

pub const SIDEBAR_TRIGGER: &str = "inline-flex size-7 items-center justify-center rounded-[var(--control-radius)] hover:bg-hover hover:text-ink";

pub const SIDEBAR_RAIL: &str = "hover:after:bg-border absolute inset-y-0 z-20 hidden w-4 -translate-x-1/2 transition-all ease-linear group-data-[side=left]:-right-4 group-data-[side=right]:left-0 after:absolute after:inset-y-0 after:left-1/2 after:w-[2px] sm:flex";

pub const SIDEBAR_INSET: &str = "bg-background relative flex w-full flex-1 flex-col md:peer-data-[variant=inset]:m-2 md:peer-data-[variant=inset]:ml-0 md:peer-data-[variant=inset]:rounded-lg md:peer-data-[variant=inset]:shadow-card md:peer-data-[variant=inset]:peer-data-[state=collapsed]:ml-2";

pub const SIDEBAR_HEADER: &str = "flex flex-col gap-2 p-2";

pub const SIDEBAR_FOOTER: &str = "flex flex-col gap-2 p-2";

pub const SIDEBAR_SEPARATOR: &str = "bg-border mx-2 h-px w-auto shrink-0";

pub const SIDEBAR_CONTENT: &str = "flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden";

pub const SIDEBAR_GROUP: &str = "relative flex w-full min-w-0 flex-col p-2";

pub const SIDEBAR_GROUP_LABEL: &str = "text-ink-soft ring-ring flex h-6 shrink-0 items-center rounded-[var(--control-radius)] px-2 text-body-s outline-hidden transition-[margin,opacity] duration-200 ease-linear focus-visible:ring-2 [&>svg]:size-4 [&>svg]:shrink-0 group-data-[collapsible=icon]:-mt-8 group-data-[collapsible=icon]:opacity-0";

pub const SIDEBAR_GROUP_CONTENT: &str = "w-full text-body";

pub const SIDEBAR_MENU: &str = "flex w-full min-w-0 flex-col gap-1";

pub const SIDEBAR_MENU_ITEM: &str = "group/menu-item relative";

/// The rail's rows are full-width and left-aligned, so they take their height
/// from the shared [`Size`](crate::Size) scale and nothing else — colour comes
/// from `ButtonVariant`, like every other button in the kit.
pub fn sidebar_menu_button_size_class(size: crate::Size) -> &'static str {
	match size {
		crate::Size::Xs => "h-6 text-body-s",
		crate::Size::Sm | crate::Size::Md => "h-8 text-body",
		crate::Size::Lg | crate::Size::Touch | crate::Size::Xl => "h-11 text-body group-data-[collapsible=icon]:p-0!",
	}
}
