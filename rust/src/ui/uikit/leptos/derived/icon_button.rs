use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{ICON_BUTTON_BASE, ICON_BUTTON_PRESSED, IconButtonVariant, Size, Tooltip, TooltipContent, button_classes, icon_button_size_class, tooltip::TooltipCtx},
};

/// A square button holding one glyph. Mirrors the TS `IconButton`.
///
/// `aria_label` is required because a glyph alone names nothing. `pressed`
/// makes it a toggle: left unset, `aria-pressed` stays off, so a plain action
/// is not announced as a toggle. With a `tooltip` the same `<button>` anchors
/// the tooltip, the way the TS one sits under `TooltipTrigger asChild`.
///
/// `size` takes the shared [`Size`] scale; the TS type narrows it to
/// `sm | md | touch`, and the kit adds no per-component size enums.
#[component]
pub fn IconButton(
	#[prop(into)] aria_label: String,
	#[prop(optional)] variant: IconButtonVariant,
	#[prop(optional)] size: Size,
	#[prop(optional, into)] pressed: MaybeProp<bool>,
	#[prop(optional, into)] tooltip: Option<ViewFn>,
	#[prop(optional, into)] class: String,
	#[prop(optional, into)] disabled: Signal<bool>,
	/// `button` by default, unlike [`Button`](super::Button): a toolbar glyph
	/// inside a form must not submit it.
	#[prop(default = String::from("button"), into)]
	r#type: String,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	let base = button_classes(&variant.button_variant(), size, true, None, "");
	let cls = Signal::derive(move || {
		cn!(
			&base,
			icon_button_size_class(size),
			ICON_BUTTON_BASE,
			if pressed.get() == Some(true) { ICON_BUTTON_PRESSED } else { "" },
			&class
		)
	});
	let face = move |tip: Option<TooltipCtx>| {
		let show = move |open: bool| {
			if let Some(t) = tip {
				t.open.set(open);
			}
		};
		view! {
			<button
				type=r#type
				class=cls
				data-slot="icon-button"
				data-variant=variant.as_ref().to_owned()
				aria-label=aria_label
				aria-pressed=move || pressed.get().map(|p| p.to_string())
				disabled=disabled
				on:click=move |e| {
					if let Some(h) = on_click {
						h.run(e);
					}
				}
				on:mouseenter=move |_| show(true)
				on:mouseleave=move |_| show(false)
				on:focus=move |_| show(true)
				on:blur=move |_| show(false)
			>
				{children()}
			</button>
		}
	};
	match tooltip {
		None => face(None).into_any(),
		Some(tip) => view! {
			<Tooltip>
				// Only its own tooltip: a plain IconButton placed inside some other
				// tooltip's subtree must not open that one.
				{face(use_context::<TooltipCtx>())} <TooltipContent>{tip.run()}</TooltipContent>
			</Tooltip>
		}
		.into_any(),
	}
}

/// The TS `UiIcon` frame: a 24-unit lucide glyph drawn in the current ink.
#[component]
pub(in crate::ui::uikit::leptos) fn Glyph(class: &'static str, children: Children) -> impl IntoView {
	view! {
		<svg
			class=class
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="1.75"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			{children()}
		</svg>
	}
}
