/// Wears its [`Tone`](crate::Tone) whole, the fill as a layer over the card; the
/// icon and title take the ink, the description stays `ink-mid`.
pub const ALERT_BASE: &str = "relative w-full rounded-panel border border-(color:--tone-edge) bg-card \
                              bg-[image:linear-gradient(var(--tone-fill),var(--tone-fill))] px-4 py-3 text-sm text-(color:--tone-ink) grid \
                              has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] grid-cols-[0_1fr] \
                              has-[>svg]:gap-x-3 gap-y-0.5 items-start [&>svg]:size-4 [&>svg]:translate-y-0.5 \
                              [&>svg]:text-current";

pub const ALERT_TITLE: &str = "col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight";

pub const ALERT_DESCRIPTION: &str = "text-ink-mid col-start-2 grid justify-items-start gap-1 text-sm [&_p]:leading-relaxed";
