pub use ev_lib_classes::{AsideHeaderLabels, AsidePanelMode};
use ev_lib_classes::{Icon, copy};
use leptos::prelude::*;

pub use crate::ui::uikit::core::AsideCollapse;
use crate::{
	cn,
	ui::uikit::leptos::{
		ASIDE_BODY, ASIDE_FOOTER, ASIDE_HEADER_BACK_ICON, ASIDE_HEADER_BASE, ASIDE_HEADER_CLOSE_ICON, ASIDE_HEADER_TITLE, ASIDE_PANEL_DOCKED, ASIDE_PANEL_DRAWER, ASIDE_PANEL_NARROW_ONLY,
		ASIDE_PANEL_SHEET, ASIDE_PANEL_WIDE_ONLY, ASIDE_PANEL_WIDE_ONLY_WRAPPER, ASIDE_RAIL_BASE, ASIDE_RAIL_SPACER, AsidePanelSide, Drawer, DrawerContent, DrawerDirection, IconButton,
		Sheet, SheetContent, Size, Slot,
		icon_button::Glyph,
		primitives::{Side, icon_shapes},
	},
};

/// The 44 px title row of an aside. Mirrors the TS `AsideHeader`.
///
/// The back and close buttons render only when their handler is set, so a
/// button that does nothing is never shown.
#[component]
pub fn AsideHeader(
	#[prop(into)] title: ViewFn,
	/// Shows the back button first.
	#[prop(optional)]
	on_back: Option<Callback<()>>,
	/// Shows the close button last.
	#[prop(optional)]
	on_close: Option<Callback<()>>,
	/// `IconButton`s between the title and the close button: pin, more.
	#[prop(optional, into)]
	actions: Option<ViewFn>,
	#[prop(optional)] labels: AsideHeaderLabels,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let back_label = labels.back;
	let close_label = labels.close;
	let cls = cn!(ASIDE_HEADER_BASE, class);
	view! {
		<div data-slot=Slot::AsideHeader.as_str() class=cls>
			{on_back
				.map(|back| {
					view! {
						<IconButton
							aria_label=back_label
							size=Size::Sm
							on_click=Callback::new(move |_| back.run(()))
						>
							<Glyph class=ASIDE_HEADER_BACK_ICON>{icon_shapes(Icon::ChevronRight)}</Glyph>
						</IconButton>
					}
				})}
			<h2 class=ASIDE_HEADER_TITLE>{title.run()}</h2>
			{actions.map(|a| a.run())}
			{on_close
				.map(|close| {
					view! {
						<IconButton
							aria_label=close_label
							size=Size::Sm
							on_click=Callback::new(move |_| close.run(()))
						>
							<Glyph class=ASIDE_HEADER_CLOSE_ICON>{icon_shapes(Icon::X)}</Glyph>
						</IconButton>
					}
				})}
		</div>
	}
}

/// One button of an [`AsideRail`]. Mirrors the TS `AsideRailItem`.
#[derive(Clone)]
pub struct AsideRailItem {
	pub id: String,
	/// Names the button and fills its tooltip.
	pub label: String,
	pub icon: ViewFn,
}

/// The 44 px strip of icon buttons that opens and closes the asides. Mirrors
/// the TS `AsideRail`.
///
/// The button of the open aside is pressed. Pressing it again asks for `None`,
/// so the rail closes what it opened.
#[component]
pub fn AsideRail(
	items: Vec<AsideRailItem>,
	/// Pinned to the bottom, e.g. settings.
	#[prop(optional)]
	end_items: Vec<AsideRailItem>,
	/// The open aside's id; unset when all are closed.
	#[prop(optional, into)]
	value: MaybeProp<String>,
	on_value_change: Callback<Option<String>>,
	#[prop(default = copy::ASIDE_RAIL.to_owned(), into)] aria_label: String,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let cls = cn!(ASIDE_RAIL_BASE, class);
	let buttons = move |items: Vec<AsideRailItem>| items.into_iter().map(|item| view! { <AsideRailButton item value on_value_change /> }).collect_view();
	view! {
		<div role="group" aria-label=aria_label data-slot=Slot::AsideRail.as_str() class=cls>
			{buttons(items)}
			<span class=ASIDE_RAIL_SPACER></span>
			{buttons(end_items)}
		</div>
	}
}

