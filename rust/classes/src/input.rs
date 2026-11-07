use crate::Size;

pub const INPUT_BASE: &str = "file:text-ink placeholder:text-ink-soft selection:bg-primary \
                              selection:text-on-primary border-input h-(--control-h-md) w-full min-w-0 rounded-control border \
                              bg-transparent px-3 py-1 text-base shadow-xs transition-[color,box-shadow] outline-none \
                              file:inline-flex file:h-(--control-h-xs) file:border-0 file:bg-transparent file:text-sm file:font-medium \
                              disabled:pointer-events-none disabled:cursor-not-allowed disabled:opacity-(--disabled-opacity) md:text-sm \
                              focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] \
                              aria-invalid:ring-accent-error/20 aria-invalid:border-accent-error";

/// Height, and for `Lg` the inset and type, fused over [`INPUT_BASE`]. A form
/// field has three steps: `Xs` reads as `Sm` and `Xl` as `Lg`, and its `Lg` sits
/// on the `xl` rung, level with the call to action it is filled in for.
///
/// `Lg` holds 16px text at every width: iOS Safari zooms the page into any
/// focused field set smaller, and `md:text-sm` would be exactly that on a
/// tablet filling in a landing page's lead form.
pub fn input_size_class(size: Size) -> &'static str {
	match size {
		Size::Xs | Size::Sm => "h-(--control-h-sm)",
		Size::Md => "h-(--control-h-md)",
		Size::Lg | Size::Touch | Size::Xl => "h-(--control-h-xl) px-(--control-inset-md) text-base md:text-base file:h-(--control-h-md)",
	}
}
