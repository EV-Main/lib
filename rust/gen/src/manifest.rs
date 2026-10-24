//! Flat per-component list of TS exports. Adding a class string is one line here;
//! the class values themselves live only in `ev_lib_classes`. Generic enum tables
//! use [`table`]; the few key-quirk tables are hand-built `Ts::Table` recipes.

use ev_lib::ts_gen::Ts;
use ev_lib_classes::*;
use strum::IntoEnumIterator;

use crate::table;

pub fn manifest() -> Vec<(&'static str, Vec<Ts>)> {
	vec![
		("button", button()),
		("accent", accent()),
		("alert", alert()),
		("toggle", toggle()),
		("badge", badge()),
		("icon-button", icon_button()),
		("status-item", status_item()),
		("form-row", form_row()),
		("settings", uikit_settings()),
		("system-banner", system_banner()),
		("section-label", section_label()),
		("workspace-tab", workspace_tab()),
		("account-list", account_list()),
		("account-settings-dialog", account_settings_dialog()),
		("popover-form", popover_form()),
		("confirm-dialog", confirm_dialog()),
		("key-value", key_value()),
		("top-bar", top_bar()),
		("smart-header", smart_header()),
		("header-controls", header_controls()),
		("aside", aside()),
		("focus", focus()),
		("switch", switch()),
		("checkbox", checkbox()),
		("separator", separator()),
		("button-group", button_group()),
		("empty", empty()),
		("scroll-area", scroll_area()),
		("input", input()),
		("label", label()),
		("skeleton", skeleton()),
		("textarea", textarea()),
		("select", select()),
		("kbd", kbd()),
		("container", container()),
		("band", band()),
		("card", card()),
		("breadcrumb", breadcrumb()),
		("avatar", avatar()),
		("alert-dialog", alert_dialog()),
		("sheet", sheet()),
		("popover", popover()),
		("tooltip", tooltip()),
		("dialog", dialog()),
		("command", command()),
		("command-palette", command_palette()),
		("table", data_table()),
		("form", form()),
		("navigation-menu", navigation_menu()),
		("hover-card", hover_card()),
		("progress", progress()),
		("radio-group", radio_group()),
		("input-otp", input_otp()),
		("calendar", calendar()),
		("date-time-picker", date_time_picker()),
		("drawer", drawer()),
		("slider", slider()),
		("tabs", tabs()),
		("carousel", carousel()),
		("accordion", accordion()),
		("dropdown-menu", dropdown_menu()),
		("context-menu", context_menu()),
		("menubar", menubar()),
		("sonner", sonner()),
		("field", field()),
		("input-group", input_group()),
		("item", item()),
		("sidebar", sidebar()),
	]
}

/// Registries shared by more than the uikit: `(package, module, exports)` into
/// `ts/{package}/src/generated/{module}.ts`. Not class tables — these are the
/// same data the Rust libraries own, which the TS ports used to re-type by hand.
pub fn shared() -> Vec<(&'static str, &'static str, Vec<Ts>)> {
	vec![
		("i18n", "locales", locales()),
		("types", "e164", e164()),
		("experiments", "contract", experiments()),
		("settings", "contract", settings()),
		("analytics", "host", analytics()),
		("kitstart", "contract", kitstart()),
	]
}
fn band() -> Vec<Ts> {
	vec![
		table::<Polarity>("polarityClasses", "Polarity"),
		table::<Surface>("surfaceClasses", "Surface"),
		Ts::Const {
			name: "SECTION_BASE",
			value: SECTION_BASE,
		},
		Ts::Const {
			name: "SECTION_PY",
			value: SECTION_PY,
		},
		Ts::Const {
			name: "SECTION_PY_TIGHT",
			value: SECTION_PY_TIGHT,
		},
		Ts::Const {
			name: "SECTION_HEAD",
			value: SECTION_HEAD,
		},
		Ts::Const { name: "EYEBROW", value: EYEBROW },
		Ts::Const {
			name: "DISPLAY_BASE",
			value: DISPLAY_BASE,
		},
		Ts::Const { name: "LEDE", value: LEDE },
		Ts::Const { name: "PROSE", value: PROSE },
		Ts::Const { name: "STAT", value: STAT },
		Ts::Const {
			name: "STAT_FIGURE",
			value: STAT_FIGURE,
		},
		Ts::Const {
			name: "STAT_LABEL",
			value: STAT_LABEL,
		},
		Ts::Const { name: "CHECK", value: CHECK },
	]
}

fn sidebar() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SIDEBAR_MENU_BUTTON_BASE",
			value: SIDEBAR_MENU_BUTTON_BASE,
		},
		Ts::Table {
			name: "sidebarMenuButtonSizeClasses",
			ty: "SidebarMenuButtonSize",
			entries: [(Size::Xs, "xs"), (Size::Sm, "sm"), (Size::Lg, "lg")]
				.into_iter()
				.map(|(s, k)| (k.to_string(), sidebar_menu_button_size_class(s).to_string()))
				.collect(),
		},
		Ts::Const {
			name: "SIDEBAR_WRAPPER",
			value: SIDEBAR_WRAPPER,
		},
		Ts::Const {
			name: "SIDEBAR_FLAT",
			value: SIDEBAR_FLAT,
		},
		Ts::Const {
			name: "SIDEBAR_INNER",
			value: SIDEBAR_INNER,
		},
		Ts::Const {
			name: "SIDEBAR_TRIGGER",
			value: SIDEBAR_TRIGGER,
		},
		Ts::Const {
			name: "SIDEBAR_RAIL",
			value: SIDEBAR_RAIL,
		},
		Ts::Const {
			name: "SIDEBAR_INSET",
			value: SIDEBAR_INSET,
		},
		Ts::Const {
			name: "SIDEBAR_HEADER",
			value: SIDEBAR_HEADER,
		},
		Ts::Const {
			name: "SIDEBAR_FOOTER",
			value: SIDEBAR_FOOTER,
		},
		Ts::Const {
			name: "SIDEBAR_SEPARATOR",
			value: SIDEBAR_SEPARATOR,
		},
		Ts::Const {
			name: "SIDEBAR_CONTENT",
			value: SIDEBAR_CONTENT,
		},
		Ts::Const {
			name: "SIDEBAR_GROUP",
			value: SIDEBAR_GROUP,
		},
		Ts::Const {
			name: "SIDEBAR_GROUP_LABEL",
			value: SIDEBAR_GROUP_LABEL,
		},
		Ts::Const {
			name: "SIDEBAR_GROUP_CONTENT",
			value: SIDEBAR_GROUP_CONTENT,
		},
		Ts::Const {
			name: "SIDEBAR_MENU",
			value: SIDEBAR_MENU,
		},
		Ts::Const {
			name: "SIDEBAR_MENU_ITEM",
			value: SIDEBAR_MENU_ITEM,
		},
	]
}

fn item() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ITEM_GROUP",
			value: ITEM_GROUP,
		},
		Ts::Const {
			name: "ITEM_SEPARATOR",
			value: ITEM_SEPARATOR,
		},
		Ts::Const {
			name: "ITEM_BASE",
			value: ITEM_BASE,
		},
		table::<ItemVariant>("itemVariants", "ItemVariant"),
		table::<ItemSize>("itemSizes", "ItemSize"),
		Ts::Const {
			name: "ITEM_MEDIA_BASE",
			value: ITEM_MEDIA_BASE,
		},
		table::<ItemMediaVariant>("itemMediaVariants", "ItemMediaVariant"),
		Ts::Const {
			name: "ITEM_CONTENT",
			value: ITEM_CONTENT,
		},
		Ts::Const {
			name: "ITEM_TITLE",
			value: ITEM_TITLE,
		},
		Ts::Const {
			name: "ITEM_DESCRIPTION",
			value: ITEM_DESCRIPTION,
		},
		Ts::Const {
			name: "ITEM_ACTIONS",
			value: ITEM_ACTIONS,
		},
		Ts::Const {
			name: "ITEM_HEADER",
			value: ITEM_HEADER,
		},
		Ts::Const {
			name: "ITEM_FOOTER",
			value: ITEM_FOOTER,
		},
	]
}

