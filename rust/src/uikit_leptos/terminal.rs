//! The trading terminal — an order book over an investment product's shares,
//! laid out like a spot exchange: ticker, then chart | book | order form, then
//! the trader's own orders. Every pane is a slot; the chart host is empty on
//! purpose (the plotting engine is the consumer's), the open-orders table is
//! [`Table`](super::Table), the tabs over it are `Tabs`.

use leptos::prelude::*;

use crate::{
	cn,
	uikit::depth_percent,
	uikit_leptos::{
		BookSide, OPEN_ORDERS_EMPTY, ORDER_BOOK, ORDER_BOOK_DEPTH, ORDER_BOOK_HEAD, ORDER_BOOK_PRICE, ORDER_BOOK_ROW, ORDER_BOOK_SPREAD, ORDER_FORM, ORDER_FORM_LABEL, ORDER_FORM_ROW,
		ORDER_FORM_SUBMIT_BASE, ORDER_FORM_VALUE, OrderSide, TERMINAL_CHART, TERMINAL_PANE, TERMINAL_PANE_BODY, TERMINAL_PANE_HEADER, TERMINAL_ROOT, TERMINAL_TICKER, TERMINAL_TICKER_LABEL,
		TERMINAL_TICKER_STAT, TERMINAL_TICKER_VALUE, TRADES_TAPE_ROW, TerminalArea, book_depth_class,
	},
};

/// The frame: a phone stacks the panes in DOM order, `lg` and up places them
/// by [`TerminalArea`] on a viewport-bound grid.
#[component]
pub fn Terminal(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TERMINAL_ROOT, class);
	view! {
		<div class=cls data-slot="terminal">
			{children()}
		</div>
	}
}

/// One cell of the frame. `area` is where it sits on the desktop grid;
/// [`TerminalPaneHeader`] and [`TerminalPaneBody`] go inside.
#[component]
pub fn TerminalPane(area: TerminalArea, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TERMINAL_PANE, area.as_class(), class);
	view! {
		<section class=cls data-slot="terminal-pane" data-area=area.as_ref().to_owned()>
			{children()}
		</section>
	}
}

#[component]
pub fn TerminalPaneHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TERMINAL_PANE_HEADER, class);
	view! {
		<header class=cls data-slot="terminal-pane-header">
			{children()}
		</header>
	}
}

/// The scrolling part of a pane.
#[component]
pub fn TerminalPaneBody(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TERMINAL_PANE_BODY, class);
	view! {
		<div class=cls data-slot="terminal-pane-body">
			{children()}
		</div>
	}
}

/// The sized host a plotting engine mounts into. Renders nothing itself; a
/// consumer mounts its chart on the element (`id`) or fills it with children.
#[component]
pub fn TerminalChart(#[prop(optional, into)] id: Option<String>, #[prop(optional, into)] class: String, #[prop(optional)] children: Option<Children>) -> impl IntoView {
	let cls = cn!(TERMINAL_CHART, class);
	view! {
		<div id=id class=cls data-slot="terminal-chart">
			{children.map(|c| c())}
		</div>
	}
}

/// The strip across the top; holds [`TickerStat`]s. Already placed in the
/// [`TerminalArea::Ticker`] slot.
#[component]
pub fn TerminalTicker(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TERMINAL_TICKER, TerminalArea::Ticker.as_class(), class);
	view! {
		<div class=cls data-slot="terminal-ticker" data-area=TerminalArea::Ticker.as_ref().to_owned()>
			{children()}
		</div>
	}
}

/// A labelled figure in the ticker. `class` lands on the figure, so a change
/// figure passes its valence colour there.
#[component]
pub fn TickerStat(#[prop(into)] label: String, #[prop(into)] value: Signal<String>, #[prop(optional, into)] class: String) -> impl IntoView {
	let value_cls = cn!(TERMINAL_TICKER_VALUE, class);
	view! {
		<div class=TERMINAL_TICKER_STAT data-slot="ticker-stat">
			<span class=TERMINAL_TICKER_LABEL>{label}</span>
			<span class=value_cls>{value}</span>
		</div>
	}
}

/// The book: [`OrderBookHead`], then ask rows, [`OrderBookSpread`], bid rows.
#[component]
pub fn OrderBook(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ORDER_BOOK, class);
	view! {
		<div class=cls data-slot="order-book">
			{children()}
		</div>
	}
}

