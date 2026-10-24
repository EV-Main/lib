pub const TOP_BAR_BASE: &str = "flex h-14 items-center gap-3 border-b border-border bg-background px-8";

/// `min-w-0` lets a long service name truncate instead of pushing the end
/// group off the bar.
pub const TOP_BAR_START: &str = "flex min-w-0 items-center gap-2";

/// `shrink-0` keeps the account menu whole when the start side is long.
pub const TOP_BAR_END: &str = "ml-auto flex shrink-0 items-center gap-3";
