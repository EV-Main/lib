//! What a section shows a caller it is not for: an [`Empty`] with a lock, a title, why,
//! and an action slot. A guest gets the sign-in button there; a signed-in caller without
//! the permission gets no action, or the host's own (request access).

use ev_lib_classes::Icon;
use leptos::prelude::*;

use crate::ui::uikit::leptos::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle, primitives::icon_shapes};

#[component]
pub fn AuthWall(
	#[prop(into)] title: String,
	#[prop(into)] description: String,
	#[prop(optional, into)] class: String,
	/// The action row — typically the host's sign-in button.
	#[prop(optional)]
	children: Option<Children>,
) -> impl IntoView {
	view! {
		<Empty class=class>
			<EmptyHeader>
				<EmptyMedia variant=EmptyMediaVariant::Icon>{lock_icon()}</EmptyMedia>
				<EmptyTitle>{title}</EmptyTitle>
				<EmptyDescription>{description}</EmptyDescription>
			</EmptyHeader>
			<EmptyContent>{children.map(|c| c())}</EmptyContent>
		</Empty>
	}
}

fn lock_icon() -> impl IntoView {
	view! {
		<svg
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 24 24"
			fill="none"
			stroke="currentColor"
			stroke-width="2"
			stroke-linecap="round"
			stroke-linejoin="round"
			aria-hidden="true"
		>
			{icon_shapes(Icon::Lock)}
		</svg>
	}
}
