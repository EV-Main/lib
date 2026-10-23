use crate::Size;

/// The Figma input: a 28 px well in `muted`, edged by `--input` (the 3:1 control
/// edge; Figma's own `border` is too faint). Focus turns the edge to `--ring` and
/// adds a 1 px ring so the change reads as more than a hue shift.
///
/// 16 px text on a coarse pointer: iOS Safari zooms the page into any focused
/// field set smaller (lib#154).
pub const INPUT_BASE: &str = "file:text-ink placeholder:text-ink-soft selection:bg-primary \
                              selection:text-on-primary border-input h-7 w-full min-w-0 rounded-[var(--control-radius)] border \
                              bg-muted px-2 text-base pointer-fine:text-body transition-colors outline-none \
                              file:inline-flex file:h-6 file:border-0 file:bg-transparent file:text-body file:font-medium \
                              disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-40 \
                              focus-visible:border-ring focus-visible:ring-1 focus-visible:ring-ring aria-invalid:border-accent-error aria-invalid:focus-visible:ring-accent-error";

/// Height, and for `Touch` the inset and type, fused over [`INPUT_BASE`]. A form
/// field has three steps, keyed `sm`, `md` and `touch`: `Xs` reads as `Sm`, `Lg` as
/// `Md` and `Xl` as `Touch`.
///
/// `Touch` is the 44 px touch-first field and holds 16px text at every width, not
/// only on a coarse pointer: a landing page's lead form is filled in on a tablet
/// that reports a fine one.
pub fn input_size_class(size: Size) -> &'static str {
	match size {
		Size::Xs | Size::Sm => "h-6",
		Size::Md | Size::Lg => "h-7",
		Size::Touch | Size::Xl => "h-11 px-3 text-base pointer-fine:text-base file:h-8",
	}
}