fn input_group() -> Vec<Ts> {
	use InputGroupButtonSize::{Sm, Xs};
	vec![
		Ts::Const {
			name: "INPUT_GROUP_BASE",
			value: INPUT_GROUP_BASE,
		},
		Ts::Const {
			name: "INPUT_GROUP_ADDON_BASE",
			value: INPUT_GROUP_ADDON_BASE,
		},
		table::<InputGroupAddonAlign>("inputGroupAddonAligns", "InputGroupAddonAlign"),
		Ts::Const {
			name: "INPUT_GROUP_BUTTON_BASE",
			value: INPUT_GROUP_BUTTON_BASE,
		},
		// Size × icon → shadcn's flat `xs`/`sm`/`icon-xs`/`icon-sm` keys.
		Ts::Table {
			name: "inputGroupButtonSizes",
			ty: "InputGroupButtonSize",
			entries: [(Xs, false, "xs"), (Sm, false, "sm"), (Xs, true, "icon-xs"), (Sm, true, "icon-sm")]
				.into_iter()
				.map(|(size, icon, key)| (key.to_string(), input_group_button_size_class(size, icon).to_string()))
				.collect(),
		},
		Ts::Const {
			name: "INPUT_GROUP_TEXT",
			value: INPUT_GROUP_TEXT,
		},
		Ts::Const {
			name: "INPUT_GROUP_INPUT_CONTROL",
			value: INPUT_GROUP_INPUT_CONTROL,
		},
		Ts::Const {
			name: "INPUT_GROUP_TEXTAREA_CONTROL",
			value: INPUT_GROUP_TEXTAREA_CONTROL,
		},
	]
}

fn field() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "FIELD_BASE",
			value: FIELD_BASE,
		},
		table::<FieldOrientation>("fieldOrientation", "FieldOrientation"),
		Ts::Const {
			name: "FIELD_SET",
			value: FIELD_SET,
		},
		Ts::Const {
			name: "FIELD_LEGEND",
			value: FIELD_LEGEND,
		},
		Ts::Const {
			name: "FIELD_GROUP",
			value: FIELD_GROUP,
		},
		Ts::Const {
			name: "FIELD_CONTENT",
			value: FIELD_CONTENT,
		},
		Ts::Const {
			name: "FIELD_LABEL",
			value: FIELD_LABEL,
		},
		Ts::Const {
			name: "FIELD_TITLE",
			value: FIELD_TITLE,
		},
		Ts::Const {
			name: "FIELD_DESCRIPTION",
			value: FIELD_DESCRIPTION,
		},
		Ts::Const {
			name: "FIELD_SEPARATOR",
			value: FIELD_SEPARATOR,
		},
		Ts::Const {
			name: "FIELD_SEPARATOR_LINE",
			value: FIELD_SEPARATOR_LINE,
		},
		Ts::Const {
			name: "FIELD_SEPARATOR_CONTENT",
			value: FIELD_SEPARATOR_CONTENT,
		},
		Ts::Const {
			name: "FIELD_ERROR",
			value: FIELD_ERROR,
		},
	]
}

fn sonner() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "TOAST_BASE",
			value: TOAST_BASE,
		},
		table::<ToastVariant>("toastVariantClasses", "ToastVariant"),
		Ts::Const {
			name: "TOASTER_BASE",
			value: TOASTER_BASE,
		},
		table::<ToastPosition>("positionClasses", "ToastPosition"),
		Ts::Const {
			name: "TOAST_CLOSE",
			value: TOAST_CLOSE,
		},
		Ts::Const {
			name: "TOAST_CONTENT",
			value: TOAST_CONTENT,
		},
		Ts::Const {
			name: "TOAST_TITLE",
			value: TOAST_TITLE,
		},
	]
}

fn dropdown_menu() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "DROPDOWN_MENU_CONTENT",
			value: DROPDOWN_MENU_CONTENT,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_SUB_CONTENT",
			value: DROPDOWN_MENU_SUB_CONTENT,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_ITEM",
			value: DROPDOWN_MENU_ITEM,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_CHECK_ITEM",
			value: DROPDOWN_MENU_CHECK_ITEM,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_SUB_TRIGGER",
			value: DROPDOWN_MENU_SUB_TRIGGER,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_ITEM_INDICATOR",
			value: DROPDOWN_MENU_ITEM_INDICATOR,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_LABEL",
			value: DROPDOWN_MENU_LABEL,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_SEPARATOR",
			value: DROPDOWN_MENU_SEPARATOR,
		},
		Ts::Const {
			name: "DROPDOWN_MENU_SHORTCUT",
			value: DROPDOWN_MENU_SHORTCUT,
		},
	]
}

fn context_menu() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "CONTEXT_MENU_CONTENT",
			value: CONTEXT_MENU_CONTENT,
		},
		Ts::Const {
			name: "CONTEXT_MENU_SUB_CONTENT",
			value: CONTEXT_MENU_SUB_CONTENT,
		},
		Ts::Const {
			name: "CONTEXT_MENU_ITEM",
			value: CONTEXT_MENU_ITEM,
		},
		Ts::Const {
			name: "CONTEXT_MENU_CHECK_ITEM",
			value: CONTEXT_MENU_CHECK_ITEM,
		},
		Ts::Const {
			name: "CONTEXT_MENU_SUB_TRIGGER",
			value: CONTEXT_MENU_SUB_TRIGGER,
		},
		Ts::Const {
			name: "CONTEXT_MENU_LABEL",
			value: CONTEXT_MENU_LABEL,
		},
		Ts::Const {
			name: "CONTEXT_MENU_SEPARATOR",
			value: CONTEXT_MENU_SEPARATOR,
		},
		Ts::Const {
			name: "CONTEXT_MENU_SHORTCUT",
			value: CONTEXT_MENU_SHORTCUT,
		},
	]
}

fn menubar() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "MENUBAR_ROOT",
			value: MENUBAR_ROOT,
		},
		Ts::Const {
			name: "MENUBAR_TRIGGER",
			value: MENUBAR_TRIGGER,
		},
		Ts::Const {
			name: "MENUBAR_CONTENT",
			value: MENUBAR_CONTENT,
		},
		Ts::Const {
			name: "MENUBAR_ITEM",
			value: MENUBAR_ITEM,
		},
		Ts::Const {
			name: "MENUBAR_CHECKBOX_ITEM",
			value: MENUBAR_CHECKBOX_ITEM,
		},
		Ts::Const {
			name: "MENUBAR_RADIO_ITEM",
			value: MENUBAR_RADIO_ITEM,
		},
		Ts::Const {
			name: "MENUBAR_ITEM_INDICATOR",
			value: MENUBAR_ITEM_INDICATOR,
		},
		Ts::Const {
			name: "MENUBAR_LABEL",
			value: MENUBAR_LABEL,
		},
		Ts::Const {
			name: "MENUBAR_SEPARATOR",
			value: MENUBAR_SEPARATOR,
		},
		Ts::Const {
			name: "MENUBAR_SHORTCUT",
			value: MENUBAR_SHORTCUT,
		},
		Ts::Const {
			name: "MENUBAR_SUB_TRIGGER",
			value: MENUBAR_SUB_TRIGGER,
		},
		Ts::Const {
			name: "MENUBAR_SUB_CONTENT",
			value: MENUBAR_SUB_CONTENT,
		},
	]
}

fn slider() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SLIDER_ROOT",
			value: SLIDER_ROOT,
		},
		Ts::Const {
			name: "SLIDER_TRACK",
			value: SLIDER_TRACK,
		},
		Ts::Const {
			name: "SLIDER_RANGE",
			value: SLIDER_RANGE,
		},
		Ts::Const {
			name: "SLIDER_THUMB",
			value: SLIDER_THUMB,
		},
	]
}

fn tabs() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "TABS_ROOT",
			value: TABS_ROOT,
		},
		Ts::Const {
			name: "TABS_LIST",
			value: TABS_LIST,
		},
		Ts::Const {
			name: "TABS_TRIGGER",
			value: TABS_TRIGGER,
		},
		Ts::Const {
			name: "TABS_CONTENT",
			value: TABS_CONTENT,
		},
	]
}

