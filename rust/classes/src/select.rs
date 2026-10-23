use crate::{Size, focus::option_focus_ring};

/// How big the open list may get. At least the trigger's width (the TS port
/// writes it to `--select-trigger-width`; the Rust one has no measure and keeps
/// the 8rem floor), at most 24rem and never wider or taller than the viewport
/// less a 1rem gutter a side — so the list the floating placement clamps
/// always fits between the edges, and a long option wraps (see
/// [`SELECT_ITEM`]) instead of widening the list off-screen. The trigger's width
/// wins over the 24rem cap — `min-width` beats `max-width` in CSS — so a trigger
/// wider than 24rem opens a list exactly as wide as itself.
pub const SELECT_CONTENT_BOUNDS: &str = "min-w-[max(8rem,var(--select-trigger-width,0px))] max-w-[min(24rem,calc(100vw-2rem))] \
	 max-h-[min(24rem,calc(100dvh-2rem))]";

/// One option of the open list. The focused row carries
/// [`OPTION_FOCUS_RING`](crate::OPTION_FOCUS_RING) over its tint.
///
/// `overflow-wrap: anywhere` rather than `break-words`: the label sits in a
/// flex item, whose minimum width is its min-content, and only `anywhere`
/// shrinks that — with `break-word` an unbroken word still props the row open.
pub const SELECT_ITEM: &str = concat!(
	"focus:bg-hover focus:text-ink ",
	option_focus_ring!(),
	" [&_svg:not([class*='text-'])]:text-ink-mid relative flex min-h-7 w-full cursor-default items-center gap-2 rounded-[var(--control-radius)] py-1 pr-8 pl-2 \
	 text-body text-ink-mid whitespace-normal [overflow-wrap:anywhere] outline-hidden select-none data-[disabled]:pointer-events-none data-[disabled]:opacity-40 [&_svg]:pointer-events-none \
	 [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4 *:[span]:last:flex *:[span]:last:items-center *:[span]:last:gap-2"
);

/// The `SelectTrigger`: a raised 28 px control with the `--input` edge. The hover
/// is a second layer over the fill, not a replacement of it.
pub const SELECT_TRIGGER_BASE: &str = "border-input [&_[data-placeholder]]:text-ink-soft [&_svg:not([class*='text-'])]:text-ink-soft \
	 focus-visible:border-ring focus-visible:ring-1 focus-visible:ring-ring aria-invalid:border-accent-error aria-invalid:focus-visible:ring-accent-error data-[state=open]:border-ring \
	 flex w-fit items-center justify-between gap-2 rounded-[var(--control-radius)] border bg-secondary pr-1.5 pl-2 text-body whitespace-nowrap \
	 hover:bg-[image:linear-gradient(var(--hover),var(--hover))] transition-colors outline-none disabled:cursor-not-allowed disabled:opacity-40 \
	 *:data-[slot=select-value]:line-clamp-1 *:data-[slot=select-value]:flex \
	 *:data-[slot=select-value]:items-center *:data-[slot=select-value]:gap-2 [&_svg]:pointer-events-none [&_svg]:shrink-0 \
	 [&_svg:not([class*='size-'])]:size-4";

/// The open list's surface; the placement, animation and layer classes stay with
/// each port's component. The inner padding is [`SELECT_LIST`].
pub const SELECT_CONTENT_SURFACE: &str = "bg-popover text-ink overflow-x-hidden overflow-y-auto rounded-lg border border-border shadow-elevated";

/// The padded column holding the options.
pub const SELECT_LIST: &str = "flex flex-col gap-0.5 p-1.5";

pub const SELECT_LABEL: &str = "text-ink-soft flex h-6 items-center px-2 text-body-s";

pub const SELECT_SEPARATOR: &str = "bg-border-subtle pointer-events-none my-1 h-px";

/// The native `<select>`; the size table fuses over it.
pub const NATIVE_SELECT_BASE: &str = "border-input text-ink h-7 w-full min-w-0 appearance-none rounded-[var(--control-radius)] border bg-secondary py-0 pr-9 pl-2 \
	 text-base pointer-fine:text-body transition-colors outline-none disabled:pointer-events-none disabled:cursor-not-allowed \
	 focus-visible:border-ring focus-visible:ring-1 focus-visible:ring-ring aria-invalid:border-accent-error aria-invalid:focus-visible:ring-accent-error has-[option[value='']:checked]:text-ink-soft \
	 [&_option]:bg-popover [&_option]:text-ink [&_optgroup]:bg-popover [&_optgroup]:text-ink-soft [color-scheme:var(--scheme,normal)]";

/// The `SelectTrigger`'s height, and for `Touch` its inset and type, on the same
/// three steps as [`input_size_class`](crate::input_size_class), so a trigger
/// lines up with the input beside it in a form row.
pub fn select_trigger_size_class(size: Size) -> &'static str {
	match size {
		Size::Xs | Size::Sm => "h-6",
		Size::Md | Size::Lg => "h-7",
		Size::Touch | Size::Xl => "h-11 pl-3 pr-2.5 text-base",
	}
}

/// The `NativeSelect`'s height, and for `Touch` its inset and type. The inset is
/// written per side because the right one clears the arrow: a `px-*` here would
/// fuse that clearance away.
pub fn native_select_size_class(size: Size) -> &'static str {
	match size {
		Size::Xs | Size::Sm => "h-6",
		Size::Md | Size::Lg => "h-7",
		Size::Touch | Size::Xl => "h-11 pr-11 pl-3 text-base pointer-fine:text-base",
	}
}
