use leptos::prelude::*;

use crate::cn;

/// The consumer's mark, painted as a CSS mask (`background-color: currentColor`
/// + `mask-image`) so a monochrome lockup follows the surrounding text colour.
/// `with_background` seats it on the brand field, keeping a chip consistent with
/// a favicon.
///
/// The kit ships no artwork: the consumer declares `--brand-mark` (a `url()` at
/// its own asset) and `--brand-aspect` beside its palette (see `tokens.css`).
/// Undeclared, the span paints as a solid block: loud, and nobody else's logo.
#[component]
pub fn Logo(#[prop(optional, into)] class: String, #[prop(optional)] with_background: bool) -> impl IntoView {
	let mark_cls = cn!("inline-block", if with_background { "w-3/5 h-3/5".to_string() } else { class.clone() });
	let mark = view! {
		<span
			data-slot="logo"
			aria-hidden="true"
			// the ratio's fallback is what makes a missing mark *visible*: a masked span has no intrinsic size
			style="background-color:currentColor;mask-image:var(--brand-mark);-webkit-mask-image:var(--brand-mark);mask-repeat:no-repeat;-webkit-mask-repeat:no-repeat;mask-position:center;-webkit-mask-position:center;mask-size:contain;-webkit-mask-size:contain;aspect-ratio:var(--brand-aspect,1)"
			class=mark_cls
		></span>
	};
	if !with_background {
		return mark.into_any();
	}
	view! {
		<span
			data-slot="logo-background"
			class=cn!("inline-flex items-center justify-center bg-brand", class)
		>
			{mark}
		</span>
	}
	.into_any()
}