fn carousel() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "CAROUSEL_CONTENT_VIEWPORT",
			value: CAROUSEL_CONTENT_VIEWPORT,
		},
		Ts::Const {
			name: "CAROUSEL_CONTENT_TRACK",
			value: CAROUSEL_CONTENT_TRACK,
		},
		Ts::Const {
			name: "CAROUSEL_CONTENT_TRACK_HORIZONTAL",
			value: CAROUSEL_CONTENT_TRACK_HORIZONTAL,
		},
		Ts::Const {
			name: "CAROUSEL_CONTENT_TRACK_VERTICAL",
			value: CAROUSEL_CONTENT_TRACK_VERTICAL,
		},
		Ts::Const {
			name: "CAROUSEL_ITEM",
			value: CAROUSEL_ITEM,
		},
		Ts::Const {
			name: "CAROUSEL_ITEM_HORIZONTAL",
			value: CAROUSEL_ITEM_HORIZONTAL,
		},
		Ts::Const {
			name: "CAROUSEL_ITEM_VERTICAL",
			value: CAROUSEL_ITEM_VERTICAL,
		},
		Ts::Const {
			name: "CAROUSEL_NAV",
			value: CAROUSEL_NAV,
		},
		Ts::Const {
			name: "CAROUSEL_PREVIOUS_HORIZONTAL",
			value: CAROUSEL_PREVIOUS_HORIZONTAL,
		},
		Ts::Const {
			name: "CAROUSEL_PREVIOUS_VERTICAL",
			value: CAROUSEL_PREVIOUS_VERTICAL,
		},
		Ts::Const {
			name: "CAROUSEL_NEXT_HORIZONTAL",
			value: CAROUSEL_NEXT_HORIZONTAL,
		},
		Ts::Const {
			name: "CAROUSEL_NEXT_VERTICAL",
			value: CAROUSEL_NEXT_VERTICAL,
		},
		Ts::Const {
			name: "CAROUSEL_EDGE_FADE_PREV",
			value: CAROUSEL_EDGE_FADE_PREV,
		},
		Ts::Const {
			name: "CAROUSEL_EDGE_FADE_NEXT",
			value: CAROUSEL_EDGE_FADE_NEXT,
		},
	]
}

fn accordion() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ACCORDION_ITEM",
			value: ACCORDION_ITEM,
		},
		Ts::Const {
			name: "ACCORDION_HEADER",
			value: ACCORDION_HEADER,
		},
		Ts::Const {
			name: "ACCORDION_TRIGGER",
			value: ACCORDION_TRIGGER,
		},
		Ts::Const {
			name: "ACCORDION_CONTENT",
			value: ACCORDION_CONTENT,
		},
		Ts::Const {
			name: "ACCORDION_CONTENT_INNER",
			value: ACCORDION_CONTENT_INNER,
		},
	]
}

fn drawer() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "DRAWER_CONTENT_BASE",
			value: DRAWER_CONTENT_BASE,
		},
		table::<DrawerDirection>("drawerDirectionClasses", "DrawerDirection"),
		Ts::Const {
			name: "DRAWER_OVERLAY",
			value: DRAWER_OVERLAY,
		},
		Ts::Const {
			name: "DRAWER_HANDLE",
			value: DRAWER_HANDLE,
		},
		Ts::Const {
			name: "DRAWER_BODY",
			value: DRAWER_BODY,
		},
		Ts::Const {
			name: "DRAWER_HEADER",
			value: DRAWER_HEADER,
		},
		Ts::Const {
			name: "DRAWER_FOOTER",
			value: DRAWER_FOOTER,
		},
		Ts::Const {
			name: "DRAWER_TITLE",
			value: DRAWER_TITLE,
		},
		Ts::Const {
			name: "DRAWER_DESCRIPTION",
			value: DRAWER_DESCRIPTION,
		},
	]
}

fn input_otp() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "INPUT_OTP_CONTAINER",
			value: INPUT_OTP_CONTAINER,
		},
		Ts::Const {
			name: "INPUT_OTP_INPUT",
			value: INPUT_OTP_INPUT,
		},
		Ts::Const {
			name: "INPUT_OTP_GROUP",
			value: INPUT_OTP_GROUP,
		},
		Ts::Const {
			name: "INPUT_OTP_SLOT",
			value: INPUT_OTP_SLOT,
		},
		Ts::Const {
			name: "INPUT_OTP_SLOT_CARET_WRAPPER",
			value: INPUT_OTP_SLOT_CARET_WRAPPER,
		},
		Ts::Const {
			name: "INPUT_OTP_SLOT_CARET",
			value: INPUT_OTP_SLOT_CARET,
		},
	]
}

fn calendar() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "CALENDAR_ROOT",
			value: CALENDAR_ROOT,
		},
		Ts::Const {
			name: "CALENDAR_NAV_BUTTON",
			value: CALENDAR_NAV_BUTTON,
		},
		Ts::Const {
			name: "CALENDAR_NAV",
			value: CALENDAR_NAV,
		},
		Ts::Const {
			name: "CALENDAR_CAPTION",
			value: CALENDAR_CAPTION,
		},
		Ts::Const {
			name: "CALENDAR_GRID",
			value: CALENDAR_GRID,
		},
		Ts::Const {
			name: "CALENDAR_WEEKDAY_ROW",
			value: CALENDAR_WEEKDAY_ROW,
		},
		Ts::Const {
			name: "CALENDAR_WEEKDAY",
			value: CALENDAR_WEEKDAY,
		},
		Ts::Const {
			name: "CALENDAR_WEEK",
			value: CALENDAR_WEEK,
		},
		Ts::Const {
			name: "CALENDAR_DAY_EMPTY",
			value: CALENDAR_DAY_EMPTY,
		},
		Ts::Const {
			name: "CALENDAR_DAY_CELL",
			value: CALENDAR_DAY_CELL,
		},
		Ts::Const {
			name: "CALENDAR_DAY",
			value: CALENDAR_DAY,
		},
		Ts::Const {
			name: "CALENDAR_DAY_SELECTED",
			value: CALENDAR_DAY_SELECTED,
		},
		Ts::Const {
			name: "CALENDAR_DAY_TODAY",
			value: CALENDAR_DAY_TODAY,
		},
	]
}

fn date_time_picker() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "DATE_TIME_PICKER_TRIGGER",
			value: DATE_TIME_PICKER_TRIGGER,
		},
		Ts::Const {
			name: "DATE_TIME_PICKER_CONTENT",
			value: DATE_TIME_PICKER_CONTENT,
		},
		Ts::Const {
			name: "DATE_TIME_PICKER_TIME",
			value: DATE_TIME_PICKER_TIME,
		},
		Ts::Const {
			name: "DATE_TIME_PICKER_TIME_INPUT",
			value: DATE_TIME_PICKER_TIME_INPUT,
		},
		Ts::Const {
			name: "DATE_TIME_PICKER_TIME_SEPARATOR",
			value: DATE_TIME_PICKER_TIME_SEPARATOR,
		},
		Ts::Const {
			name: "DATE_TIME_PICKER_CLEAR",
			value: DATE_TIME_PICKER_CLEAR,
		},
	]
}

fn dialog() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "DIALOG_OVERLAY",
			value: DIALOG_OVERLAY,
		},
		Ts::Const {
			name: "DIALOG_CONTENT",
			value: DIALOG_CONTENT,
		},
		Ts::Const {
			name: "DIALOG_CLOSE",
			value: DIALOG_CLOSE,
		},
		Ts::Const {
			name: "DIALOG_HEADER",
			value: DIALOG_HEADER,
		},
		Ts::Const {
			name: "DIALOG_FOOTER",
			value: DIALOG_FOOTER,
		},
		Ts::Const {
			name: "DIALOG_TITLE",
			value: DIALOG_TITLE,
		},
		Ts::Const {
			name: "DIALOG_DESCRIPTION",
			value: DIALOG_DESCRIPTION,
		},
	]
}

