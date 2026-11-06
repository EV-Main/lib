use leptos::prelude::*;

use crate::{
	cn,
	ui::uikit::leptos::{AVATAR, AVATAR_FALLBACK, AVATAR_IMAGE, Slot},
};

#[component]
pub fn Avatar(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(AVATAR, class);
	view! {
		<div class=cls data-slot=Slot::Avatar.as_str()>
			{children()}
		</div>
	}
}

#[component]
pub fn AvatarImage(#[prop(optional, into)] class: String, #[prop(optional, into)] src: String, #[prop(optional, into)] alt: String) -> impl IntoView {
	let errored = RwSignal::new(false);
	let cls = cn!(AVATAR_IMAGE, class);
	move || {
		(!errored.get()).then(|| {
			view! {
				<img
					class=cls.clone()
					data-slot=Slot::AvatarImage.as_str()
					src=src.clone()
					alt=alt.clone()
					on:error=move |_| errored.set(true)
				/>
			}
		})
	}
}

#[component]
pub fn AvatarFallback(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(AVATAR_FALLBACK, class);
	view! {
		<div class=cls data-slot=Slot::AvatarFallback.as_str()>
			{children()}
		</div>
	}
}
