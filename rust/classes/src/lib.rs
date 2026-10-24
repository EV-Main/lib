#![feature(default_field_values)]
//! Pure styling data for the uikit: Tailwind class tables, variant/size enums and
//! the `cn!` fuse macro. Carries no Dioxus — it's the single source of truth that
//! both the Rust components (`ev_lib`) and the TS codegen (`ev_lib_gen`) read from.

mod accent;
mod accordion;
mod account_list;
mod account_settings_dialog;
mod alert;
mod alert_dialog;
mod aside;
mod avatar;
mod badge;
mod band;
mod breadcrumb;
mod button;
mod button_group;
mod calendar;
mod card;
mod carousel;
mod chat_composer;
mod chat_message;
mod chat_typing;
mod checkbox;
mod command;
mod command_palette;
mod confirm_dialog;
mod container;
mod context_menu;
mod date_time_picker;
mod dialog;
mod drawer;
mod dropdown_menu;
mod empty;
mod field;
mod focus;
mod form;
mod form_row;
mod header_controls;
mod hover_card;
mod icon_button;
mod input;
mod input_group;
mod input_otp;
mod item;
mod kbd;
mod key_value;
mod label;
mod menubar;
mod navigation_menu;
mod popover;
mod popover_form;
mod progress;
mod radio_group;
mod scroll_area;
mod section_label;
mod select;
mod separator;
mod settings;
mod sheet;
mod sidebar;
mod size;
mod skeleton;
mod slider;
mod smart_header;
mod sonner;
mod status_item;
mod switch;
mod system_banner;
mod table;
mod tabs;
mod terminal;
mod textarea;
mod toggle;
mod tooltip;
mod top_bar;
mod workspace_tabs;

