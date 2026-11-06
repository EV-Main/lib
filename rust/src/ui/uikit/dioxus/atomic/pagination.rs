use dioxus::prelude::*;
use ev_lib_classes::{Icon, PAGINATION_CONTENT, PAGINATION_ELLIPSIS, PAGINATION_LINK_LABEL, PAGINATION_NEXT_LINK, PAGINATION_PREVIOUS_LINK, PAGINATION_ROOT, copy};

use crate::{
	cn,
	ui::uikit::dioxus::{ButtonVariant, Size, Slot, button::button_classes, primitives::icon_shapes},
};

#[component]
pub fn Pagination(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(PAGINATION_ROOT, class);
	rsx! {
		nav {
			role: "navigation",
			"aria-label": copy::PAGINATION,
			"data-slot": Slot::Pagination.as_str(),
			class: cls,
			{children}
		}
	}
}

#[component]
pub fn PaginationContent(#[props(default)] class: String, children: Element) -> Element {
	let cls = cn!(PAGINATION_CONTENT, class);
	rsx! {
		ul { "data-slot": Slot::PaginationContent.as_str(), class: cls, {children} }
	}
}

#[component]
pub fn PaginationItem(children: Element) -> Element {
	rsx! {
		li { "data-slot": Slot::PaginationItem.as_str(), {children} }
	}
}

/// `href` is optional but wanted: an `<a>` without one is neither focusable nor
/// announced as a link, so a link given neither `href` nor `onclick` is inert.
#[component]
pub fn PaginationLink(
	#[props(default)] is_active: bool,
	#[props(default)] size: Size,
	#[props(default = true)] icon: bool,
	#[props(default)] class: String,
	href: Option<String>,
	onclick: Option<EventHandler<MouseEvent>>,
	aria_label: Option<String>,
	children: Element,
) -> Element {
	let variant = if is_active { ButtonVariant::Outline } else { ButtonVariant::Ghost };
	let cls = button_classes(&variant, size, icon, None, &class);
	rsx! {
		a {
			"aria-current": if is_active { "page" },
			"aria-label": aria_label,
			"data-slot": Slot::PaginationLink.as_str(),
			"data-active": is_active,
			class: cls,
			href,
			onclick: move |e| { if let Some(h) = onclick { h.call(e); } },
			{children}
		}
	}
}

#[component]
pub fn PaginationPrevious(#[props(default)] class: String, href: Option<String>, onclick: Option<EventHandler<MouseEvent>>) -> Element {
	let cls = cn!(PAGINATION_PREVIOUS_LINK, class);
	rsx! {
		PaginationLink {
			is_active: false,
			icon: false,
			aria_label: copy::PAGINATION_PREVIOUS_PAGE,
			class: cls,
			href,
			onclick: move |e| { if let Some(h) = onclick { h.call(e); } },
			svg {
				view_box: "0 0 24 24",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "2",
				{icon_shapes(Icon::ChevronLeft)}
			}
			span { class: PAGINATION_LINK_LABEL, {copy::PAGINATION_PREVIOUS} }
		}
	}
}

#[component]
pub fn PaginationNext(#[props(default)] class: String, href: Option<String>, onclick: Option<EventHandler<MouseEvent>>) -> Element {
	let cls = cn!(PAGINATION_NEXT_LINK, class);
	rsx! {
		PaginationLink {
			is_active: false,
			icon: false,
			aria_label: copy::PAGINATION_NEXT_PAGE,
			class: cls,
			href,
			onclick: move |e| { if let Some(h) = onclick { h.call(e); } },
			span { class: PAGINATION_LINK_LABEL, {copy::PAGINATION_NEXT} }
			svg {
				view_box: "0 0 24 24",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "2",
				{icon_shapes(Icon::ChevronRight)}
			}
		}
	}
}

#[component]
pub fn PaginationEllipsis(#[props(default)] class: String) -> Element {
	let cls = cn!(PAGINATION_ELLIPSIS, class);
	rsx! {
		span {
			"aria-hidden": "true",
			"data-slot": Slot::PaginationEllipsis.as_str(),
			class: cls,
			svg {
				class: "size-4",
				view_box: "0 0 24 24",
				fill: "none",
				stroke: "currentColor",
				stroke_width: "2",
				{icon_shapes(Icon::Ellipsis)}
			}
			span { class: "sr-only", {copy::PAGINATION_MORE} }
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ui::uikit::dioxus::test_util::render;

	#[test]
	fn pagination_wraps_a_nav() {
		fn app() -> Element {
			rsx! {
				Pagination {
					PaginationContent {
						PaginationItem {
							PaginationLink { "1" }
						}
					}
				}
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"pagination\""), "{html}");
		assert!(html.contains("data-slot=\"pagination-link\""));
	}

	#[test]
	fn active_link_uses_outline_variant() {
		fn app() -> Element {
			rsx! {
				PaginationLink { is_active: true, "2" }
			}
		}
		let html = render(app);
		assert!(html.contains("aria-current=\"page\""), "{html}");
		assert!(html.contains("border"), "outline variant adds border: {html}");
	}

	#[test]
	fn previous_renders_chevron_and_label() {
		fn app() -> Element {
			rsx! { PaginationPrevious {} }
		}
		let html = render(app);
		assert!(html.contains("m15 18-6-6 6-6"), "{html}");
		assert!(html.contains("Previous"));
	}

	#[test]
	fn previous_and_next_are_named_at_every_width() {
		fn app() -> Element {
			rsx! {
				PaginationPrevious {}
				PaginationNext {}
			}
		}
		let html = render(app);
		assert!(html.contains("aria-label=\"Go to previous page\""), "{html}");
		assert!(html.contains("aria-label=\"Go to next page\""), "{html}");
	}

	#[test]
	fn link_aria_label_is_optional() {
		fn named() -> Element {
			rsx! {
				PaginationLink { aria_label: "Page 2", "2" }
			}
		}
		assert!(render(named).contains("aria-label=\"Page 2\""), "{}", render(named));
		fn unnamed() -> Element {
			rsx! {
				PaginationLink { "2" }
			}
		}
		assert!(!render(unnamed).contains("aria-label"), "{}", render(unnamed));
	}

	#[test]
	fn link_href_reaches_the_anchor() {
		fn app() -> Element {
			rsx! {
				PaginationLink { href: "?page=2", "2" }
			}
		}
		let html = render(app);
		assert!(html.contains("href=\"?page=2\""), "{html}");
	}

	#[test]
	fn previous_and_next_forward_href() {
		fn app() -> Element {
			rsx! {
				PaginationPrevious { href: "?page=1" }
				PaginationNext { href: "?page=3" }
			}
		}
		let html = render(app);
		assert!(html.contains("href=\"?page=1\""), "{html}");
		assert!(html.contains("href=\"?page=3\""), "{html}");
	}

	#[test]
	fn link_without_href_emits_no_empty_href() {
		fn app() -> Element {
			rsx! {
				PaginationLink { "2" }
			}
		}
		let html = render(app);
		assert!(!html.contains("href"), "an empty href would point the link at the current page: {html}");
	}

	#[test]
	fn ellipsis_is_aria_hidden() {
		fn app() -> Element {
			rsx! { PaginationEllipsis {} }
		}
		let html = render(app);
		assert!(html.contains("data-slot=\"pagination-ellipsis\""), "{html}");
		assert!(html.contains("More pages"));
	}
}