fn command_palette() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "COMMAND_PALETTE_CONTENT",
			value: COMMAND_PALETTE_CONTENT,
		},
		Ts::Const {
			name: "COMMAND_PALETTE_TITLE",
			value: COMMAND_PALETTE_TITLE,
		},
		Ts::Const {
			name: "COMMAND_PALETTE_COMMAND",
			value: COMMAND_PALETTE_COMMAND,
		},
		Ts::Const {
			name: "COMMAND_PALETTE_LIST",
			value: COMMAND_PALETTE_LIST,
		},
		Ts::Const {
			name: "COMMAND_PALETTE_LABEL",
			value: COMMAND_PALETTE_LABEL,
		},
		Ts::Const {
			name: "COMMAND_PALETTE_HINTS",
			value: COMMAND_PALETTE_HINTS,
		},
		Ts::Const {
			name: "COMMAND_PALETTE_HINT",
			value: COMMAND_PALETTE_HINT,
		},
	]
}

fn command() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "COMMAND_ROOT",
			value: COMMAND_ROOT,
		},
		Ts::Const {
			name: "COMMAND_DIALOG_OVERLAY",
			value: COMMAND_DIALOG_OVERLAY,
		},
		Ts::Const {
			name: "COMMAND_DIALOG_CONTENT",
			value: COMMAND_DIALOG_CONTENT,
		},
		Ts::Const {
			name: "COMMAND_DIALOG_COMMAND",
			value: COMMAND_DIALOG_COMMAND,
		},
		Ts::Const {
			name: "COMMAND_INPUT_WRAPPER",
			value: COMMAND_INPUT_WRAPPER,
		},
		Ts::Const {
			name: "COMMAND_INPUT",
			value: COMMAND_INPUT,
		},
		Ts::Const {
			name: "COMMAND_LIST",
			value: COMMAND_LIST,
		},
		Ts::Const {
			name: "COMMAND_EMPTY",
			value: COMMAND_EMPTY,
		},
		Ts::Const {
			name: "COMMAND_GROUP",
			value: COMMAND_GROUP,
		},
		Ts::Const {
			name: "COMMAND_ITEM",
			value: COMMAND_ITEM,
		},
		Ts::Const {
			name: "COMMAND_SEPARATOR",
			value: COMMAND_SEPARATOR,
		},
		Ts::Const {
			name: "COMMAND_SHORTCUT",
			value: COMMAND_SHORTCUT,
		},
	]
}

fn data_table() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "TABLE_CONTAINER",
			value: TABLE_CONTAINER,
		},
		Ts::Const { name: "TABLE", value: TABLE },
		table::<TableVariant>("tableVariants", "TableVariant"),
		table::<TableDensity>("tableDensities", "TableDensity"),
		Ts::Const {
			name: "TABLE_CARD",
			value: TABLE_CARD,
		},
		Ts::Const {
			name: "TABLE_HEADER",
			value: TABLE_HEADER,
		},
		Ts::Const {
			name: "TABLE_BODY",
			value: TABLE_BODY,
		},
		Ts::Const {
			name: "TABLE_FOOTER",
			value: TABLE_FOOTER,
		},
		Ts::Const {
			name: "TABLE_ROW",
			value: TABLE_ROW,
		},
		Ts::Const {
			name: "TABLE_HEAD",
			value: TABLE_HEAD,
		},
		Ts::Const {
			name: "TABLE_CELL",
			value: TABLE_CELL,
		},
		table::<TableAlign>("tableAligns", "TableAlign"),
		Ts::Const {
			name: "TABLE_CAPTION",
			value: TABLE_CAPTION,
		},
	]
}

fn form() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "FORM_ITEM",
			value: FORM_ITEM,
		},
		Ts::Const {
			name: "FORM_LABEL",
			value: FORM_LABEL,
		},
		Ts::Const {
			name: "FORM_DESCRIPTION",
			value: FORM_DESCRIPTION,
		},
		Ts::Const {
			name: "FORM_MESSAGE",
			value: FORM_MESSAGE,
		},
	]
}

fn navigation_menu() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "NAVIGATION_MENU",
			value: NAVIGATION_MENU,
		},
		Ts::Const {
			name: "NAVIGATION_MENU_LIST",
			value: NAVIGATION_MENU_LIST,
		},
		Ts::Const {
			name: "NAVIGATION_MENU_ITEM",
			value: NAVIGATION_MENU_ITEM,
		},
		Ts::Const {
			name: "NAVIGATION_MENU_TRIGGER_STYLE",
			value: NAVIGATION_MENU_TRIGGER_STYLE,
		},
		Ts::Const {
			name: "NAVIGATION_MENU_CONTENT",
			value: NAVIGATION_MENU_CONTENT,
		},
		Ts::Const {
			name: "NAVIGATION_MENU_LINK",
			value: NAVIGATION_MENU_LINK,
		},
		Ts::Const {
			name: "NAVIGATION_MENU_INDICATOR",
			value: NAVIGATION_MENU_INDICATOR,
		},
		Ts::Const {
			name: "NAVIGATION_MENU_VIEWPORT",
			value: NAVIGATION_MENU_VIEWPORT,
		},
	]
}

fn hover_card() -> Vec<Ts> {
	vec![Ts::Const {
		name: "HOVER_CARD_CONTENT",
		value: HOVER_CARD_CONTENT,
	}]
}

fn progress() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "PROGRESS_TRACK",
			value: PROGRESS_TRACK,
		},
		Ts::Const {
			name: "PROGRESS_INDICATOR",
			value: PROGRESS_INDICATOR,
		},
	]
}

fn radio_group() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "RADIO_GROUP_ROOT",
			value: RADIO_GROUP_ROOT,
		},
		Ts::Const {
			name: "RADIO_GROUP_ITEM",
			value: RADIO_GROUP_ITEM,
		},
	]
}

fn card() -> Vec<Ts> {
	vec![
		Ts::Const { name: "CARD", value: CARD },
		Ts::Const {
			name: "CARD_HEADER",
			value: CARD_HEADER,
		},
		Ts::Const {
			name: "CARD_TITLE",
			value: CARD_TITLE,
		},
		Ts::Const {
			name: "CARD_DESCRIPTION",
			value: CARD_DESCRIPTION,
		},
		Ts::Const {
			name: "CARD_ACTION",
			value: CARD_ACTION,
		},
		Ts::Const {
			name: "CARD_CONTENT",
			value: CARD_CONTENT,
		},
		Ts::Const {
			name: "CARD_FOOTER",
			value: CARD_FOOTER,
		},
	]
}

fn breadcrumb() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "BREADCRUMB_LIST",
			value: BREADCRUMB_LIST,
		},
		Ts::Const {
			name: "BREADCRUMB_ITEM",
			value: BREADCRUMB_ITEM,
		},
		Ts::Const {
			name: "BREADCRUMB_LINK",
			value: BREADCRUMB_LINK,
		},
		Ts::Const {
			name: "BREADCRUMB_PAGE",
			value: BREADCRUMB_PAGE,
		},
		Ts::Const {
			name: "BREADCRUMB_SEPARATOR",
			value: BREADCRUMB_SEPARATOR,
		},
		Ts::Const {
			name: "BREADCRUMB_ELLIPSIS",
			value: BREADCRUMB_ELLIPSIS,
		},
	]
}

fn avatar() -> Vec<Ts> {
	vec![
		Ts::Const { name: "AVATAR", value: AVATAR },
		Ts::Const {
			name: "AVATAR_IMAGE",
			value: AVATAR_IMAGE,
		},
		Ts::Const {
			name: "AVATAR_FALLBACK",
			value: AVATAR_FALLBACK,
		},
		table::<AvatarShape>("avatarShapeClasses", "AvatarShape"),
		table::<AvatarSize>("avatarSizeClasses", "AvatarSize"),
	]
}

fn alert_dialog() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ALERT_DIALOG_OVERLAY",
			value: ALERT_DIALOG_OVERLAY,
		},
		Ts::Const {
			name: "ALERT_DIALOG_CONTENT",
			value: ALERT_DIALOG_CONTENT,
		},
		Ts::Const {
			name: "ALERT_DIALOG_HEADER",
			value: ALERT_DIALOG_HEADER,
		},
		Ts::Const {
			name: "ALERT_DIALOG_FOOTER",
			value: ALERT_DIALOG_FOOTER,
		},
		Ts::Const {
			name: "ALERT_DIALOG_TITLE",
			value: ALERT_DIALOG_TITLE,
		},
		Ts::Const {
			name: "ALERT_DIALOG_DESCRIPTION",
			value: ALERT_DIALOG_DESCRIPTION,
		},
	]
}