pub use accent::{Accent, accent_fill_class, accent_outline_class, accent_text_class, button_accent_class};
pub use accordion::{ACCORDION_CONTENT, ACCORDION_CONTENT_INNER, ACCORDION_HEADER, ACCORDION_ITEM, ACCORDION_TRIGGER};
pub use account_list::{
	ACCOUNT_LIST_BASE, ACCOUNT_LIST_ITEM_ACTIONS, ACCOUNT_LIST_ITEM_AVATAR, ACCOUNT_LIST_ITEM_AVATAR_FALLBACK, ACCOUNT_LIST_ITEM_BADGES, ACCOUNT_LIST_ITEM_BASE, ACCOUNT_LIST_ITEM_CHECK,
	ACCOUNT_LIST_ITEM_CHECK_HIDDEN, ACCOUNT_LIST_ITEM_CHECKED, ACCOUNT_LIST_ITEM_DISABLED, ACCOUNT_LIST_ITEM_FIGURE, ACCOUNT_LIST_ITEM_HANDLE, ACCOUNT_LIST_ITEM_NAME,
	ACCOUNT_LIST_ITEM_RADIO, ACCOUNT_LIST_ITEM_TEXT, ACCOUNT_LIST_ITEM_UNCHECKED,
};
pub use account_settings_dialog::{
	ACCOUNT_SETTINGS_DIALOG_CLOSE, ACCOUNT_SETTINGS_DIALOG_CONTENT, ACCOUNT_SETTINGS_DIALOG_DESCRIPTION, ACCOUNT_SETTINGS_DIALOG_ENTRY, ACCOUNT_SETTINGS_DIALOG_FOOTER,
	ACCOUNT_SETTINGS_DIALOG_HEADING, ACCOUNT_SETTINGS_DIALOG_LIST, ACCOUNT_SETTINGS_DIALOG_PANE, ACCOUNT_SETTINGS_DIALOG_PANEL, ACCOUNT_SETTINGS_DIALOG_SIDEBAR,
	ACCOUNT_SETTINGS_DIALOG_TABS, ACCOUNT_SETTINGS_DIALOG_TITLE,
};
pub use alert::{ALERT_BASE, ALERT_DESCRIPTION, ALERT_TITLE, AlertVariant};
pub use alert_dialog::{ALERT_DIALOG_CONTENT, ALERT_DIALOG_DESCRIPTION, ALERT_DIALOG_FOOTER, ALERT_DIALOG_HEADER, ALERT_DIALOG_OVERLAY, ALERT_DIALOG_TITLE};
pub use aside::{
	ASIDE_BODY, ASIDE_FOOTER, ASIDE_HEADER_BACK_ICON, ASIDE_HEADER_BASE, ASIDE_HEADER_CLOSE_ICON, ASIDE_HEADER_TITLE, ASIDE_PANEL_DOCKED, ASIDE_PANEL_DRAWER, ASIDE_PANEL_NARROW_ONLY,
	ASIDE_PANEL_SHEET, ASIDE_PANEL_WIDE_ONLY, ASIDE_PANEL_WIDE_ONLY_WRAPPER, ASIDE_RAIL_BASE, ASIDE_RAIL_SPACER, AsidePanelSide,
};
pub use avatar::{AVATAR, AVATAR_FALLBACK, AVATAR_IMAGE, AvatarShape, AvatarSize};
pub use badge::{BADGE_BASE, BadgeVariant};
pub use band::{CHECK, DISPLAY_BASE, EYEBROW, LEDE, PROSE, Polarity, SECTION_BASE, SECTION_HEAD, SECTION_PY, SECTION_PY_TIGHT, STAT, STAT_FIGURE, STAT_LABEL, Surface};
pub use breadcrumb::{BREADCRUMB_ELLIPSIS, BREADCRUMB_ITEM, BREADCRUMB_LINK, BREADCRUMB_LIST, BREADCRUMB_PAGE, BREADCRUMB_SEPARATOR};
pub use button::{BUTTON_BASE, ButtonVariant, button_size_class};
pub use button_group::{BUTTON_GROUP_BASE, BUTTON_GROUP_SEPARATOR_BASE, BUTTON_GROUP_TEXT_BASE, ButtonGroupOrientation};
pub use calendar::{
	CALENDAR_CAPTION, CALENDAR_DAY, CALENDAR_DAY_CELL, CALENDAR_DAY_EMPTY, CALENDAR_DAY_SELECTED, CALENDAR_DAY_TODAY, CALENDAR_GRID, CALENDAR_NAV, CALENDAR_NAV_BUTTON, CALENDAR_ROOT,
	CALENDAR_WEEK, CALENDAR_WEEKDAY, CALENDAR_WEEKDAY_ROW,
};
pub use card::{CARD, CARD_ACTION, CARD_CONTENT, CARD_DESCRIPTION, CARD_FOOTER, CARD_HEADER, CARD_TITLE};
pub use carousel::{
	CAROUSEL_CONTENT_TRACK, CAROUSEL_CONTENT_TRACK_HORIZONTAL, CAROUSEL_CONTENT_TRACK_VERTICAL, CAROUSEL_CONTENT_VIEWPORT, CAROUSEL_EDGE_FADE_NEXT, CAROUSEL_EDGE_FADE_PREV, CAROUSEL_ITEM,
	CAROUSEL_ITEM_HORIZONTAL, CAROUSEL_ITEM_VERTICAL, CAROUSEL_NAV, CAROUSEL_NEXT_HORIZONTAL, CAROUSEL_NEXT_VERTICAL, CAROUSEL_PREVIOUS_HORIZONTAL, CAROUSEL_PREVIOUS_VERTICAL,
};
pub use chat_composer::{
	CHAT_COMPOSER_ATTACHMENTS, CHAT_COMPOSER_BASE, CHAT_COMPOSER_FIELD, CHAT_COMPOSER_HINT, CHAT_COMPOSER_IDLE, CHAT_COMPOSER_SPACER, CHAT_COMPOSER_STREAMING, CHAT_COMPOSER_TOOLBAR,
	MODEL_CHIP_BASE,
};
pub use chat_message::{
	CHAT_ATTACHMENT_BASE, CHAT_ATTACHMENT_BODY, CHAT_ATTACHMENT_ICON, CHAT_ATTACHMENT_META, CHAT_ATTACHMENT_NAME, CHAT_ATTACHMENT_PROGRESS, CHAT_ATTACHMENT_PROGRESS_FILL,
	CHAT_ATTACHMENT_ROW, CHAT_BUBBLE_BASE, CHAT_MARKER_DOT, CHAT_MARKER_RULE, CHAT_MARKER_SEPARATOR, CHAT_MARKER_SEPARATOR_LABEL, CHAT_MARKER_STATUS, CHAT_MARKER_TOOL,
	CHAT_MARKER_TOOL_BODY, CHAT_MARKER_TOOL_BORDER, CHAT_MARKER_TOOL_CHECK, CHAT_MARKER_TOOL_DETAIL, CHAT_MARKER_TOOL_ERROR_BORDER, CHAT_MARKER_TOOL_ICON, CHAT_MARKER_TOOL_META,
	CHAT_MARKER_TOOL_TITLE, CHAT_MESSAGE_AVATAR, CHAT_MESSAGE_AVATAR_TILE, CHAT_MESSAGE_BASE, CHAT_MESSAGE_BODY, CHAT_MESSAGE_BODY_ASSISTANT, CHAT_MESSAGE_BODY_USER, CHAT_MESSAGE_HEADER,
	CHAT_MESSAGE_NAME, CHAT_MESSAGE_TIME, CHAT_MESSAGE_USER, ChatBubbleVariant, ChatStatusState, ChatToolState,
};
pub use chat_typing::{CHAT_TYPING_BASE, CHAT_TYPING_DOT, CHAT_TYPING_STILL};
pub use checkbox::{CHECKBOX_BASE, CHECKBOX_INDICATOR};
pub use command::{
	COMMAND_DIALOG_COMMAND, COMMAND_DIALOG_CONTENT, COMMAND_DIALOG_OVERLAY, COMMAND_EMPTY, COMMAND_GROUP, COMMAND_INPUT, COMMAND_INPUT_WRAPPER, COMMAND_ITEM, COMMAND_LIST, COMMAND_ROOT,
	COMMAND_SEPARATOR, COMMAND_SHORTCUT,
};
pub use command_palette::{
	COMMAND_PALETTE_COMMAND, COMMAND_PALETTE_CONTENT, COMMAND_PALETTE_HINT, COMMAND_PALETTE_HINTS, COMMAND_PALETTE_LABEL, COMMAND_PALETTE_LIST, COMMAND_PALETTE_TITLE,
};
pub use confirm_dialog::{
	CONFIRM_DIALOG_CONTENT, CONFIRM_DIALOG_DESCRIPTION, CONFIRM_DIALOG_ERROR, CONFIRM_DIALOG_FOOTER, CONFIRM_DIALOG_HEADER, CONFIRM_DIALOG_ICON, CONFIRM_DIALOG_PENDING,
	CONFIRM_DIALOG_SPINNER, CONFIRM_DIALOG_TEXT, CONFIRM_DIALOG_TITLE, ConfirmDialogTone,
};
pub use container::CONTAINER_BASE;
pub use context_menu::{
	CONTEXT_MENU_CHECK_ITEM, CONTEXT_MENU_CONTENT, CONTEXT_MENU_ITEM, CONTEXT_MENU_LABEL, CONTEXT_MENU_SEPARATOR, CONTEXT_MENU_SHORTCUT, CONTEXT_MENU_SUB_CONTENT, CONTEXT_MENU_SUB_TRIGGER,
};
pub use date_time_picker::{DATE_TIME_PICKER_CLEAR, DATE_TIME_PICKER_CONTENT, DATE_TIME_PICKER_TIME, DATE_TIME_PICKER_TIME_INPUT, DATE_TIME_PICKER_TIME_SEPARATOR, DATE_TIME_PICKER_TRIGGER};
pub use dialog::{DIALOG_CLOSE, DIALOG_CONTENT, DIALOG_DESCRIPTION, DIALOG_FOOTER, DIALOG_HEADER, DIALOG_OVERLAY, DIALOG_TITLE};
pub use drawer::{DRAWER_BODY, DRAWER_CONTENT_BASE, DRAWER_DESCRIPTION, DRAWER_FOOTER, DRAWER_HANDLE, DRAWER_HEADER, DRAWER_OVERLAY, DRAWER_TITLE, DrawerDirection};
pub use dropdown_menu::{
	DROPDOWN_MENU_CHECK_ITEM, DROPDOWN_MENU_CONTENT, DROPDOWN_MENU_ITEM, DROPDOWN_MENU_ITEM_INDICATOR, DROPDOWN_MENU_LABEL, DROPDOWN_MENU_SEPARATOR, DROPDOWN_MENU_SHORTCUT,
	DROPDOWN_MENU_SUB_CONTENT, DROPDOWN_MENU_SUB_TRIGGER,
};
pub use empty::{EMPTY, EMPTY_CONTENT, EMPTY_DESCRIPTION, EMPTY_HEADER, EMPTY_MEDIA_BASE, EMPTY_TITLE, EmptyMediaVariant};
pub use field::{
	FIELD_BASE, FIELD_CONTENT, FIELD_DESCRIPTION, FIELD_ERROR, FIELD_GROUP, FIELD_LABEL, FIELD_LEGEND, FIELD_SEPARATOR, FIELD_SEPARATOR_CONTENT, FIELD_SEPARATOR_LINE, FIELD_SET,
	FIELD_TITLE, FieldOrientation,
};
pub use focus::{FILLED_FOCUS_RING, OPTION_FOCUS_RING};
pub use form::{FORM_DESCRIPTION, FORM_ITEM, FORM_LABEL, FORM_MESSAGE};
pub use form_row::{FORM_ROW_BASE, FORM_ROW_CONTROL, FORM_ROW_DESCRIPTION, FORM_ROW_DIVIDER, FORM_ROW_HINT, FORM_ROW_LABEL, FORM_ROW_LABEL_COLUMN};
pub use header_controls::{
	HEADER_BALANCE_BASE, HEADER_BALANCE_CHEVRON, HEADER_BALANCE_FIGURE, HEADER_BALANCE_INTERACTIVE, HEADER_BALANCE_LABEL, HEADER_BALANCE_SETTLED, HEADER_BALANCE_SKELETON,
	HEADER_SEARCH_BASE, HEADER_SEARCH_PLACEHOLDER, HeaderBalanceTone, NOTIFICATION_BELL_CONTENT, NOTIFICATION_BELL_EMPTY, NOTIFICATION_BELL_FOOTER, NOTIFICATION_BELL_ITEM,
	NOTIFICATION_BELL_ITEM_BODY, NOTIFICATION_BELL_ITEM_DESCRIPTION, NOTIFICATION_BELL_ITEM_DOT, NOTIFICATION_BELL_ITEM_DOT_READ, NOTIFICATION_BELL_ITEM_DOT_UNREAD,
	NOTIFICATION_BELL_ITEM_TIME, NOTIFICATION_BELL_ITEM_TITLE, NOTIFICATION_BELL_ITEM_TITLE_UNREAD, NOTIFICATION_BELL_LIST, NOTIFICATION_BELL_TRIGGER, NOTIFICATION_DOT,
};
pub use hover_card::HOVER_CARD_CONTENT;
pub use icon_button::{ICON_BUTTON_BASE, ICON_BUTTON_PRESSED, IconButtonVariant};
pub use input::{INPUT_BASE, input_size_class};
pub use input_group::{
	INPUT_GROUP_ADDON_BASE, INPUT_GROUP_BASE, INPUT_GROUP_BUTTON_BASE, INPUT_GROUP_INPUT_CONTROL, INPUT_GROUP_TEXT, INPUT_GROUP_TEXTAREA_CONTROL, InputGroupAddonAlign, InputGroupButtonSize,
	input_group_button_size_class,
};
pub use input_otp::{INPUT_OTP_CONTAINER, INPUT_OTP_GROUP, INPUT_OTP_INPUT, INPUT_OTP_SLOT, INPUT_OTP_SLOT_CARET, INPUT_OTP_SLOT_CARET_WRAPPER};
pub use item::{
	ITEM_ACTIONS, ITEM_BASE, ITEM_CONTENT, ITEM_DESCRIPTION, ITEM_FOOTER, ITEM_GROUP, ITEM_HEADER, ITEM_MEDIA_BASE, ITEM_SEPARATOR, ITEM_TITLE, ItemMediaVariant, ItemSize, ItemVariant,
};
pub use kbd::{KBD_BASE, KBD_GROUP_BASE};
pub use key_value::{KEY_VALUE_BASE, KEY_VALUE_COPY, KEY_VALUE_LABEL, KEY_VALUE_ROW, KEY_VALUE_STATUS, KEY_VALUE_VALUE};
pub use label::LABEL_BASE;
pub use menubar::{
	MENUBAR_CHECKBOX_ITEM, MENUBAR_CONTENT, MENUBAR_ITEM, MENUBAR_ITEM_INDICATOR, MENUBAR_LABEL, MENUBAR_RADIO_ITEM, MENUBAR_ROOT, MENUBAR_SEPARATOR, MENUBAR_SHORTCUT, MENUBAR_SUB_CONTENT,
	MENUBAR_SUB_TRIGGER, MENUBAR_TRIGGER,
};
pub use navigation_menu::{
	NAVIGATION_MENU, NAVIGATION_MENU_CONTENT, NAVIGATION_MENU_INDICATOR, NAVIGATION_MENU_ITEM, NAVIGATION_MENU_LINK, NAVIGATION_MENU_LIST, NAVIGATION_MENU_TRIGGER_STYLE,
	NAVIGATION_MENU_VIEWPORT,
};
pub use popover::POPOVER_CONTENT;
pub use popover_form::{POPOVER_FORM_ACTIONS, POPOVER_FORM_CONTENT, POPOVER_FORM_FIELDS, POPOVER_FORM_FORM, POPOVER_FORM_TITLE};
pub use progress::{PROGRESS_INDICATOR, PROGRESS_TRACK};
pub use radio_group::{RADIO_GROUP_ITEM, RADIO_GROUP_ROOT};
pub use scroll_area::{SCROLL_AREA_THUMB, SCROLL_AREA_VIEWPORT, SCROLLBAR_BASE, ScrollBarOrientation};
pub use section_label::{SECTION_LABEL_BASE, SectionLabelTone};
pub use select::{
	NATIVE_SELECT_BASE, SELECT_CONTENT_BOUNDS, SELECT_CONTENT_SURFACE, SELECT_ITEM, SELECT_LABEL, SELECT_LIST, SELECT_SEPARATOR, SELECT_TRIGGER_BASE, native_select_size_class,
	select_trigger_size_class,
};
pub use separator::{Orientation, SEPARATOR_BASE};
pub use settings::{
	SETTINGS_LAYOUT, SETTINGS_MAIN, SETTINGS_NAV, SETTINGS_ROW_INLINE, SETTINGS_SECTION_ACTION, SETTINGS_SECTION_BASE, SETTINGS_SECTION_BODY, SETTINGS_SECTION_BODY_FLUSH,
	SETTINGS_SECTION_BODY_PADDED, SETTINGS_SECTION_DESCRIPTION, SETTINGS_SECTION_HEADER, SETTINGS_SECTION_HEADER_FLUSH, SETTINGS_SECTION_HEADING, SETTINGS_SECTION_TITLE,
	SettingsSectionTone,
};
pub use sheet::{
	SHEET_CLOSE, SHEET_CONTENT, SHEET_DESCRIPTION, SHEET_FOOTER, SHEET_HEADER, SHEET_OVERLAY, SHEET_SIDE_BOTTOM, SHEET_SIDE_LEFT, SHEET_SIDE_RIGHT, SHEET_SIDE_TOP, SHEET_TITLE,
};
pub use sidebar::{
	SIDEBAR_CONTENT, SIDEBAR_FLAT, SIDEBAR_FOOTER, SIDEBAR_GROUP, SIDEBAR_GROUP_CONTENT, SIDEBAR_GROUP_LABEL, SIDEBAR_HEADER, SIDEBAR_INNER, SIDEBAR_INSET, SIDEBAR_MENU,
	SIDEBAR_MENU_BUTTON_BASE, SIDEBAR_MENU_ITEM, SIDEBAR_RAIL, SIDEBAR_SEPARATOR, SIDEBAR_TRIGGER, SIDEBAR_WRAPPER, sidebar_menu_button_size_class,
};
pub use size::Size;
pub use skeleton::SKELETON_BASE;
pub use slider::{SLIDER_RANGE, SLIDER_ROOT, SLIDER_THUMB, SLIDER_TRACK};
pub use smart_header::{SMART_HEADER_BAR, SMART_HEADER_BASE, SMART_HEADER_CONTEXT, SMART_HEADER_TOOLBAR};
pub use sonner::{TOAST_BASE, TOAST_CLOSE, TOAST_CONTENT, TOAST_TITLE, TOASTER_BASE, ToastPosition, ToastVariant};
pub use status_item::{STATUS_DOT_BASE, STATUS_ITEM_BASE, STATUS_ITEM_VALUE, StatusTone};
pub use switch::{SWITCH_BASE, SWITCH_THUMB};
pub use system_banner::{
	SYSTEM_BANNER_ACTION, SYSTEM_BANNER_BASE, SYSTEM_BANNER_BODY_AFTER_TITLE, SYSTEM_BANNER_DISMISS, SYSTEM_BANNER_GLYPH, SYSTEM_BANNER_ICON, SYSTEM_BANNER_MESSAGE, SYSTEM_BANNER_TITLE,
	SystemBannerTone,
};
pub use table::{TABLE, TABLE_BODY, TABLE_CAPTION, TABLE_CARD, TABLE_CELL, TABLE_CONTAINER, TABLE_FOOTER, TABLE_HEAD, TABLE_HEADER, TABLE_ROW, TableAlign, TableDensity, TableVariant};
pub use tabs::{TABS_CONTENT, TABS_LIST, TABS_ROOT, TABS_TRIGGER};
pub use terminal::{
	BookSide, OPEN_ORDERS_EMPTY, ORDER_BOOK, ORDER_BOOK_DEPTH, ORDER_BOOK_HEAD, ORDER_BOOK_PRICE, ORDER_BOOK_ROW, ORDER_BOOK_SPREAD, ORDER_FORM, ORDER_FORM_LABEL, ORDER_FORM_ROW,
	ORDER_FORM_SUBMIT_BASE, ORDER_FORM_VALUE, OrderSide, TERMINAL_CHART, TERMINAL_PANE, TERMINAL_PANE_BODY, TERMINAL_PANE_HEADER, TERMINAL_ROOT, TERMINAL_TICKER, TERMINAL_TICKER_LABEL,
	TERMINAL_TICKER_STAT, TERMINAL_TICKER_VALUE, TRADES_TAPE_ROW, TerminalArea, book_depth_class,
};
pub use textarea::{TEXTAREA_BASE, textarea_size_class};
pub use toggle::{TOGGLE_BASE, ToggleVariant, toggle_size_class};
pub use tooltip::TOOLTIP_CONTENT;
pub use top_bar::{TOP_BAR_BASE, TOP_BAR_END, TOP_BAR_START};
pub use workspace_tabs::{WORKSPACE_TAB_BASE, WORKSPACE_TAB_CLOSE, WORKSPACE_TAB_LABEL, WORKSPACE_TAB_SELECTED, WORKSPACE_TAB_SPACER, WORKSPACE_TAB_TRIGGER, WORKSPACE_TABS_BASE};

