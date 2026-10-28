use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{AVATAR, AVATAR_FALLBACK, AVATAR_IMAGE},
};

#[component]
pub fn Avatar(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(AVATAR, class);
	view! {
		<div class=cls data-slot="avatar">
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
					data-slot="avatar-image"
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
		<div class=cls data-slot="avatar-fallback">
			{children()}
		</div>
	}
}