fn sheet() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SHEET_OVERLAY",
			value: SHEET_OVERLAY,
		},
		Ts::Const {
			name: "SHEET_CONTENT",
			value: SHEET_CONTENT,
		},
		Ts::Const {
			name: "SHEET_CLOSE",
			value: SHEET_CLOSE,
		},
		Ts::Const {
			name: "SHEET_SIDE_RIGHT",
			value: SHEET_SIDE_RIGHT,
		},
		Ts::Const {
			name: "SHEET_SIDE_LEFT",
			value: SHEET_SIDE_LEFT,
		},
		Ts::Const {
			name: "SHEET_SIDE_TOP",
			value: SHEET_SIDE_TOP,
		},
		Ts::Const {
			name: "SHEET_SIDE_BOTTOM",
			value: SHEET_SIDE_BOTTOM,
		},
		Ts::Const {
			name: "SHEET_HEADER",
			value: SHEET_HEADER,
		},
		Ts::Const {
			name: "SHEET_FOOTER",
			value: SHEET_FOOTER,
		},
		Ts::Const {
			name: "SHEET_TITLE",
			value: SHEET_TITLE,
		},
		Ts::Const {
			name: "SHEET_DESCRIPTION",
			value: SHEET_DESCRIPTION,
		},
	]
}

fn popover() -> Vec<Ts> {
	vec![Ts::Const {
		name: "POPOVER_CONTENT",
		value: POPOVER_CONTENT,
	}]
}

fn tooltip() -> Vec<Ts> {
	vec![Ts::Const {
		name: "TOOLTIP_CONTENT",
		value: TOOLTIP_CONTENT,
	}]
}

fn input() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "INPUT_BASE",
			value: INPUT_BASE,
		},
		form_size_table("inputSizeClasses", "InputSize", input_size_class),
	]
}

fn select() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SELECT_CONTENT_BOUNDS",
			value: SELECT_CONTENT_BOUNDS,
		},
		Ts::Const {
			name: "SELECT_ITEM",
			value: SELECT_ITEM,
		},
		Ts::Const {
			name: "SELECT_TRIGGER_BASE",
			value: SELECT_TRIGGER_BASE,
		},
		Ts::Const {
			name: "SELECT_CONTENT_SURFACE",
			value: SELECT_CONTENT_SURFACE,
		},
		Ts::Const {
			name: "SELECT_LIST",
			value: SELECT_LIST,
		},
		Ts::Const {
			name: "SELECT_LABEL",
			value: SELECT_LABEL,
		},
		Ts::Const {
			name: "SELECT_SEPARATOR",
			value: SELECT_SEPARATOR,
		},
		Ts::Const {
			name: "NATIVE_SELECT_BASE",
			value: NATIVE_SELECT_BASE,
		},
		form_size_table("selectTriggerSizeClasses", "SelectTriggerSize", select_trigger_size_class),
		form_size_table("nativeSelectSizeClasses", "NativeSelectSize", native_select_size_class),
	]
}

/// A form field's `fn(Size) -> &str` over the three steps a field has; the
/// outer two of the shared scale fold into these, so they get no key.
fn form_size_table(name: &'static str, ty: &'static str, class: fn(Size) -> &'static str) -> Ts {
	Ts::Table {
		name,
		ty,
		entries: [Size::Sm, Size::Md, Size::Touch].into_iter().map(|s| (s.as_ref().to_string(), class(s).to_string())).collect(),
	}
}

fn label() -> Vec<Ts> {
	vec![Ts::Const {
		name: "LABEL_BASE",
		value: LABEL_BASE,
	}]
}

fn skeleton() -> Vec<Ts> {
	vec![Ts::Const {
		name: "SKELETON_BASE",
		value: SKELETON_BASE,
	}]
}

fn textarea() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "TEXTAREA_BASE",
			value: TEXTAREA_BASE,
		},
		form_size_table("textareaSizeClasses", "TextareaSize", textarea_size_class),
	]
}

fn kbd() -> Vec<Ts> {
	vec![
		Ts::Const { name: "KBD_BASE", value: KBD_BASE },
		Ts::Const {
			name: "KBD_GROUP_BASE",
			value: KBD_GROUP_BASE,
		},
	]
}

fn container() -> Vec<Ts> {
	vec![Ts::Const {
		name: "CONTAINER_BASE",
		value: CONTAINER_BASE,
	}]
}

fn badge() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "BADGE_BASE",
			value: BADGE_BASE,
		},
		table::<BadgeVariant>("badgeVariants", "BadgeVariant"),
	]
}

fn icon_button() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ICON_BUTTON_BASE",
			value: ICON_BUTTON_BASE,
		},
		Ts::Const {
			name: "ICON_BUTTON_PRESSED",
			value: ICON_BUTTON_PRESSED,
		},
	]
}

fn status_item() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "STATUS_DOT_BASE",
			value: STATUS_DOT_BASE,
		},
		Ts::Const {
			name: "STATUS_ITEM_BASE",
			value: STATUS_ITEM_BASE,
		},
		Ts::Const {
			name: "STATUS_ITEM_VALUE",
			value: STATUS_ITEM_VALUE,
		},
		table::<StatusTone>("statusDotClasses", "StatusTone"),
	]
}

fn form_row() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "FORM_ROW_BASE",
			value: FORM_ROW_BASE,
		},
		Ts::Const {
			name: "FORM_ROW_DIVIDER",
			value: FORM_ROW_DIVIDER,
		},
		Ts::Const {
			name: "FORM_ROW_LABEL_COLUMN",
			value: FORM_ROW_LABEL_COLUMN,
		},
		Ts::Const {
			name: "FORM_ROW_LABEL",
			value: FORM_ROW_LABEL,
		},
		Ts::Const {
			name: "FORM_ROW_DESCRIPTION",
			value: FORM_ROW_DESCRIPTION,
		},
		Ts::Const {
			name: "FORM_ROW_CONTROL",
			value: FORM_ROW_CONTROL,
		},
		Ts::Const {
			name: "FORM_ROW_HINT",
			value: FORM_ROW_HINT,
		},
	]
}

fn uikit_settings() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SETTINGS_LAYOUT",
			value: SETTINGS_LAYOUT,
		},
		Ts::Const {
			name: "SETTINGS_NAV",
			value: SETTINGS_NAV,
		},
		Ts::Const {
			name: "SETTINGS_MAIN",
			value: SETTINGS_MAIN,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_BASE",
			value: SETTINGS_SECTION_BASE,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_HEADER",
			value: SETTINGS_SECTION_HEADER,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_HEADER_FLUSH",
			value: SETTINGS_SECTION_HEADER_FLUSH,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_HEADING",
			value: SETTINGS_SECTION_HEADING,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_TITLE",
			value: SETTINGS_SECTION_TITLE,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_DESCRIPTION",
			value: SETTINGS_SECTION_DESCRIPTION,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_ACTION",
			value: SETTINGS_SECTION_ACTION,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_BODY",
			value: SETTINGS_SECTION_BODY,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_BODY_FLUSH",
			value: SETTINGS_SECTION_BODY_FLUSH,
		},
		Ts::Const {
			name: "SETTINGS_SECTION_BODY_PADDED",
			value: SETTINGS_SECTION_BODY_PADDED,
		},
		Ts::Const {
			name: "SETTINGS_ROW_INLINE",
			value: SETTINGS_ROW_INLINE,
		},
		table::<SettingsSectionTone>("settingsSectionToneClasses", "SettingsSectionTone"),
	]
}