/// Fuses any number of class fragments into a single `String` with real Tailwind
/// conflict resolution (like `clsx` + `tailwind-merge`): empty fragments drop, the
/// rightmost conflicting utility wins. The TS mirror is `cn` in `@evinvest/uikit`.
///
/// `tw_merge` parses every utility in the string on every render, and the
/// overwhelmingly common call is one kit constant plus a caller override that is
/// empty — so that call returns the constant untouched.
#[macro_export]
macro_rules! cn {
	($base:expr, $class:expr $(,)?) => {{
		// `tw_merge!` brings its own traits into scope for the argument
		// expressions (`variant.as_class()`); the fast path has to do the same.
		#[allow(unused_imports)]
		use ::tailwind_fuse::*;
		let base = $base;
		let class = $class;
		if ::core::convert::AsRef::<str>::as_ref(&class).is_empty() {
			::std::borrow::ToOwned::to_owned(::core::convert::AsRef::<str>::as_ref(&base))
		} else {
			$crate::fuse(&::tailwind_fuse::tw_join!(base, class))
		}
	}};
	($($frag:expr),+ $(,)?) => {
		$crate::fuse(&::tailwind_fuse::tw_join!($($frag),+))
	};
}

/// Every Tailwind class literal the kit can emit, one per line.
///
/// A Rust consumer takes this crate from crates.io, whose unpacked sources sit at
/// no path a committed `@source` could reach, so Tailwind cannot see the kit's
/// class strings. Write this out from `build.rs` and `@source` the result:
///
/// ```no_run
/// std::fs::write("uikit-classes.txt", ev_lib_classes::CLASS_INVENTORY).unwrap();
/// ```
///
/// Regenerated by `cargo run -p ev_lib_gen`.
pub const CLASS_INVENTORY: &str = include_str!("../uikit-classes.txt");
/// The token stylesheet the class tables above are written against, flattened.
///
/// Tailwind cannot `@import` out of a crates.io checkout any more than it can
/// `@source` into one, so a Rust consumer writes this out from `build.rs` beside
/// its entrypoint and imports the result:
///
/// ```no_run
/// std::fs::write("assets/tokens.css", ev_lib_classes::TOKENS_CSS).unwrap();
/// ```
///
/// Regenerated by `cargo run -p ev_lib_gen`.
pub const TOKENS_CSS: &str = include_str!("../css/tokens.css");
/// The brand-neutral half of [`TOKENS_CSS`], flattened: the Tailwind mapping,
/// the derived tokens, the geometry and the motion. A consumer with its own
/// palette imports this and its palette instead of [`TOKENS_CSS`], which would
/// otherwise leave EV's values under every name the palette misses.
///
/// Regenerated by `cargo run -p ev_lib_gen`.
pub const THEME_CSS: &str = include_str!("../css/theme.css");
/// EV's palette against [`THEME_CSS`] — the other half of [`TOKENS_CSS`].
///
/// Regenerated by `cargo run -p ev_lib_gen`.
pub const EV_CSS: &str = include_str!("../css/ev.css");
/// `cn!`'s merge step: `tailwind_fuse` with one rule brought in line with
/// `tailwind-merge`. Out of the box it files `ring-[3px]` under ring *colour* —
/// its width rule takes only a bare integer — so the kit's halo width and a
/// `ring-0` never collide, and the halo's `ring-ring/50` is the one that gets
/// dropped instead. The TS port reads an arbitrary length as a width; so must
/// this one, or the two ports fuse the same tables into different classes.
///
/// `tailwind_fuse` cannot parse Tailwind v4's `(--var)` shorthand, and keeps a
/// class it cannot parse beside the one it should replace: `rounded-full
/// rounded-(--control-radius)` would survive whole, with the CSS order picking
/// the radius. Such a class is merged in its `[var(--var)]` long form and
/// written back as the caller wrote it.
#[doc(hidden)]
pub fn fuse(class: &str) -> String {
	if !class.contains("-(") {
		return merge(class);
	}
	let mut written = std::collections::HashMap::new();
	let long: Vec<String> = class
		.split_whitespace()
		.map(|token| match var_long_form(token) {
			Some(long) => {
				written.insert(long.clone(), token);
				long
			}
			None => token.to_owned(),
		})
		.collect();
	merge(&long.join(" "))
		.split(' ')
		.map(|token| written.get(token).copied().unwrap_or(token))
		.collect::<Vec<_>>()
		.join(" ")
}