/// The three column captions — price, size, total — in the consumer's words.
#[component]
pub fn OrderBookHead(#[prop(into)] price: String, #[prop(into)] size: String, #[prop(into)] total: String, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(ORDER_BOOK_HEAD, class);
	view! {
		<div class=cls data-slot="order-book-head">
			<span>{price}</span>
			<span>{size}</span>
			<span>{total}</span>
		</div>
	}
}

/// One price level. `depth` is this level's cumulative size as a fraction of
/// the deepest level shown (`0.0..=1.0`); it sizes the bar behind the figures.
#[component]
pub fn OrderBookRow(
	side: BookSide,
	#[prop(into)] price: Signal<String>,
	#[prop(into)] size: Signal<String>,
	#[prop(into)] total: Signal<String>,
	#[prop(optional, into)] depth: Signal<f64>,
	#[prop(optional, into)] class: String,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
) -> impl IntoView {
	let cls = cn!(ORDER_BOOK_ROW, class);
	let price_cls = cn!(ORDER_BOOK_PRICE, side.as_class());
	let depth_cls = cn!(ORDER_BOOK_DEPTH, book_depth_class(side));
	view! {
		<div
			class=cls
			data-slot="order-book-row"
			data-side=side.as_ref().to_owned()
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			<span
				class=depth_cls
				style=move || format!("width: {}%", depth_percent(depth.get()))
				aria-hidden="true"
			></span>
			<span class=price_cls>{price}</span>
			<span>{size}</span>
			<span>{total}</span>
		</div>
	}
}

/// The line between asks and bids: the consumer passes the mid and the spread
/// as it wants them formatted.
#[component]
pub fn OrderBookSpread(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(ORDER_BOOK_SPREAD, class);
	view! {
		<div class=cls data-slot="order-book-spread">
			{children()}
		</div>
	}
}

/// One print in the tape: price coloured by the aggressor's side, then size
/// and time.
#[component]
pub fn TradesTapeRow(side: BookSide, #[prop(into)] price: String, #[prop(into)] size: String, #[prop(into)] time: String, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(TRADES_TAPE_ROW, class);
	let price_cls = cn!(ORDER_BOOK_PRICE, side.as_class());
	view! {
		<div class=cls data-slot="trades-tape-row" data-side=side.as_ref().to_owned()>
			<span class=price_cls>{price}</span>
			<span>{size}</span>
			<span>{time}</span>
		</div>
	}
}

/// The order form. Owns no state: the consumer wires the inputs and handles
/// `on_submit`; the default is prevented here so a submit never navigates.
#[component]
pub fn OrderForm(#[prop(optional, into)] class: String, #[prop(optional)] on_submit: Option<Callback<leptos::ev::SubmitEvent>>, children: Children) -> impl IntoView {
	let cls = cn!(ORDER_FORM, class);
	view! {
		<form
			class=cls
			data-slot="order-form"
			on:submit=move |e| {
				e.prevent_default();
				if let Some(h) = on_submit {
					h.run(e);
				}
			}
		>
			{children()}
		</form>
	}
}

/// A label / figure line in the order summary — cost, fee, what is available.
#[component]
pub fn OrderFormRow(#[prop(into)] label: String, #[prop(into)] value: Signal<String>, #[prop(optional, into)] class: String) -> impl IntoView {
	let cls = cn!(ORDER_FORM_ROW, class);
	view! {
		<div class=cls data-slot="order-form-row">
			<span class=ORDER_FORM_LABEL>{label}</span>
			<span class=ORDER_FORM_VALUE>{value}</span>
		</div>
	}
}

/// The submit, filled in the colour of the intent.
#[component]
pub fn OrderFormSubmit(
	#[prop(optional)] side: OrderSide,
	#[prop(optional, into)] disabled: Signal<bool>,
	#[prop(optional, into)] class: String,
	#[prop(optional)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
	children: Children,
) -> impl IntoView {
	let cls = cn!(ORDER_FORM_SUBMIT_BASE, side.as_class(), class);
	view! {
		<button
			class=cls
			data-slot="order-form-submit"
			data-side=side.as_ref().to_owned()
			type="submit"
			disabled=disabled
			on:click=move |e| {
				if let Some(h) = on_click {
					h.run(e);
				}
			}
		>
			{children()}
		</button>
	}
}

/// The open-orders pane with nothing in it.
#[component]
pub fn OpenOrdersEmpty(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(OPEN_ORDERS_EMPTY, class);
	view! {
		<div class=cls data-slot="open-orders-empty">
			{children()}
		</div>
	}
}