fn system_banner() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SYSTEM_BANNER_BASE",
			value: SYSTEM_BANNER_BASE,
		},
		Ts::Const {
			name: "SYSTEM_BANNER_ICON",
			value: SYSTEM_BANNER_ICON,
		},
		Ts::Const {
			name: "SYSTEM_BANNER_GLYPH",
			value: SYSTEM_BANNER_GLYPH,
		},
		Ts::Const {
			name: "SYSTEM_BANNER_MESSAGE",
			value: SYSTEM_BANNER_MESSAGE,
		},
		Ts::Const {
			name: "SYSTEM_BANNER_TITLE",
			value: SYSTEM_BANNER_TITLE,
		},
		Ts::Const {
			name: "SYSTEM_BANNER_BODY_AFTER_TITLE",
			value: SYSTEM_BANNER_BODY_AFTER_TITLE,
		},
		Ts::Const {
			name: "SYSTEM_BANNER_ACTION",
			value: SYSTEM_BANNER_ACTION,
		},
		Ts::Const {
			name: "SYSTEM_BANNER_DISMISS",
			value: SYSTEM_BANNER_DISMISS,
		},
		table::<SystemBannerTone>("systemBannerIconToneClasses", "SystemBannerTone"),
	]
}

fn section_label() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SECTION_LABEL_BASE",
			value: SECTION_LABEL_BASE,
		},
		table::<SectionLabelTone>("sectionLabelToneClasses", "SectionLabelTone"),
	]
}

fn workspace_tab() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "WORKSPACE_TABS_BASE",
			value: WORKSPACE_TABS_BASE,
		},
		Ts::Const {
			name: "WORKSPACE_TAB_BASE",
			value: WORKSPACE_TAB_BASE,
		},
		Ts::Const {
			name: "WORKSPACE_TAB_SELECTED",
			value: WORKSPACE_TAB_SELECTED,
		},
		Ts::Const {
			name: "WORKSPACE_TAB_TRIGGER",
			value: WORKSPACE_TAB_TRIGGER,
		},
		Ts::Const {
			name: "WORKSPACE_TAB_LABEL",
			value: WORKSPACE_TAB_LABEL,
		},
		Ts::Const {
			name: "WORKSPACE_TAB_CLOSE",
			value: WORKSPACE_TAB_CLOSE,
		},
		Ts::Const {
			name: "WORKSPACE_TAB_SPACER",
			value: WORKSPACE_TAB_SPACER,
		},
	]
}

fn account_list() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ACCOUNT_LIST_BASE",
			value: ACCOUNT_LIST_BASE,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_BASE",
			value: ACCOUNT_LIST_ITEM_BASE,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_CHECKED",
			value: ACCOUNT_LIST_ITEM_CHECKED,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_UNCHECKED",
			value: ACCOUNT_LIST_ITEM_UNCHECKED,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_DISABLED",
			value: ACCOUNT_LIST_ITEM_DISABLED,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_RADIO",
			value: ACCOUNT_LIST_ITEM_RADIO,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_AVATAR",
			value: ACCOUNT_LIST_ITEM_AVATAR,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_AVATAR_FALLBACK",
			value: ACCOUNT_LIST_ITEM_AVATAR_FALLBACK,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_TEXT",
			value: ACCOUNT_LIST_ITEM_TEXT,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_NAME",
			value: ACCOUNT_LIST_ITEM_NAME,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_HANDLE",
			value: ACCOUNT_LIST_ITEM_HANDLE,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_BADGES",
			value: ACCOUNT_LIST_ITEM_BADGES,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_FIGURE",
			value: ACCOUNT_LIST_ITEM_FIGURE,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_CHECK",
			value: ACCOUNT_LIST_ITEM_CHECK,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_CHECK_HIDDEN",
			value: ACCOUNT_LIST_ITEM_CHECK_HIDDEN,
		},
		Ts::Const {
			name: "ACCOUNT_LIST_ITEM_ACTIONS",
			value: ACCOUNT_LIST_ITEM_ACTIONS,
		},
	]
}

fn account_settings_dialog() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_CONTENT",
			value: ACCOUNT_SETTINGS_DIALOG_CONTENT,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_TABS",
			value: ACCOUNT_SETTINGS_DIALOG_TABS,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_SIDEBAR",
			value: ACCOUNT_SETTINGS_DIALOG_SIDEBAR,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_LIST",
			value: ACCOUNT_SETTINGS_DIALOG_LIST,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_ENTRY",
			value: ACCOUNT_SETTINGS_DIALOG_ENTRY,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_FOOTER",
			value: ACCOUNT_SETTINGS_DIALOG_FOOTER,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_PANE",
			value: ACCOUNT_SETTINGS_DIALOG_PANE,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_HEADING",
			value: ACCOUNT_SETTINGS_DIALOG_HEADING,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_TITLE",
			value: ACCOUNT_SETTINGS_DIALOG_TITLE,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_DESCRIPTION",
			value: ACCOUNT_SETTINGS_DIALOG_DESCRIPTION,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_PANEL",
			value: ACCOUNT_SETTINGS_DIALOG_PANEL,
		},
		Ts::Const {
			name: "ACCOUNT_SETTINGS_DIALOG_CLOSE",
			value: ACCOUNT_SETTINGS_DIALOG_CLOSE,
		},
	]
}

fn popover_form() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "POPOVER_FORM_CONTENT",
			value: POPOVER_FORM_CONTENT,
		},
		Ts::Const {
			name: "POPOVER_FORM_FORM",
			value: POPOVER_FORM_FORM,
		},
		Ts::Const {
			name: "POPOVER_FORM_TITLE",
			value: POPOVER_FORM_TITLE,
		},
		Ts::Const {
			name: "POPOVER_FORM_FIELDS",
			value: POPOVER_FORM_FIELDS,
		},
		Ts::Const {
			name: "POPOVER_FORM_ACTIONS",
			value: POPOVER_FORM_ACTIONS,
		},
	]
}

fn confirm_dialog() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "CONFIRM_DIALOG_CONTENT",
			value: CONFIRM_DIALOG_CONTENT,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_HEADER",
			value: CONFIRM_DIALOG_HEADER,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_ICON",
			value: CONFIRM_DIALOG_ICON,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_TEXT",
			value: CONFIRM_DIALOG_TEXT,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_TITLE",
			value: CONFIRM_DIALOG_TITLE,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_DESCRIPTION",
			value: CONFIRM_DIALOG_DESCRIPTION,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_ERROR",
			value: CONFIRM_DIALOG_ERROR,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_FOOTER",
			value: CONFIRM_DIALOG_FOOTER,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_PENDING",
			value: CONFIRM_DIALOG_PENDING,
		},
		Ts::Const {
			name: "CONFIRM_DIALOG_SPINNER",
			value: CONFIRM_DIALOG_SPINNER,
		},
		table::<ConfirmDialogTone>("confirmDialogIconToneClasses", "ConfirmDialogTone"),
	]
}

fn key_value() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "KEY_VALUE_BASE",
			value: KEY_VALUE_BASE,
		},
		Ts::Const {
			name: "KEY_VALUE_LABEL",
			value: KEY_VALUE_LABEL,
		},
		Ts::Const {
			name: "KEY_VALUE_ROW",
			value: KEY_VALUE_ROW,
		},
		Ts::Const {
			name: "KEY_VALUE_VALUE",
			value: KEY_VALUE_VALUE,
		},
		Ts::Const {
			name: "KEY_VALUE_COPY",
			value: KEY_VALUE_COPY,
		},
		Ts::Const {
			name: "KEY_VALUE_STATUS",
			value: KEY_VALUE_STATUS,
		},
	]
}

fn top_bar() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "TOP_BAR_BASE",
			value: TOP_BAR_BASE,
		},
		Ts::Const {
			name: "TOP_BAR_START",
			value: TOP_BAR_START,
		},
		Ts::Const {
			name: "TOP_BAR_END",
			value: TOP_BAR_END,
		},
	]
}

fn smart_header() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SMART_HEADER_BASE",
			value: SMART_HEADER_BASE,
		},
		Ts::Const {
			name: "SMART_HEADER_BAR",
			value: SMART_HEADER_BAR,
		},
		Ts::Const {
			name: "SMART_HEADER_TOOLBAR",
			value: SMART_HEADER_TOOLBAR,
		},
		Ts::Const {
			name: "SMART_HEADER_CONTEXT",
			value: SMART_HEADER_CONTEXT,
		},
	]
}