fn merge(class: &str) -> String {
	tailwind_fuse::merge::tw_merge_override(&[class], tailwind_fuse::merge::MergeOptions::default(), ring_width_collision, |_: &str| None)
}

/// `rounded-(--r)` as `rounded-[var(--r)]`, and `text-(length:--s)` as
/// `text-[length:var(--s)]`; `None` for a class without the shorthand.
fn var_long_form(token: &str) -> Option<String> {
	let inner = token.strip_suffix(')')?;
	let open = inner.rfind("-(")?;
	let (head, var) = (&inner[..open], &inner[open + 2..]);
	match var.split_once(':') {
		Some((hint, var)) if var.starts_with("--") => Some(format!("{head}-[{hint}:var({var})]")),
		_ if var.starts_with("--") => Some(format!("{head}-[var({var})]")),
		_ => None,
	}
}

fn ring_width_collision(elements: &[&str], arbitrary: Option<&str>) -> Option<&'static str> {
	match (elements, arbitrary) {
		(["ring"], Some(value)) if is_arbitrary_length(value) => Some("ring-width"),
		(["text", size @ ..], None) if !size.is_empty() && TEXT_SCALE.contains(&size.join("-").as_str()) => Some("font-size"),
		// The kit's display face (theme.css `--font-display`). `tailwind_fuse`
		// knows only `sans`/`serif`/`mono` as families and files the rest under
		// weight, so a later `font-light` would drop the face. `tailwind-merge`
		// reads it as a family.
		(["font", "display"], None) => Some("font-family"),
		_ => None,
	}
}

