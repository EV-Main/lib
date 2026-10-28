use leptos::prelude::*;

use crate::{
	cn,
	uikit_leptos::{TABLE, TABLE_BODY, TABLE_CAPTION, TABLE_CARD, TABLE_CELL, TABLE_CONTAINER, TABLE_FOOTER, TABLE_HEAD, TABLE_HEADER, TABLE_ROW, TableAlign, TableDensity, TableVariant},
};

/// `variant` and `density` only set inherited `--table-*` properties on the
/// `<table>`; the heads and cells read them, so their own `class` still wins.
#[component]
pub fn Table(#[prop(optional)] variant: TableVariant, #[prop(optional)] density: TableDensity, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE, variant.as_class(), density.as_class(), class);
	view! {
		<div class=TABLE_CONTAINER data-slot="table-container">
			<table
				class=cls
				data-slot="table"
				data-variant=variant.as_ref().to_owned()
				data-density=density.as_ref().to_owned()
			>
				{children()}
			</table>
		</div>
	}
}

/// The surface a `TableVariant::Card` table sits in, edge to edge.
#[component]
pub fn TableCard(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_CARD, class);
	view! {
		<div class=cls data-slot="table-card">
			{children()}
		</div>
	}
}

#[component]
pub fn TableHeader(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_HEADER, class);
	view! {
		<thead class=cls data-slot="table-header">
			{children()}
		</thead>
	}
}

#[component]
pub fn TableBody(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_BODY, class);
	view! {
		<tbody class=cls data-slot="table-body">
			{children()}
		</tbody>
	}
}

#[component]
pub fn TableFooter(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_FOOTER, class);
	view! {
		<tfoot class=cls data-slot="table-footer">
			{children()}
		</tfoot>
	}
}

#[component]
pub fn TableRow(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_ROW, class);
	view! {
		<tr class=cls data-slot="table-row">
			{children()}
		</tr>
	}
}

#[component]
pub fn TableHead(#[prop(optional)] align: TableAlign, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_HEAD, align.as_class(), class);
	view! {
		<th class=cls data-slot="table-head">
			{children()}
		</th>
	}
}

#[component]
pub fn TableCell(#[prop(optional)] align: TableAlign, #[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_CELL, align.as_class(), class);
	view! {
		<td class=cls data-slot="table-cell">
			{children()}
		</td>
	}
}

#[component]
pub fn TableCaption(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
	let cls = cn!(TABLE_CAPTION, class);
	view! {
		<caption class=cls data-slot="table-caption">
			{children()}
		</caption>
	}
}