fn aside() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ASIDE_HEADER_BASE",
			value: ASIDE_HEADER_BASE,
		},
		Ts::Const {
			name: "ASIDE_HEADER_TITLE",
			value: ASIDE_HEADER_TITLE,
		},
		Ts::Const {
			name: "ASIDE_HEADER_BACK_ICON",
			value: ASIDE_HEADER_BACK_ICON,
		},
		Ts::Const {
			name: "ASIDE_HEADER_CLOSE_ICON",
			value: ASIDE_HEADER_CLOSE_ICON,
		},
		Ts::Const {
			name: "ASIDE_RAIL_BASE",
			value: ASIDE_RAIL_BASE,
		},
		Ts::Const {
			name: "ASIDE_RAIL_SPACER",
			value: ASIDE_RAIL_SPACER,
		},
		Ts::Const {
			name: "ASIDE_BODY",
			value: ASIDE_BODY,
		},
		Ts::Const {
			name: "ASIDE_FOOTER",
			value: ASIDE_FOOTER,
		},
		Ts::Const {
			name: "ASIDE_PANEL_DOCKED",
			value: ASIDE_PANEL_DOCKED,
		},
		Ts::Const {
			name: "ASIDE_PANEL_DRAWER",
			value: ASIDE_PANEL_DRAWER,
		},
		Ts::Const {
			name: "ASIDE_PANEL_SHEET",
			value: ASIDE_PANEL_SHEET,
		},
		Ts::Const {
			name: "ASIDE_PANEL_WIDE_ONLY",
			value: ASIDE_PANEL_WIDE_ONLY,
		},
		Ts::Const {
			name: "ASIDE_PANEL_WIDE_ONLY_WRAPPER",
			value: ASIDE_PANEL_WIDE_ONLY_WRAPPER,
		},
		Ts::Const {
			name: "ASIDE_PANEL_NARROW_ONLY",
			value: ASIDE_PANEL_NARROW_ONLY,
		},
		table::<AsidePanelSide>("asidePanelSideClasses", "AsidePanelSide"),
	]
}

fn header_controls() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "HEADER_BALANCE_BASE",
			value: HEADER_BALANCE_BASE,
		},
		Ts::Const {
			name: "HEADER_BALANCE_INTERACTIVE",
			value: HEADER_BALANCE_INTERACTIVE,
		},
		Ts::Const {
			name: "HEADER_BALANCE_LABEL",
			value: HEADER_BALANCE_LABEL,
		},
		Ts::Const {
			name: "HEADER_BALANCE_SETTLED",
			value: HEADER_BALANCE_SETTLED,
		},
		Ts::Const {
			name: "HEADER_BALANCE_SKELETON",
			value: HEADER_BALANCE_SKELETON,
		},
		Ts::Const {
			name: "HEADER_BALANCE_FIGURE",
			value: HEADER_BALANCE_FIGURE,
		},
		Ts::Const {
			name: "HEADER_BALANCE_CHEVRON",
			value: HEADER_BALANCE_CHEVRON,
		},
		table::<HeaderBalanceTone>("headerBalanceToneClasses", "HeaderBalanceTone"),
		Ts::Const {
			name: "NOTIFICATION_BELL_TRIGGER",
			value: NOTIFICATION_BELL_TRIGGER,
		},
		Ts::Const {
			name: "NOTIFICATION_DOT",
			value: NOTIFICATION_DOT,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_CONTENT",
			value: NOTIFICATION_BELL_CONTENT,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_EMPTY",
			value: NOTIFICATION_BELL_EMPTY,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_LIST",
			value: NOTIFICATION_BELL_LIST,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM",
			value: NOTIFICATION_BELL_ITEM,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_DOT",
			value: NOTIFICATION_BELL_ITEM_DOT,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_DOT_UNREAD",
			value: NOTIFICATION_BELL_ITEM_DOT_UNREAD,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_DOT_READ",
			value: NOTIFICATION_BELL_ITEM_DOT_READ,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_BODY",
			value: NOTIFICATION_BELL_ITEM_BODY,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_TITLE",
			value: NOTIFICATION_BELL_ITEM_TITLE,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_TITLE_UNREAD",
			value: NOTIFICATION_BELL_ITEM_TITLE_UNREAD,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_DESCRIPTION",
			value: NOTIFICATION_BELL_ITEM_DESCRIPTION,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_ITEM_TIME",
			value: NOTIFICATION_BELL_ITEM_TIME,
		},
		Ts::Const {
			name: "NOTIFICATION_BELL_FOOTER",
			value: NOTIFICATION_BELL_FOOTER,
		},
		Ts::Const {
			name: "HEADER_SEARCH_BASE",
			value: HEADER_SEARCH_BASE,
		},
		Ts::Const {
			name: "HEADER_SEARCH_PLACEHOLDER",
			value: HEADER_SEARCH_PLACEHOLDER,
		},
	]
}

fn focus() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "FILLED_FOCUS_RING",
			value: FILLED_FOCUS_RING,
		},
		Ts::Const {
			name: "OPTION_FOCUS_RING",
			value: OPTION_FOCUS_RING,
		},
	]
}

fn switch() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SWITCH_BASE",
			value: SWITCH_BASE,
		},
		Ts::Const {
			name: "SWITCH_THUMB",
			value: SWITCH_THUMB,
		},
	]
}

fn checkbox() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "CHECKBOX_BASE",
			value: CHECKBOX_BASE,
		},
		Ts::Const {
			name: "CHECKBOX_INDICATOR",
			value: CHECKBOX_INDICATOR,
		},
	]
}

fn separator() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SEPARATOR_BASE",
			value: SEPARATOR_BASE,
		},
		table::<Orientation>("separatorOrientations", "SeparatorOrientation"),
	]
}

fn button_group() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "BUTTON_GROUP_BASE",
			value: BUTTON_GROUP_BASE,
		},
		table::<ButtonGroupOrientation>("buttonGroupOrientationClasses", "ButtonGroupOrientation"),
		Ts::Const {
			name: "BUTTON_GROUP_TEXT_BASE",
			value: BUTTON_GROUP_TEXT_BASE,
		},
		Ts::Const {
			name: "BUTTON_GROUP_SEPARATOR_BASE",
			value: BUTTON_GROUP_SEPARATOR_BASE,
		},
	]
}

fn empty() -> Vec<Ts> {
	vec![
		Ts::Const { name: "EMPTY", value: EMPTY },
		Ts::Const {
			name: "EMPTY_HEADER",
			value: EMPTY_HEADER,
		},
		Ts::Const {
			name: "EMPTY_MEDIA_BASE",
			value: EMPTY_MEDIA_BASE,
		},
		table::<EmptyMediaVariant>("emptyMediaVariants", "EmptyMediaVariant"),
		Ts::Const {
			name: "EMPTY_TITLE",
			value: EMPTY_TITLE,
		},
		Ts::Const {
			name: "EMPTY_DESCRIPTION",
			value: EMPTY_DESCRIPTION,
		},
		Ts::Const {
			name: "EMPTY_CONTENT",
			value: EMPTY_CONTENT,
		},
	]
}

fn scroll_area() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "SCROLL_AREA_VIEWPORT",
			value: SCROLL_AREA_VIEWPORT,
		},
		Ts::Const {
			name: "SCROLL_AREA_THUMB",
			value: SCROLL_AREA_THUMB,
		},
		Ts::Const {
			name: "SCROLLBAR_BASE",
			value: SCROLLBAR_BASE,
		},
		table::<ScrollBarOrientation>("scrollBarOrientations", "ScrollBarOrientation"),
	]
}

fn alert() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "ALERT_BASE",
			value: ALERT_BASE,
		},
		table::<AlertVariant>("alertVariants", "AlertVariant"),
		Ts::Const {
			name: "ALERT_TITLE",
			value: ALERT_TITLE,
		},
		Ts::Const {
			name: "ALERT_DESCRIPTION",
			value: ALERT_DESCRIPTION,
		},
	]
}