/// The kit's `--text-*` scale (theme.css). `tailwind_fuse` knows only t-shirt
/// sizes as font sizes, so without this `text-body` is filed under text colour
/// and a later `text-ink` drops it. The TS port extends tailwind-merge the same way.
const TEXT_SCALE: [&str; 9] = ["caption", "body-s", "body", "body-l", "h3", "h2", "h1", "figure-m", "figure-l"];

/// `length:` is Tailwind's explicit type hint; otherwise a value opening with a
/// digit is a dimension (`3px`, `0.5rem`) — a colour never does.
fn is_arbitrary_length(value: &str) -> bool {
	value.starts_with("length:") || value.starts_with(|c: char| c.is_ascii_digit() || c == '.')
}

#[cfg(test)]
mod tests {
	#[test]
	fn joins_distinct_classes() {
		assert_eq!(cn!("flex", "items-center", "justify-center"), "flex items-center justify-center");
	}

	/// Mirrors `tailwind-merge`: an arbitrary ring length is a width, so it
	/// yields to `ring-0` and leaves the ring colour alone.
	#[test]
	fn arbitrary_ring_length_is_a_width() {
		assert_eq!(
			cn!("focus-visible:ring-ring/50 focus-visible:ring-[3px]", "focus-visible:ring-0"),
			"focus-visible:ring-ring/50 focus-visible:ring-0"
		);
		assert_eq!(cn!("ring-[3px]", "ring-[length:4px]"), "ring-[length:4px]");
		assert_eq!(cn!("ring-[3px]", "ring-[#fff]"), "ring-[3px] ring-[#fff]");
	}

