use leptos::{context::Provider, prelude::*};

use crate::{
	cn,
	uikit_leptos::{
		POPOVER_CONTENT,
		primitives::{Controllable, Side, use_controllable, use_stable_id},
	},
};

/// `InfoTip` — the ⓘ-beside-a-label *toggletip*, the mirror of the TS `InfoTip`:
/// a real `<button aria-expanded aria-controls>` toggling a `role="status"`
/// live-region bubble — never a `role="tooltip"`, so it stays correct for
/// keyboard and touch. Hover-with-intent (the TS 0.5s open-on-hover) is TS-only;
/// this one opens on click/tap/Enter/Space (see the README "Limitations").
#[component]
pub fn InfoTip(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	children: Children,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let id = Signal::stored(use_stable_id("info-tip"));
	let provided = InfoTipCtx { open: state, id };
	view! {
		<Provider value=provided>
			<span
				class="relative inline-flex"
				on:keydown=move |e| {
					if e.key() == "Escape" {
						state.set(false);
					}
				}
			>
				{children()}
			</span>
		</Provider>
	}
}

#[component]
pub fn InfoTipTrigger(#[prop(default = "More information".into(), into)] label: String, #[prop(optional, into)] class: String) -> impl IntoView {
	let ctx = expect_context::<InfoTipCtx>();
	let cls = cn!(
		"inline-flex size-4 shrink-0 cursor-help items-center justify-center rounded-full align-middle text-ink-soft outline-none transition-colors hover:text-ink focus-visible:ring-2 focus-visible:ring-ring data-[state=open]:text-accent-debug",
		class
	);
	view! {
		<button
			type="button"
			class=cls
			data-slot="info-tip-trigger"
			data-state=move || if ctx.open.get() { "open" } else { "closed" }
			aria-label=label
			aria-expanded=move || ctx.open.get().to_string()
			aria-controls=ctx.id
			on:click=move |_| ctx.open.set(!ctx.open.get_untracked())
		>
			// lucide `info`, inlined per the kit's no-lucide-dep icon convention.
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
				class="size-4"
				aria-hidden="true"
			>
				<circle cx="12" cy="12" r="10"></circle>
				<path d="M12 16v-4"></path>
				<path d="M12 8h.01"></path>
			</svg>
		</button>
	}
}

#[component]
pub fn InfoTipContent(#[prop(optional)] side: Side, #[prop(default = "center".into(), into)] align: String, #[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<InfoTipCtx>();
	let cls = cn!(POPOVER_CONTENT, "w-64 p-3 text-sm absolute top-full left-1/2 -translate-x-1/2 mt-1.5", class);
	move || {
		ctx.open.get().then(|| {
			view! {
				<div class="fixed inset-0 z-40" on:click=move |_| ctx.open.set(false)></div>
				<div
					id=ctx.id
					role="status"
					aria-live="polite"
					class=cls.clone()
					data-slot="info-tip-content"
					data-state="open"
					data-side=side.as_ref().to_owned()
					data-align=align.clone()
				>
					{children()}
				</div>
			}
		})
	}
}

#[derive(Clone, Copy)]
struct InfoTipCtx {
	open: Controllable<bool>,
	id: Signal<String>,
}
