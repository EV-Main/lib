pub const CARD: &str = "bg-card text-ink flex flex-col gap-4 rounded-lg border border-border py-4 shadow-card";

pub const CARD_HEADER: &str = "@container/card-header grid auto-rows-min grid-rows-[auto_auto] items-start gap-2 px-4 \
     has-data-[slot=card-action]:grid-cols-[1fr_auto] [.border-b]:pb-4";

pub const CARD_TITLE: &str = "text-h3 text-ink font-medium";

pub const CARD_DESCRIPTION: &str = "text-ink-mid text-body";

pub const CARD_ACTION: &str = "col-start-2 row-span-2 row-start-1 self-start justify-self-end";

pub const CARD_CONTENT: &str = "px-4";

pub const CARD_FOOTER: &str = "flex items-center px-4 [.border-t]:pt-4";
