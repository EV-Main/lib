pub const ACCORDION_ITEM: &str = "border-b border-border last:border-b-0";
pub const ACCORDION_HEADER: &str = "flex";
pub const ACCORDION_TRIGGER: &str = "flex flex-1 items-start justify-between gap-4 \
                                 rounded-md py-4 text-left text-sm font-medium transition-all outline-none hover:underline \
                                 focus-visible:outline-solid focus-visible:outline-2 focus-visible:-outline-offset-2 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-40 [&[data-state=open]>svg]:rotate-180";
pub const ACCORDION_CONTENT: &str = "data-[state=closed]:animate-accordion-up data-[state=open]:animate-accordion-down overflow-hidden text-sm";
pub const ACCORDION_CONTENT_INNER: &str = "pt-0 pb-4";
