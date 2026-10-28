use leptos::prelude::*;

use crate::{
	cn,
	uikit::{ACCORDION_CHEVRON, toggled},
	uikit_leptos::{
		ACCORDION_CONTENT, ACCORDION_CONTENT_INNER, ACCORDION_HEADER, ACCORDION_ITEM, ACCORDION_TRIGGER, AccordionType,
		primitives::{Controllable, use_controllable},
	},
};

#[component]
pub fn Accordion(
	#[prop(optional)] r#type: AccordionType,
	#[prop(optional)] collapsible: bool,
	#[prop(optional, into)] value: MaybeProp<Vec<String>>,
	#[prop(optional)] default_value: Vec<String>,
	#[prop(optional)] on_value_change: Option<Callback<Vec<String>>>,
	#[prop(optional, into)] class: String,
	children: Children,
) -> impl IntoView {
	let open = use_controllable(value, default_value, on_value_change);
	provide_context(AccordionCtx { open, kind: r#type, collapsible });
	view! {
		<div class=class data-slot="accordion">
			{children()}
		</div>
	}
}

#[component]
pub fn AccordionItem(#[prop(into)] value: String, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	provide_context(ItemCtx(StoredValue::new(value)));
	let cls = cn!(ACCORDION_ITEM, class);
	view! {
		<div class=cls data-slot="accordion-item">
			{children()}
		</div>
	}
}

#[component]
pub fn AccordionTrigger(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let ctx = expect_context::<AccordionCtx>();
	let item = expect_context::<ItemCtx>();
	let open = move || ctx.is_open(item);
	let cls = cn!(ACCORDION_TRIGGER, class);
	view! {
		<h3 class=ACCORDION_HEADER data-slot="accordion-header">
			<button
				type="button"
				class=cls
				data-slot="accordion-trigger"
				data-state=move || if open() { "open" } else { "closed" }
				aria-expanded=move || open().to_string()
				on:click=move |_| ctx.toggle(item)
			>
				{children()}
				<svg
					xmlns="http://www.w3.org/2000/svg"
					width="24"
					height="24"
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					stroke-linecap="round"
					stroke-linejoin="round"
					class=ACCORDION_CHEVRON
				>
					<path d="m6 9 6 6 6-6"></path>
				</svg>
			</button>
		</h3>
	}
}

#[component]
pub fn AccordionContent(#[prop(optional, into)] class: String, children: ChildrenFn) -> impl IntoView {
	let ctx = expect_context::<AccordionCtx>();
	let item = expect_context::<ItemCtx>();
	let inner = cn!(ACCORDION_CONTENT_INNER, class);
	move || {
		ctx.is_open(item).then(|| {
			view! {
				<div class=ACCORDION_CONTENT data-slot="accordion-content" data-state="open">
					<div class=inner.clone()>{children()}</div>
				</div>
			}
		})
	}
}

#[derive(Clone, Copy)]
struct AccordionCtx {
	open: Controllable<Vec<String>>,
	kind: AccordionType,
	collapsible: bool,
}

impl AccordionCtx {
	fn is_open(&self, item: ItemCtx) -> bool {
		item.0.with_value(|value| self.open.get().iter().any(|v| v == value))
	}

	fn toggle(&self, item: ItemCtx) {
		let next = item
			.0
			.with_value(|value| toggled(&self.open.get_untracked(), value, self.kind == AccordionType::Multiple, self.collapsible));
		self.open.set(next);
	}
}

#[derive(Clone, Copy)]
struct ItemCtx(StoredValue<String>);
