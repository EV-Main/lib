use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{
		ITEM_ACTIONS, ITEM_BASE, ITEM_CONTENT, ITEM_DESCRIPTION, ITEM_FOOTER, ITEM_GROUP, ITEM_HEADER, ITEM_MEDIA_BASE, ITEM_SEPARATOR, ITEM_TITLE, ItemMediaVariant, ItemSize, ItemVariant,
		Orientation, Separator, Slot,
	},
};

#[component]
pub fn ItemGroup(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_GROUP, class);
	view! {
		<div role="list" class=cls data-slot=Slot::ItemGroup.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn ItemSeparator(#[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(ITEM_SEPARATOR, class);
	view! { <Separator orientation=Orientation::Horizontal class=cls /> }
}

#[component]
pub fn Item(#[prop(optional)] variant: ItemVariant, #[prop(optional)] size: ItemSize, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_BASE, variant.as_class(), size.as_class(), class);
	view! {
		<div
			class=cls
			data-slot=Slot::Item.as_str()
			data-variant=variant.as_ref().to_owned()
			data-size=size.as_ref().to_owned()
		>
			{children()}
		</div>
	}
}

#[component]
pub fn ItemMedia(#[prop(optional)] variant: ItemMediaVariant, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_MEDIA_BASE, variant.as_class(), class);
	view! {
		<div class=cls data-slot=Slot::ItemMedia.as_str() data-variant=variant.as_ref().to_owned()>
			{children()}
		</div>
	}
}

#[component]
pub fn ItemContent(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_CONTENT, class);
	view! {
		<div class=cls data-slot=Slot::ItemContent.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn ItemTitle(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_TITLE, class);
	view! {
		<div class=cls data-slot=Slot::ItemTitle.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn ItemDescription(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_DESCRIPTION, class);
	view! {
		<p class=cls data-slot=Slot::ItemDescription.as_str()>
			{children()}
		</p>
	}
}

#[component]
pub fn ItemActions(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_ACTIONS, class);
	view! {
		<div class=cls data-slot=Slot::ItemActions.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn ItemHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_HEADER, class);
	view! {
		<div class=cls data-slot=Slot::ItemHeader.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn ItemFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ITEM_FOOTER, class);
	view! {
		<div class=cls data-slot=Slot::ItemFooter.as_str()>
			{children()}
		</div>
	}
}