	/// The kit's type scale is a font size, not a colour: it survives a text colour
	/// and yields to another size.
	#[test]
	fn type_scale_is_a_font_size() {
		assert_eq!(cn!("text-body", "text-ink"), "text-body text-ink");
		assert_eq!(cn!("text-body-s text-ink-mid", "text-body-l"), "text-ink-mid text-body-l");
		assert_eq!(cn!("text-figure-m", "text-sm"), "text-sm");
	}

	/// Mirrors `tailwind-merge`: the v4 `(--var)` shorthand conflicts like its
	/// `[var(--var)]` long form, and comes back as written.
	#[test]
	fn var_shorthand_conflicts_like_its_long_form() {
		assert_eq!(cn!("rounded-full size-7", "rounded-(--control-radius)"), "size-7 rounded-(--control-radius)");
		assert_eq!(cn!("rounded-(--control-radius)", "rounded-full"), "rounded-full");
		assert_eq!(cn!("hover:rounded-(--a)", "rounded-(--b)"), "hover:rounded-(--a) rounded-(--b)");
		assert_eq!(cn!("text-(length:--size) text-ink", "text-body"), "text-ink text-body");
		assert_eq!(cn!("has-[:focus-visible]:ring-2", "p-(--pad) p-2"), "has-[:focus-visible]:ring-2 p-2");
	}

