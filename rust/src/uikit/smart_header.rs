use dioxus::prelude::*;

use crate::{
	cn,
	uikit::{SMART_HEADER_BAR, SMART_HEADER_BASE, SMART_HEADER_CONTEXT, SMART_HEADER_TOOLBAR, TopBar},
};

/// Which rows a [`SmartHeader`] shows around its bar. Mirrors the TS
/// `SmartHeaderState`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SmartHeaderState {
	/// The bar, the toolbar and the context line.
	#[default]
	Default,
	/// The toolbar folds into the bar as `collapsed`.
	Collapsed,
	/// A banner row above the bar.
	Banner,
	/// The bar with the `palette` under it.
	Search,
}

/// A page header that adapts to scroll and alerts. Mirrors the TS
/// `SmartHeader`.
///
/// The bar is the same in every state, so switching never moves the brand or
/// the account: only the rows around it come and go. Layout only: the caller
/// owns the state and what changes it (a scroll observer, a feed status, the
/// search button), so the kit needs no scroll listener.
#[component]
pub fn SmartHeader(
	#[props(default)] state: SmartHeaderState,
	/// Brand, workspace tabs: the left side of the bar.
	start: Option<Element>,
	/// Icon actions, balance, bell, account menu: the right side of the bar.
	end: Option<Element>,
	/// The folded toolbar after `start`; shown in `Collapsed` only.
	collapsed: Option<Element>,
	/// A `SystemBanner`; shown in `Banner` only.
	banner: Option<Element>,
	/// The second row; shown in `Default` only.
	toolbar: Option<Element>,
	/// The slim line on the page fill under the toolbar; shown in `Default` only.
	context: Option<Element>,
	/// A `CommandPalette` or another panel that drops from the bar; shown in
	/// `Search` only.
	palette: Option<Element>,
	#[props(default)] class: String,
) -> Element {
	let cls = cn!(SMART_HEADER_BASE, class);
	let banner = banner.filter(|_| state == SmartHeaderState::Banner);
	let collapsed = collapsed.filter(|_| state == SmartHeaderState::Collapsed);
	let toolbar = toolbar.filter(|_| state == SmartHeaderState::Default);
	let context = context.filter(|_| state == SmartHeaderState::Default);
	let palette = palette.filter(|_| state == SmartHeaderState::Search);
	rsx! {
		div { class: cls, "data-slot": "smart-header", "data-state": state.as_ref(),
			{banner}
			TopBar {
				class: SMART_HEADER_BAR,
				start: rsx! {
					{start}
					{collapsed}
				},
				end,
			}
			if let Some(toolbar) = toolbar {
				div { class: SMART_HEADER_TOOLBAR, "data-slot": "smart-header-toolbar", {toolbar} }
			}
			if let Some(context) = context {
				div { class: SMART_HEADER_CONTEXT, "data-slot": "smart-header-context", {context} }
			}
			if let Some(palette) = palette {
				div { "data-slot": "smart-header-palette", {palette} }
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::uikit::test_util::render;

	const ALL: [&str; 5] = ["Панель", "Контекст", "Свёрнуто", "Задержка котировок", "Палитра"];

	fn view(state: SmartHeaderState) -> Element {
		rsx! {
			SmartHeader {
				state,
				start: rsx! { span { "Бренд" } },
				end: rsx! { span { "Аккаунт" } },
				collapsed: rsx! { span { "Свёрнуто" } },
				banner: rsx! { div { role: "status", "Задержка котировок" } },
				toolbar: rsx! { span { "Панель" } },
				context: rsx! { span { "Контекст" } },
				palette: rsx! { div { role: "dialog", "Палитра" } },
			}
		}
	}

	fn shown(html: &str) -> Vec<&'static str> {
		ALL.into_iter().filter(|n| html.contains(n)).collect()
	}

	fn default_view() -> Element {
		view(SmartHeaderState::Default)
	}
	fn collapsed_view() -> Element {
		view(SmartHeaderState::Collapsed)
	}
	fn banner_view() -> Element {
		view(SmartHeaderState::Banner)
	}
	fn search_view() -> Element {
		view(SmartHeaderState::Search)
	}

	#[test]
	fn keeps_the_bar_in_every_state() {
		for app in [default_view, collapsed_view, banner_view, search_view] {
			let html = render(app);
			assert!(html.contains("Бренд") && html.contains("Аккаунт"), "{html}");
			assert_eq!(html.matches("data-slot=\"top-bar\"").count(), 1, "{html}");
		}
	}

	#[test]
	fn default_shows_the_toolbar_and_the_context_line_only() {
		let html = render(default_view);
		assert_eq!(shown(&html), ["Панель", "Контекст"], "{html}");
		assert!(html.contains("data-state=\"default\""), "{html}");
		assert!(html.find("data-slot=\"top-bar\"") < html.find("data-slot=\"smart-header-toolbar\""), "{html}");
		assert!(html.find("data-slot=\"smart-header-toolbar\"") < html.find("data-slot=\"smart-header-context\""), "{html}");
	}

	#[test]
	fn collapsed_folds_the_toolbar_into_the_bar() {
		let html = render(collapsed_view);
		assert_eq!(shown(&html), ["Свёрнуто"], "{html}");
		let start = html.find("data-slot=\"top-bar-start\"").expect("start");
		let end = html.find("data-slot=\"top-bar-end\"").expect("end");
		let brand = html.find("Бренд").expect("brand");
		let folded = html.find("Свёрнуто").expect("folded");
		assert!(start < brand && brand < folded && folded < end, "the fold follows the brand on the start side: {html}");
	}

	#[test]
	fn banner_puts_the_banner_above_the_bar() {
		let html = render(banner_view);
		assert_eq!(shown(&html), ["Задержка котировок"], "{html}");
		let root_open = html.find('>').expect("root tag") + 1;
		assert!(html[root_open..].starts_with("<div role=\"status\">Задержка котировок"), "the banner is the first row: {html}");
	}

	#[test]
	fn search_drops_the_palette_under_the_bar() {
		let html = render(search_view);
		assert_eq!(shown(&html), ["Палитра"], "{html}");
		assert!(html.contains("data-state=\"search\""), "{html}");
		assert!(html.find("data-slot=\"top-bar\"") < html.find("data-slot=\"smart-header-palette\""), "{html}");
	}

	#[test]
	fn skips_a_row_it_was_not_given() {
		fn app() -> Element {
			rsx! { SmartHeader { start: rsx! { "Бренд" }, toolbar: rsx! { "Панель" } } }
		}
		let html = render(app);
		assert!(html.contains("smart-header-toolbar"), "{html}");
		assert!(!html.contains("smart-header-context"), "{html}");
	}

	#[test]
	fn the_bar_is_the_slim_card_one() {
		fn app() -> Element {
			rsx! { SmartHeader {} }
		}
		let html = render(app);
		let bar = html.split("data-slot=\"top-bar\"").next().and_then(|before| before.rsplit("<header").next()).expect("bar tag");
		assert!(
			bar.contains("h-10") && bar.contains("bg-card") && bar.contains("px-3") && bar.contains("border-border-subtle"),
			"{bar}"
		);
		assert!(
			!bar.contains("h-14") && !bar.contains("bg-background") && !bar.contains("px-8") && !bar.contains("border-border "),
			"{bar}"
		);
	}
}
