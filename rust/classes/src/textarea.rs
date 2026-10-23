use crate::Size;

pub const TEXTAREA_BASE: &str = "border-input placeholder:text-ink-soft focus-visible:border-ring focus-visible:ring-1 focus-visible:ring-ring aria-invalid:border-accent-error aria-invalid:focus-visible:ring-accent-error \
                             flex field-sizing-content min-h-16 w-full rounded-[var(--control-radius)] \
                             border bg-muted px-2 py-1.5 text-base pointer-fine:text-body transition-colors \
                             outline-none disabled:cursor-not-allowed disabled:opacity-40";

/// Minimum height, and for `Touch` the inset and type, fused over
/// [`TEXTAREA_BASE`]. The same three steps and the same 16px floor at `Touch` as
/// [`input_size_class`](crate::input_size_class).
pub fn textarea_size_class(size: Size) -> &'static str {
	match size {
		Size::Xs | Size::Sm => "min-h-14",
		Size::Md | Size::Lg => "min-h-16",
		Size::Touch | Size::Xl => "min-h-20 px-3 py-2 text-base pointer-fine:text-base",
	}
}