	/// The display face is a family, not a weight: a weight beside it keeps it,
	/// and another family replaces it.
	#[test]
	fn display_face_is_a_font_family() {
		assert_eq!(cn!("text-lg font-semibold", "font-display font-light"), "text-lg font-display font-light");
		assert_eq!(cn!("font-display", "font-sans"), "font-sans");
	}

	#[test]
	fn rightmost_wins_on_conflict() {
		assert_eq!(cn!("p-4", "p-2"), "p-2");
		assert_eq!(cn!("bg-primary", "bg-secondary"), "bg-secondary");
	}

	#[test]
	fn keeps_refinements() {
		assert_eq!(cn!("p-4", "py-2"), "p-4 py-2");
	}

	#[test]
	fn drops_empty_fragments() {
		assert_eq!(cn!("px-2", "", "py-1"), "px-2 py-1");
		assert_eq!(cn!(""), "");
	}

	#[test]
	fn mixes_str_and_owned_override() {
		let base = "h-9 px-4 py-2";
		let class = String::from("px-6");
		assert_eq!(cn!(base, class), "h-9 py-2 px-6");
	}

	#[test]
	fn trailing_comma_and_single_fragment() {
		assert_eq!(cn!("rounded-md",), "rounded-md");
	}

	/// A caller's `text-*` overrides the base ink of an eyebrow but not the
	/// ancestor-scoped one it wears inside a filled band: different variant,
	/// different group.
	#[test]
	fn keeps_parent_scoped_variant_under_override() {
		assert_eq!(cn!("text-primary-ink [.bg-primary_&]:text-on-primary", "text-ink"), "[.bg-primary_&]:text-on-primary text-ink");
	}
}