/// A side panel. Mirrors the TS `AsidePanel`.
///
/// `Docked` sits in the layout and takes room from the content; `Overlay`
/// floats over a scrim on a [`Sheet`]. Below `md` both use a bottom
/// [`Drawer`]. The edge that faces the content is the only border. A closed
/// panel renders nothing.
///
/// The TS port picks the form with `matchMedia`. This port has no
/// `matchMedia`: it renders the wide form and the drawer together, and the
/// `md` breakpoint classes show one of them. So with [`AsideCollapse::Md`]
/// an open panel mounts `children` twice, and a child with state keeps it
/// only for the form it is in. [`AsideCollapse::Never`] mounts them once.
///
/// The overlays keep the kit's limits: rendered inline with `position: fixed`,
/// no portal and no focus trap. A docked panel is dismissable by Escape and
/// the scrim only on its drawer, below `md`.
#[component]
pub fn AsidePanel(
	/// Names the panel.
	#[prop(into)]
	aria_label: String,
	#[prop(optional)] mode: AsidePanelMode,
	/// Which edge of the screen it hugs.
	#[prop(optional)]
	side: AsidePanelSide,
	#[prop(default = true.into(), into)] open: Signal<bool>,
	/// Escape and the scrim call it with `false`.
	#[prop(optional)]
	on_open_change: Option<Callback<bool>>,
	/// An [`AsideHeader`].
	#[prop(optional, into)]
	header: Option<ViewFn>,
	/// Sticky under the body: actions.
	#[prop(optional, into)]
	footer: Option<ViewFn>,
	#[prop(optional)] collapse_below: AsideCollapse,
	/// Docked 380 (260 for a left navigation), overlay 420: a width class
	/// overrides it.
	#[prop(optional, into)]
	class: String,
	children: ChildrenFn,
) -> impl IntoView {
	let change = Callback::new(move |next: bool| {
		if let Some(h) = on_open_change {
			h.run(next);
		}
	});
	let body = move || {
		view! {
			{header.as_ref().map(ViewFn::run)}
			<div data-slot=Slot::AsideBody.as_str() class=ASIDE_BODY>
				{children()}
			</div>
			{footer
				.as_ref()
				.map(|f| {
					view! {
						<div data-slot=Slot::AsideFooter.as_str() class=ASIDE_FOOTER>
							{f.run()}
						</div>
					}
				})}
		}
	};
	let docked = {
		let (aria_label, class, body) = (aria_label.clone(), class.clone(), body.clone());
		move |visibility: &'static str| {
			view! {
				<aside
					aria-label=aria_label.clone()
					data-slot=Slot::AsidePanel.as_str()
					data-mode="docked"
					data-side=side.as_ref().to_owned()
					class=cn!(ASIDE_PANEL_DOCKED, side.as_class(), &class, visibility)
				>
					{body()}
				</aside>
			}
		}
	};
	let sheet = {
		let (aria_label, body) = (aria_label.clone(), body.clone());
		let sheet_side = match side {
			AsidePanelSide::Left => Side::Left,
			AsidePanelSide::Right => Side::Right,
		};
		let cls = cn!(ASIDE_PANEL_SHEET, &class);
		move || {
			let body = body.clone();
			view! {
				<Sheet open=open on_open_change=change>
					<SheetContent side=sheet_side aria_label=aria_label.clone() class=cls.clone()>
						{body()}
					</SheetContent>
				</Sheet>
			}
		}
	};
	// Mounted even while closed, so a drawer that closes plays its exit.
	let drawer = move || {
		let body = body.clone();
		view! {
			<div data-slot=Slot::AsidePanelNarrow.as_str() class=ASIDE_PANEL_NARROW_ONLY>
				<Drawer open=open on_open_change=change direction=DrawerDirection::Bottom>
					<DrawerContent aria_label=aria_label.clone() class=ASIDE_PANEL_DRAWER>
						{body()}
					</DrawerContent>
				</Drawer>
			</div>
		}
	};
	match (mode, collapse_below) {
		(AsidePanelMode::Docked, AsideCollapse::Never) => (move || open.get().then(|| docked(""))).into_any(),
		(AsidePanelMode::Docked, AsideCollapse::Md) => view! {
			{move || open.get().then(|| docked(ASIDE_PANEL_WIDE_ONLY))}
			{drawer()}
		}
		.into_any(),
		(AsidePanelMode::Overlay, AsideCollapse::Never) => sheet().into_any(),
		(AsidePanelMode::Overlay, AsideCollapse::Md) => view! {
			<div data-slot=Slot::AsidePanelWide.as_str() class=ASIDE_PANEL_WIDE_ONLY_WRAPPER>
				{sheet()}
			</div>
			{drawer()}
		}
		.into_any(),
	}
}

#[component]
fn AsideRailButton(item: AsideRailItem, value: MaybeProp<String>, on_value_change: Callback<Option<String>>) -> impl IntoView {
	let AsideRailItem { id, label, icon } = item;
	let key = StoredValue::new(id);
	let open = Signal::derive(move || key.with_value(|id| value.get().as_deref() == Some(id.as_str())));
	view! {
		<IconButton
			aria_label=label.clone()
			tooltip=move || label.clone()
			pressed=open
			on_click=Callback::new(move |_| {
				on_value_change.run(if open.get_untracked() { None } else { Some(key.get_value()) })
			})
		>
			{icon.run()}
		</IconButton>
	}
}
