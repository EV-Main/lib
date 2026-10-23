pub const TABS_ROOT: &str = "flex flex-col gap-2";
/// A hairline under the row; the active tab draws its teal underline over it.
pub const TABS_LIST: &str = "text-ink-mid border-border-subtle inline-flex h-8 w-fit items-stretch justify-center border-b";
/// The ring sits inside the tab: the underline is the indicator and an outset
/// ring would be cut by the list's edge.
pub const TABS_TRIGGER: &str = "text-ink-mid hover:text-ink data-[state=active]:text-ink data-[state=active]:border-primary \
                            focus-visible:outline-ring inline-flex h-8 flex-1 items-center justify-center gap-1.5 -mb-px \
                            border-b border-transparent px-3 text-body font-medium whitespace-nowrap transition-colors \
                            outline-none focus-visible:outline-solid focus-visible:outline-2 focus-visible:-outline-offset-2 \
                            disabled:pointer-events-none disabled:opacity-40 \
                            [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4";
pub const TABS_CONTENT: &str = "flex-1 outline-none";
