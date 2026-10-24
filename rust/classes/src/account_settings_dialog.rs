/// Fused over `DIALOG_CONTENT`: the pane and the rail carry their own padding,
/// so the content drops its padding and its gap.
pub const ACCOUNT_SETTINGS_DIALOG_CONTENT: &str = "flex max-h-[90dvh] max-w-[calc(100%-2rem)] flex-col gap-0 overflow-hidden bg-popover p-0 sm:max-w-260 md:h-170 md:flex-row";

pub const ACCOUNT_SETTINGS_DIALOG_TABS: &str = "min-h-0 flex-1 flex-col gap-0 md:flex-row";

/// Under `md` the rail sits above the pane and scrolls inside its own short
/// strip, so a long section list does not push the content off the screen.
pub const ACCOUNT_SETTINGS_DIALOG_SIDEBAR: &str = "flex max-h-44 shrink-0 flex-col gap-2 overflow-y-auto border-b border-border-subtle bg-card p-2 \
                                                   md:max-h-none md:w-60 md:border-b-0 md:border-r";

pub const ACCOUNT_SETTINGS_DIALOG_LIST: &str = "h-auto w-full flex-1 flex-col items-stretch justify-start gap-0.5 bg-transparent p-0";

/// Fused over `TABS_TRIGGER`: a rail entry is a filled row, so the underline
/// tab look is switched off.
pub const ACCOUNT_SETTINGS_DIALOG_ENTRY: &str = "h-8 w-full flex-none justify-start gap-2.5 rounded-(--control-radius) border-0 px-2.5 text-body font-normal text-ink-mid shadow-none \
                                                 data-[state=active]:bg-active data-[state=active]:text-ink data-[state=active]:shadow-none hover:bg-hover [&_svg]:text-ink-soft";

pub const ACCOUNT_SETTINGS_DIALOG_FOOTER: &str = "flex items-center gap-1 border-t border-border-subtle pt-2";

pub const ACCOUNT_SETTINGS_DIALOG_PANE: &str = "relative flex min-h-0 min-w-0 flex-1 flex-col gap-5 overflow-y-auto p-6 md:p-8";

/// The right padding keeps the heading clear of the pinned close button.
pub const ACCOUNT_SETTINGS_DIALOG_HEADING: &str = "flex flex-col gap-1 pr-10";

pub const ACCOUNT_SETTINGS_DIALOG_TITLE: &str = "font-display text-h1 font-light text-ink";

pub const ACCOUNT_SETTINGS_DIALOG_DESCRIPTION: &str = "text-body text-ink-mid";

pub const ACCOUNT_SETTINGS_DIALOG_PANEL: &str = "flex flex-col gap-5";

pub const ACCOUNT_SETTINGS_DIALOG_CLOSE: &str = "absolute right-4 top-4";
