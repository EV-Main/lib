use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{Accent, BUTTON_BASE, ButtonVariant, Size, button_accent_class, button_size_class},
};

/// Fuses the base, variant, size and accent classes with a caller override, last
/// wins. Mirrors the TS `buttonVariants` helper so consumers (e.g. pagination)
/// can reuse the same canonical class string without rendering a `Button`.
///
/// The accent fuses after the variant so it recolours whatever the variant set.
pub fn button_classes(variant: &ButtonVariant, size: Size, icon: bool, accent: Option<Accent>, class: &str) -> String {
	cn!(
		BUTTON_BASE,
		variant.as_class(),
		button_size_class(size, icon),
		accent.map_or("", |a| button_accent_class(a, variant)),
		class
	)
}

/// With `href` this is an `<a>` wearing the button's classes — a link that looks
/// like a call to action is still a link, and the kit has no `asChild` slot to
/// express that otherwise.
#[component]
pub fn Button(
	#[prop(optional)] variant: ButtonVariant,
	#[prop(optional)] size: Size,
	#[prop(optional)] icon: bool,
	/// Recolours the button at an [`Accent`] rung; unset is the normal case.
	#[prop(optional)]
	accent: Option<Accent>,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] href: Option<String>,
	/// Left unset the HTML default applies (`submit` inside a `Form`), mirroring
	/// the TS port where `type` is just another forwarded button prop. Callers
	/// that must not submit — addons, toolbars — pass `"button"`.
	#[prop(optional, into)]
	r#type: Option<String>,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	let cls = button_classes(&variant, size, icon, accent, &class);
	let click = move |e| {
		if let Some(h) = on_click {
			h.run(e);
		}
	};
	match href {
		Some(href) => view! {
			<a class=cls data-slot="button" href=href on:click=click>
				{children()}
			</a>
		}
		.into_any(),
		None => view! {
			<button class=cls data-slot="button" type=r#type disabled=disabled on:click=click>
				{children()}
			</button>
		}
		.into_any(),
	}
}
