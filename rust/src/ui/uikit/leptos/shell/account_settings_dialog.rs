use leptos::prelude::*;

pub use crate::ui::uikit::core::AccountSettingsDialogLabels;
use crate::{
	cn,
	ui::uikit::leptos::{
		ACCOUNT_SETTINGS_DIALOG_CLOSE, ACCOUNT_SETTINGS_DIALOG_CONTENT, ACCOUNT_SETTINGS_DIALOG_DESCRIPTION, ACCOUNT_SETTINGS_DIALOG_ENTRY, ACCOUNT_SETTINGS_DIALOG_FOOTER,
		ACCOUNT_SETTINGS_DIALOG_HEADING, ACCOUNT_SETTINGS_DIALOG_LIST, ACCOUNT_SETTINGS_DIALOG_PANE, ACCOUNT_SETTINGS_DIALOG_PANEL, ACCOUNT_SETTINGS_DIALOG_SIDEBAR,
		ACCOUNT_SETTINGS_DIALOG_TABS, ACCOUNT_SETTINGS_DIALOG_TITLE, Dialog, DialogContent, DialogDescription, DialogTitle, IconButton, Orientation, Tabs, TabsContent, TabsList,
		TabsTrigger,
		icon_button::Glyph,
		primitives::{use_controllable, use_stable_id},
	},
};

/// One entry of an [`AccountSettingsDialog`] rail. Mirrors the TS
/// `AccountSettingsSection`.
#[derive(Clone)]
pub struct AccountSettingsSection {
	pub id: String,
	pub label: ViewFn,
	pub icon: Option<ViewFn>,
	/// Fills the pane while this entry is active.
	pub content: ViewFn,
}

/// The account modal: a vertical tab rail on the left, the active section on
/// the right. Mirrors the TS `AccountSettingsDialog`.
///
/// Sections are the caller's, so Profile and Security (or any other) are
/// built from [`SettingsSection`](super::SettingsSection) and
/// [`SettingsRow`](super::SettingsRow) inside `content`. The rail selects on
/// arrow, as [`Tabs`] does. With no `value` the first section starts active.
///
/// It is a [`Dialog`], so it keeps the kit's overlay limits: rendered inline
/// with `position: fixed`, no portal and no focus trap.
#[component]
pub fn AccountSettingsDialog(
	#[prop(optional, into)] open: MaybeProp<bool>,
	#[prop(optional)] default_open: bool,
	#[prop(optional)] on_open_change: Option<Callback<bool>>,
	/// The heading of the content pane; it also names the dialog.
	#[prop(into)]
	title: ViewFn,
	#[prop(optional, into)] description: Option<ViewFn>,
	sections: Vec<AccountSettingsSection>,
	#[prop(optional, into)] value: MaybeProp<String>,
	#[prop(optional, into)] default_value: Option<String>,
	#[prop(optional)] on_value_change: Option<Callback<String>>,
	/// Pinned under the rail entries: sign out, language.
	#[prop(optional, into)]
	sidebar_footer: Option<ViewFn>,
	#[prop(optional)] labels: AccountSettingsDialogLabels,
	#[prop(optional, into)] class: String,
) -> impl IntoView {
	// Owned here rather than left to the `Dialog`, because the close button
	// is an `IconButton`, which cannot reach the dialog's private state the
	// way `DialogClose` does.
	let open = use_controllable(open, default_open, on_open_change);
	let title_id = use_stable_id("account-settings-title");
	let description_id = use_stable_id("account-settings-description");
	let described_by = description.as_ref().map(|_| description_id.clone());
	let first = sections.first().map(|s| s.id.clone()).unwrap_or_default();
	let default_value = default_value.unwrap_or(first);
	let close_label = labels.close.unwrap_or_else(|| String::from("Close"));
	let cls = cn!(ACCOUNT_SETTINGS_DIALOG_CONTENT, class);
	let labelled_by = title_id.clone();
	// Run on every opening, so it clones what the inner parts take by value.
	let body = move || {
		let (rail, panes, sidebar_footer, title, description, close_label) = (
			sections.clone(),
			sections.clone(),
			sidebar_footer.clone(),
			title.clone(),
			description.clone(),
			close_label.clone(),
		);
		let (title_id, description_id, default_value) = (title_id.clone(), description_id.clone(), default_value.clone());
		view! {
			<Tabs
				orientation=Orientation::Vertical
				value=value
				default_value=default_value
				on_value_change=Callback::new(move |v| {
					if let Some(h) = on_value_change {
						h.run(v);
					}
				})
				class=ACCOUNT_SETTINGS_DIALOG_TABS
			>
				<div data-slot="account-settings-dialog-sidebar" class=ACCOUNT_SETTINGS_DIALOG_SIDEBAR>
					<TabsList class=ACCOUNT_SETTINGS_DIALOG_LIST>
						{rail
							.iter()
							.map(|s| {
								let (icon, label) = (s.icon.clone(), s.label.clone());
								view! {
									<TabsTrigger value=s.id.clone() class=ACCOUNT_SETTINGS_DIALOG_ENTRY>
										{icon.as_ref().map(ViewFn::run)}
										{label.run()}
									</TabsTrigger>
								}
							})
							.collect_view()}
					</TabsList>
					{sidebar_footer
						.map(|f| {
							view! {
								<div
									data-slot="account-settings-dialog-footer"
									class=ACCOUNT_SETTINGS_DIALOG_FOOTER
								>
									{f.run()}
								</div>
							}
						})}
				</div>
				<div data-slot="account-settings-dialog-pane" class=ACCOUNT_SETTINGS_DIALOG_PANE>
					<div data-slot="account-settings-dialog-heading" class=ACCOUNT_SETTINGS_DIALOG_HEADING>
						<DialogTitle id=title_id class=ACCOUNT_SETTINGS_DIALOG_TITLE>
							{title.run()}
						</DialogTitle>
						{description
							.map(|d| {
								view! {
									<DialogDescription id=description_id class=ACCOUNT_SETTINGS_DIALOG_DESCRIPTION>
										{d.run()}
									</DialogDescription>
								}
							})}
					</div>
					{panes
						.iter()
						.map(|s| {
							let content = s.content.clone();
							view! {
								<TabsContent value=s.id.clone() class=ACCOUNT_SETTINGS_DIALOG_PANEL>
									{content.run()}
								</TabsContent>
							}
						})
						.collect_view()}
				</div>
			</Tabs>
			<IconButton
				aria_label=close_label
				class=ACCOUNT_SETTINGS_DIALOG_CLOSE
				on_click=Callback::new(move |_| open.set(false))
			>
				<Glyph class="size-4">
					<path d="M18 6 6 18M6 6l12 12" />
				</Glyph>
			</IconButton>
		}
	};
	view! {
		<Dialog
			open=Signal::derive(move || open.get())
			on_open_change=Callback::new(move |next| open.set(next))
		>
			<DialogContent
				show_close_button=false
				aria_labelledby=labelled_by
				aria_describedby=described_by
				class=cls
			>
				{body()}
			</DialogContent>
		</Dialog>
	}
}
