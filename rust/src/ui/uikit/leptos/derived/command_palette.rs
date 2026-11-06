use leptos::prelude::*;

pub use ev_lib_classes::CommandPaletteLabels;
use crate::{
	cn,
	ui::uikit::leptos::{
		COMMAND_PALETTE_CONTENT, COMMAND_PALETTE_HINT, COMMAND_PALETTE_HINTS, COMMAND_PALETTE_LABEL, COMMAND_PALETTE_LIST, COMMAND_PALETTE_TITLE, CommandDialog, CommandEmpty, CommandGroup,
		CommandInput, CommandItem, CommandList, CommandSeparator, CommandShortcut, DialogTitle, Kbd,
		primitives::{use_controllable, use_stable_id},
	},
};

/// One action of a [`CommandPalette`]. Mirrors the TS `CommandPaletteItem`.
#[derive(Clone)]
pub struct CommandPaletteItem {
	pub id: String,
	pub label: String,
	pub icon: Option<ViewFn>,
	/// Shown at the row end, e.g. "⌘N".
	pub shortcut: Option<String>,
	/// Extra words the search matches besides the label.
	pub keywords: Vec<String>,
	pub disabled: bool,
	pub on_select: Callback<()>,
}

/// A run of [`CommandPaletteItem`]s under one heading. Mirrors the TS
/// `CommandPaletteGroup`.
#[derive(Clone)]
pub struct CommandPaletteGroup {
	pub id: String,
	pub heading: Option<String>,
	pub items: Vec<CommandPaletteItem>,
}

/// Search-and-run over a list of actions. Mirrors the TS `CommandPalette`.
/// Choosing a row closes the palette, then runs the row's `on_select`. Pair it
/// with [`use_command_palette_shortcut`] for ⌘K / Ctrl+K.
///
/// Built on [`CommandDialog`], so it keeps that dialog's limits: inline
/// `position: fixed`, no portal, no focus trap, and no place on the layer
/// stack. Escape and a click on the scrim close it.
#[component]
pub fn CommandPalette(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	groups: Vec<CommandPaletteGroup>,
	#[prop(optional)] labels: CommandPaletteLabels,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	let state = use_controllable(open, default_open, on_open_change);
	let title_id = use_stable_id("command-palette-title");
	let cls = cn!(COMMAND_PALETTE_CONTENT, class);
	let title = labels.title;
	let placeholder = labels.placeholder;
	let empty = labels.empty;
	let navigate = labels.navigate;
	let select = labels.select;
	let close = labels.close;
	let labelled_by = title_id.clone();
	// Run on every opening, so it clones what the inner parts take by value.
	let body = move || {
		let (title, empty, groups) = (title.clone(), empty.clone(), groups.clone());
		view! {
			<DialogTitle id=title_id.clone() class=COMMAND_PALETTE_TITLE>
				{title}
			</DialogTitle>
			<CommandInput placeholder=placeholder.clone() />
			<CommandList class=COMMAND_PALETTE_LIST>
				<CommandEmpty>{empty.clone()}</CommandEmpty>
				{groups
					.into_iter()
					.enumerate()
					.map(|(i, group)| {
						let items = group
							.items
							.into_iter()
							.map(|item| {
								let value = std::iter::once(item.label.as_str())
									.chain(item.keywords.iter().map(String::as_str))
									.collect::<Vec<_>>()
									.join(" ");
								let CommandPaletteItem { label, icon, shortcut, disabled, on_select: run, .. } = item;
								view! {
									<CommandItem
										value=value
										disabled=disabled
										on_select=Callback::new(move |_| {
											state.set(false);
											run.run(());
										})
									>
										{icon.as_ref().map(ViewFn::run)}
										<span class=COMMAND_PALETTE_LABEL>{label.clone()}</span>
										{shortcut.clone().map(|s| view! { <CommandShortcut>{s}</CommandShortcut> })}
									</CommandItem>
								}
							})
							.collect_view();
						view! {
							{(i > 0).then(|| view! { <CommandSeparator /> })}
							<CommandGroup heading=group.heading.unwrap_or_default()>{items}</CommandGroup>
						}
					})
					.collect_view()}
			</CommandList>
			<div data-slot="command-palette-hints" class=COMMAND_PALETTE_HINTS>
				<span class=COMMAND_PALETTE_HINT>
					<Kbd>"↑↓"</Kbd>
					{navigate.clone()}
				</span>
				<span class=COMMAND_PALETTE_HINT>
					<Kbd>"↵"</Kbd>
					{select.clone()}
				</span>
				<span class=COMMAND_PALETTE_HINT>
					<Kbd>"esc"</Kbd>
					{close.clone()}
				</span>
			</div>
		}
	};
	view! {
		<CommandDialog
			open=Signal::derive(move || state.get())
			on_open_change=Callback::new(move |next| state.set(next))
			aria_labelledby=labelled_by
			class=cls
		>
			{body()}
		</CommandDialog>
	}
}

/// Calls `on_trigger` on ⌘K (Mac) or Ctrl+K, and keeps the browser from taking
/// the key (Chrome focuses the address bar on Ctrl+K in some layouts). Mirrors
/// the TS `useCommandPaletteShortcut`.
///
/// It matches the physical key (`code`), so it also works on a Russian or other
/// non-Latin layout. A held key does not fire again, or it would toggle the
/// palette open and shut. The listener sits on `document` while `enabled` is
/// true and goes with the calling component.
///
/// Only the `wasm` feature on `wasm32` has a `document` to listen on; any other
/// build does nothing here, server rendering included.
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub fn use_command_palette_shortcut(on_trigger: impl FnMut() + 'static, enabled: impl Into<Signal<bool>>) {
	use std::{cell::RefCell, rc::Rc};

	use crate::ui::uikit::host::ShortcutListener;

	let enabled = enabled.into();
	let trigger = Rc::new(RefCell::new(on_trigger));
	// Dropping the listener removes it, so it goes with this owner.
	let listener = StoredValue::new_local(None::<ShortcutListener>);
	Effect::new(move |_| {
		if !enabled.get() {
			listener.set_value(None);
		} else if listener.with_value(Option::is_none) {
			let trigger = Rc::clone(&trigger);
			listener.set_value(ShortcutListener::add(move || (trigger.borrow_mut())()));
		}
	});
}

/// See the `wasm` build of this hook: without a `document` there is nothing to
/// listen on.
#[cfg(not(all(target_arch = "wasm32", feature = "wasm")))]
pub fn use_command_palette_shortcut(on_trigger: impl FnMut() + 'static, enabled: impl Into<Signal<bool>>) {
	let _ = (on_trigger, enabled);
}