fn toggle() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "TOGGLE_BASE",
			value: TOGGLE_BASE,
		},
		table::<ToggleVariant>("toggleVariantClasses", "ToggleVariant"),
		Ts::Table {
			name: "toggleSizeClasses",
			ty: "ToggleSize",
			entries: [(Size::Md, "md"), (Size::Xs, "xs"), (Size::Sm, "sm"), (Size::Lg, "lg"), (Size::Xl, "xl")]
				.into_iter()
				.map(|(s, k)| (k.to_string(), toggle_size_class(s).to_string()))
				.collect(),
		},
	]
}

fn button() -> Vec<Ts> {
	vec![
		Ts::Const {
			name: "BUTTON_BASE",
			value: BUTTON_BASE,
		},
		table::<ButtonVariant>("buttonVariantClasses", "ButtonVariant"),
		// Size and shape are two axes, so they are two tables over the same keys —
		// not one table whose keys fuse them (`"icon-sm"`) and elide the default
		// (`"icon"` meaning icon-at-Md). See docs/spec/variants.md.
		size_table("buttonSizeClasses", "ButtonSize", |s| button_size_class(s, false)),
		size_table("buttonIconSizeClasses", "ButtonIconSize", |s| button_size_class(s, true)),
	]
}

/// Any `fn(Size) -> &str` over the whole control scale, keyed by the size name.
fn size_table(name: &'static str, ty: &'static str, class: fn(Size) -> &'static str) -> Ts {
	Ts::Table {
		name,
		ty,
		entries: Size::iter().map(|s| (s.as_ref().to_string(), class(s).to_string())).collect(),
	}
}

fn accent() -> Vec<Ts> {
	vec![
		accent_table("accentTextClasses", "Accent", accent_text_class),
		accent_table("accentFillClasses", "AccentFill", accent_fill_class),
		accent_table("accentOutlineClasses", "AccentOutline", accent_outline_class),
	]
}

fn accent_table(name: &'static str, ty: &'static str, class: fn(Accent) -> &'static str) -> Ts {
	Ts::Table {
		name,
		ty,
		entries: Accent::iter().map(|a| (a.as_ref().to_string(), class(a).to_string())).collect(),
	}
}

fn locales() -> Vec<Ts> {
	use ev_lib::i18n::{DEFAULT_LOCALE, LOCALES};
	vec![
		Ts::Value {
			name: "LOCALES",
			value: LOCALES.iter().map(|l| l.code()).collect(),
		},
		// `Locale` derives off this one, so the labels table is what carries the
		// union — a locale nobody can name in a switcher does not exist.
		Ts::Table {
			name: "LOCALE_LABELS",
			ty: "Locale",
			entries: LOCALES.iter().map(|l| (l.code().to_string(), l.label().to_string())).collect(),
		},
		Ts::Const {
			name: "DEFAULT_LOCALE",
			value: DEFAULT_LOCALE.code(),
		},
	]
}

fn e164() -> Vec<Ts> {
	use ev_lib::types::{COUNTRY_CODES, PhoneNumber, PhoneNumberError};
	vec![
		Ts::Union {
			name: "PHONE_NUMBER_ERROR_CODES",
			ty: "PhoneNumberErrorCode",
			items: [
				PhoneNumberError::Empty,
				PhoneNumberError::NoPlusPrefix,
				PhoneNumberError::NonDigitChars,
				PhoneNumberError::TooShort { actual: 0 },
				PhoneNumberError::TooLong { actual: 0 },
				PhoneNumberError::InvalidCountryCode,
			]
			.iter()
			.map(PhoneNumberError::code)
			.collect(),
		},
		Ts::Value {
			name: "COUNTRY_CODES",
			value: COUNTRY_CODES.iter().copied().collect(),
		},
		Ts::Value {
			name: "MIN_DIGITS",
			value: PhoneNumber::MIN_DIGITS.into(),
		},
		Ts::Value {
			name: "MAX_DIGITS",
			value: PhoneNumber::MAX_DIGITS.into(),
		},
	]
}

fn experiments() -> Vec<Ts> {
	use ev_lib::experiments::{COOKIE_MAX_AGE_SECS, COOKIE_PREFIX, FNV_OFFSET_BASIS, FNV_PRIME};
	vec![
		Ts::Value {
			name: "FNV_OFFSET_BASIS",
			value: FNV_OFFSET_BASIS.into(),
		},
		Ts::Value {
			name: "FNV_PRIME",
			value: FNV_PRIME.into(),
		},
		// The TS port lets a caller override it; this is the default.
		Ts::Const {
			name: "DEFAULT_COOKIE_PREFIX",
			value: COOKIE_PREFIX,
		},
		Ts::Value {
			name: "COOKIE_MAX_AGE",
			value: COOKIE_MAX_AGE_SECS.into(),
		},
	]
}

fn settings() -> Vec<Ts> {
	use ev_lib::settings::{
		DEFAULT_PROFILE, EX_CONFIG, PROFILE_VAR,
		presets::{AppEnv, Posthog, Sentry},
	};
	vec![
		Ts::Const {
			name: "PROFILE_VAR",
			value: PROFILE_VAR,
		},
		Ts::Const {
			name: "DEFAULT_PROFILE",
			value: DEFAULT_PROFILE,
		},
		Ts::Value {
			name: "EX_CONFIG",
			value: EX_CONFIG.into(),
		},
		// Keyed by the TS `presets` member each group backs.
		Ts::Value {
			name: "PRESET_VARS",
			value: serde_json::json!({
				"posthog": Posthog::var_names(),
				"sentry": Sentry::var_names(),
				"appEnv": AppEnv::var_names(),
			}),
		},
	]
}

fn analytics() -> Vec<Ts> {
	vec![Ts::Const {
		name: "DEFAULT_HOST",
		value: ev_lib::analytics::DEFAULT_HOST,
	}]
}

fn kitstart() -> Vec<Ts> {
	use ev_lib::kitstart::{
		AI_CRAWLERS, GONE, GONE_HEADER, HONEYPOT_FIELD, HOST_MARK, LANG_COOKIE, LANG_COOKIE_MAX_AGE, LEGACY_HONEYPOT_FIELDS, MIN_FILL_MS, NON_PAGE_ROUTES, OG_IMAGE_SIZE,
		RATE_LIMIT_OVERFLOW_KEY, RATING_MAX_AGE_DAYS, RENDERED_AT_FIELD, THANKS,
	};
	vec![
		Ts::Const {
			name: "HONEYPOT_FIELD",
			value: HONEYPOT_FIELD,
		},
		Ts::Value {
			name: "LEGACY_HONEYPOT_FIELDS",
			value: LEGACY_HONEYPOT_FIELDS.into(),
		},
		Ts::Const {
			name: "RENDERED_AT_FIELD",
			value: RENDERED_AT_FIELD,
		},
		Ts::Value {
			name: "MIN_FILL_MS",
			value: MIN_FILL_MS.into(),
		},
		Ts::Const {
			name: "RATE_LIMIT_OVERFLOW_KEY",
			value: RATE_LIMIT_OVERFLOW_KEY,
		},
		Ts::Const { name: "THANKS", value: THANKS },
		Ts::Const {
			name: "LANG_COOKIE",
			value: LANG_COOKIE,
		},
		Ts::Value {
			name: "LANG_COOKIE_MAX_AGE",
			value: LANG_COOKIE_MAX_AGE.into(),
		},
		Ts::Const {
			name: "HOST_MARK",
			value: HOST_MARK,
		},
		Ts::Const { name: "GONE", value: GONE },
		Ts::Const {
			name: "GONE_HEADER",
			value: GONE_HEADER,
		},
		Ts::Value {
			name: "NON_PAGE_ROUTES",
			value: NON_PAGE_ROUTES.as_slice().into(),
		},
		Ts::Value {
			name: "OG_IMAGE_SIZE",
			value: serde_json::json!(OG_IMAGE_SIZE),
		},
		Ts::Value {
			name: "RATING_MAX_AGE_DAYS",
			value: RATING_MAX_AGE_DAYS.into(),
		},
		Ts::Value {
			name: "AI_CRAWLERS",
			value: AI_CRAWLERS.as_slice().into(),
		},
	]
}
