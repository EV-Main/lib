use tailwind_fuse::{AsTailwindClass, TwVariant};

pub const ALERT_BASE: &str = "relative w-full rounded-lg border px-4 py-3 text-body grid \
                              has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] grid-cols-[0_1fr] \
                              has-[>svg]:gap-x-3 gap-y-0.5 items-start [&>svg]:size-4 [&>svg]:translate-y-0.5 \
                              [&>svg]:text-current";

pub const ALERT_TITLE: &str = "col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight";

pub const ALERT_DESCRIPTION: &str = "text-ink-soft col-start-2 grid justify-items-start gap-1 text-body [&_p]:leading-relaxed";

/// The badge tones (docs/spec/variants.md). A tinted alert colours the root in
/// the role's ink (so the icon and title inherit it) and edges it in the role's
/// chip edge; the description is lifted to `text-ink-mid` instead of a `/90` role
/// tint, which composited onto `bg-card` sits at ~4.0:1 — under the 4.5:1 body-text
/// floor — and bleeds into any control nested in the description.
#[derive(PartialEq, TwVariant, strum::AsRefStr, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum AlertVariant {
	#[tw(default, class = "bg-card text-ink border-border")]
	Neutral,
	#[tw(class = "text-accent-info-ink border-chip-info-border bg-card [&>svg]:text-current *:data-[slot=alert-description]:text-ink-mid")]
	Info,
	#[tw(class = "text-positive-ink border-chip-positive-border bg-card [&>svg]:text-current *:data-[slot=alert-description]:text-ink-mid")]
	Positive,
	#[tw(class = "text-accent-warn-ink border-chip-warn-border bg-card [&>svg]:text-current *:data-[slot=alert-description]:text-ink-mid")]
	Warn,
	#[tw(class = "text-accent-error-ink border-chip-error-border bg-card [&>svg]:text-current *:data-[slot=alert-description]:text-ink-mid")]
	Error,
}
