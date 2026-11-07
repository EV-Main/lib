//! What of the TS is not a plain `ev_lib_classes` const (those are listed by
//! `build.rs`): the variant tables, the few key-quirk `Ts::Table` recipes, and
//! the registries the non-uikit packages share.

use ev_lib::ts_gen::Ts;
use ev_lib_classes::*;
use strum::IntoEnumIterator;
use tailwind_fuse::AsTailwindClass;

/// Per uikit module, merged after its consts. A module named here need not have a classes file.
pub fn uikit() -> Vec<(&'static str, Vec<Ts>)> {
	vec![
		("size", vec![table::<Size>("sizeClasses", "Size")]),
		("tone", vec![table::<Tone>("toneClasses", "Tone")]),
		("slot", vec![union::<Slot>("DATA_SLOTS", "DataSlot")]),
		("icons", vec![union::<Icon>("ICON_NAMES", "IconName"), icons()]),
		("footer", vec![Ts::types::<(FooterLinkGroup, FooterLink, FooterOffice)>()]),
		("popover-form", vec![labels::<PopoverFormLabels>("POPOVER_FORM_LABELS")]),
		("key-value", vec![labels::<KeyValueLabels>("KEY_VALUE_LABELS")]),
		("date-time-picker", vec![labels::<DateTimePickerLabels>("DATE_TIME_PICKER_LABELS")]),
		("command-palette", vec![labels::<CommandPaletteLabels>("COMMAND_PALETTE_LABELS")]),
		("chat-composer", vec![labels::<ChatComposerLabels>("CHAT_COMPOSER_LABELS")]),
		("account-settings-dialog", vec![labels::<AccountSettingsDialogLabels>("ACCOUNT_SETTINGS_DIALOG_LABELS")]),
		("accordion", vec![union::<AccordionType>("ACCORDION_TYPES", "AccordionType")]),
		(
			"accent",
			vec![
				accent_table("accentTextClasses", "Accent", accent_text_class),
				accent_table("accentFillClasses", "AccentFill", accent_fill_class),
				accent_table("accentOutlineClasses", "AccentOutline", accent_outline_class),
			],
		),
		(
			"aside",
			vec![
				labels::<AsideHeaderLabels>("ASIDE_HEADER_LABELS"),
				table::<AsidePanelSide>("asidePanelSideClasses", "AsidePanelSide"),
				union::<AsidePanelMode>("ASIDE_PANEL_MODES", "AsidePanelMode"),
			],
		),
		("band", vec![table::<Polarity>("polarityClasses", "Polarity"), table::<Surface>("surfaceClasses", "Surface")]),
		("button", vec![table::<ButtonVariant>("buttonVariantClasses", "ButtonVariant")]),
		("button-group", vec![table::<ButtonGroupOrientation>("buttonGroupOrientationClasses", "ButtonGroupOrientation")]),
		(
			"chat-message",
			vec![
				table::<ChatBubbleVariant>("chatBubbleVariantClasses", "ChatBubbleVariant"),
				table::<ChatStatusState>("chatStatusDotClasses", "ChatStatusState"),
				table::<ChatToolState>("chatToolMetaClasses", "ChatToolState"),
				union::<ChatMessageRole>("CHAT_MESSAGE_ROLES", "ChatMessageRole"),
				union::<ChatAttachmentType>("CHAT_ATTACHMENT_TYPES", "ChatAttachmentType"),
				union::<ChatAttachmentState>("CHAT_ATTACHMENT_STATES", "ChatAttachmentState"),
			],
		),
		("confirm-dialog", vec![labels::<ConfirmDialogLabels>("CONFIRM_DIALOG_LABELS")]),
		("drawer", vec![table::<DrawerDirection>("drawerDirectionClasses", "DrawerDirection")]),
		("empty", vec![table::<EmptyMediaVariant>("emptyMediaVariants", "EmptyMediaVariant")]),
		(
			"field",
			vec![
				table::<FieldOrientation>("fieldOrientation", "FieldOrientation"),
				union::<FieldLegendVariant>("FIELD_LEGEND_VARIANTS", "FieldLegendVariant"),
			],
		),
		(
			"icon-button",
			vec![
				union::<IconButtonVariant>("ICON_BUTTON_VARIANTS", "IconButtonVariant"),
				Ts::Table {
					name: "iconButtonSizeClasses",
					ty: "IconButtonSize",
					entries: [Size::Sm, Size::Md, Size::Touch]
						.into_iter()
						.map(|s| (s.as_ref().to_string(), icon_button_size_class(s).to_string()))
						.collect(),
				},
			],
		),
		("input", vec![form_size_table("inputSizeClasses", "InputSize", input_size_class)]),
		(
			"input-group",
			vec![
				table::<InputGroupAddonAlign>("inputGroupAddonAligns", "InputGroupAddonAlign"),
				// Size × icon → shadcn's flat `xs`/`sm`/`icon-xs`/`icon-sm` keys.
				Ts::Table {
					name: "inputGroupButtonSizes",
					ty: "InputGroupButtonSize",
					entries: [
						(InputGroupButtonSize::Xs, false, "xs"),
						(InputGroupButtonSize::Sm, false, "sm"),
						(InputGroupButtonSize::Xs, true, "icon-xs"),
						(InputGroupButtonSize::Sm, true, "icon-sm"),
					]
					.into_iter()
					.map(|(size, icon, key)| (key.to_string(), input_group_button_size_class(size, icon).to_string()))
					.collect(),
				},
			],
		),
		(
			"item",
			vec![
				table::<ItemVariant>("itemVariants", "ItemVariant"),
				table::<ItemSize>("itemSizes", "ItemSize"),
				table::<ItemMediaVariant>("itemMediaVariants", "ItemMediaVariant"),
			],
		),
		("menu", vec![union::<MenuItemVariant>("MENU_ITEM_VARIANTS", "MenuItemVariant")]),
		("orientation", vec![union::<Orientation>("ORIENTATIONS", "Orientation")]),
		(
			"primitives",
			vec![union::<Side>("SIDES", "Side"), union::<RovingOrientation>("ROVING_ORIENTATIONS", "RovingOrientation")],
		),
		("scroll-area", vec![table::<ScrollBarOrientation>("scrollBarOrientations", "ScrollBarOrientation")]),
		("section-label", vec![table::<SectionLabelTone>("sectionLabelToneClasses", "SectionLabelTone")]),
		(
			"select",
			vec![
				form_size_table("selectTriggerSizeClasses", "SelectTriggerSize", select_trigger_size_class),
				form_size_table("nativeSelectSizeClasses", "NativeSelectSize", native_select_size_class),
			],
		),
		(
			"separator",
			vec![Ts::Table {
				name: "separatorOrientations",
				ty: "SeparatorOrientation",
				entries: Orientation::iter().map(|o| (o.as_ref().to_string(), separator_orientation_class(o).to_string())).collect(),
			}],
		),
		(
			"sidebar",
			vec![
				union::<SidebarSide>("SIDEBAR_SIDES", "SidebarSide"),
				union::<SidebarVariant>("SIDEBAR_VARIANTS", "SidebarVariant"),
				union::<SidebarCollapsible>("SIDEBAR_COLLAPSIBLES", "SidebarCollapsible"),
				Ts::Table {
					name: "sidebarMenuButtonSizeClasses",
					ty: "SidebarMenuButtonSize",
					entries: [(Size::Xs, "xs"), (Size::Sm, "sm"), (Size::Lg, "lg")]
						.into_iter()
						.map(|(s, k)| (k.to_string(), sidebar_menu_button_size_class(s).to_string()))
						.collect(),
				},
			],
		),
		("smart-header", vec![union::<SmartHeaderState>("SMART_HEADER_STATES", "SmartHeaderState")]),
		("sonner", vec![table::<ToastPosition>("positionClasses", "ToastPosition")]),
		("status-item", vec![table::<StatusTone>("statusDotClasses", "StatusTone")]),
		("system-banner", vec![labels::<SystemBannerLabels>("SYSTEM_BANNER_LABELS")]),
		(
			"table",
			vec![
				table::<TableVariant>("tableVariants", "TableVariant"),
				table::<TableDensity>("tableDensities", "TableDensity"),
				table::<TableAlign>("tableAligns", "TableAlign"),
			],
		),
		("textarea", vec![form_size_table("textareaSizeClasses", "TextareaSize", textarea_size_class)]),
		("toggle-group", vec![union::<ToggleGroupType>("TOGGLE_GROUP_TYPES", "ToggleGroupType")]),
		("toggle", vec![table::<ToggleVariant>("toggleVariantClasses", "ToggleVariant")]),
	]
}

/// Registries shared by more than the uikit: `(package, module, exports)` into
/// `ts/{package}/src/generated/{module}.ts`. Not class tables — these are the
/// same data the Rust libraries own, which the TS ports used to re-type by hand.
pub fn shared() -> Vec<(&'static str, &'static str, Vec<Ts>)> {
	vec![
		("i18n", "locales", locales()),
		("i18n", "policy", policy()),
		("experiments", "contract", experiments()),
		("settings", "contract", settings()),
		("analytics", "host", analytics()),
		("mfe", "manifest", mfe()),
	]
}

/// Any `TwVariant` enum → a `{ kebab-key: "class" }` table. Base classes are split
/// into a `*_BASE` const, so `as_class()` yields the per-variant string only.
fn table<T>(name: &'static str, ty: &'static str) -> Ts
where
	T: IntoEnumIterator + AsRef<str> + AsTailwindClass, {
	Ts::Table {
		name,
		ty,
		entries: T::iter().map(|v| (v.as_ref().to_string(), v.as_class().to_string())).collect(),
	}
}

/// A Labels struct's English defaults, which the TS spreads its overrides over.
fn labels<T: Default + serde::Serialize>(name: &'static str) -> Ts {
	Ts::Value {
		name,
		value: serde_json::to_value(T::default()).expect("labels are plain strings"),
	}
}

/// Every icon's children as `[tag, attributes]` pairs, which the TS renders
/// inside each site's own `<svg>` frame.
fn icons() -> Ts {
	let shape = |s: &Shape| match *s {
		Shape::Path { d } => serde_json::json!(["path", { "d": d }]),
		Shape::Circle { cx, cy, r } => serde_json::json!(["circle", { "cx": cx, "cy": cy, "r": r }]),
		Shape::Rect { x, y, width, height, rx, ry } => {
			let mut attrs = serde_json::json!({ "x": x, "y": y, "width": width, "height": height, "rx": rx });
			if let Some(ry) = ry {
				attrs["ry"] = ry.into();
			}
			serde_json::json!(["rect", attrs])
		}
	};
	Ts::Value {
		name: "ICONS",
		value: Icon::iter()
			.map(|i| (i.as_ref().to_owned(), i.shapes().iter().map(shape).collect()))
			.collect::<serde_json::Map<_, _>>()
			.into(),
	}
}

/// A closed word set: the array and its type, in the enum's order.
fn union<T>(name: &'static str, ty: &'static str) -> Ts
where
	T: IntoEnumIterator + Into<&'static str>, {
	Ts::Union {
		name,
		ty,
		items: T::iter().map(Into::into).collect(),
	}
}

/// A form field's `fn(Size) -> &str` over the three steps a field has; the
/// outer two of the shared scale fold into these, so they get no key.
fn form_size_table(name: &'static str, ty: &'static str, class: fn(Size) -> &'static str) -> Ts {
	Ts::Table {
		name,
		ty,
		entries: [Size::Sm, Size::Md, Size::Lg].into_iter().map(|s| (s.as_ref().to_string(), class(s).to_string())).collect(),
	}
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

fn policy() -> Vec<Ts> {
	use ev_lib::i18n::policy::RejectionReason;
	vec![Ts::Union {
		name: "REJECTION_REASONS",
		ty: "RejectionReason",
		items: RejectionReason::iter().map(RejectionReason::as_str).collect(),
	}]
}

fn experiments() -> Vec<Ts> {
	use ev_lib::experiments::{COOKIE_MAX_AGE_SECS, COOKIE_PREFIX, Experiment, FNV_OFFSET_BASIS, FNV_PRIME};
	vec![
		Ts::types::<Experiment>(),
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
	vec![
		Ts::Const {
			name: "DEFAULT_HOST",
			value: ev_lib::analytics::DEFAULT_HOST,
		},
		Ts::Const {
			name: "LIB_MARKER",
			value: ev_lib::analytics::LIB_MARKER,
		},
	]
}

fn mfe() -> Vec<Ts> {
	use ev_lib::mfe::{MfeKind, MfeManifest};
	vec![
		Ts::Union {
			name: "MFE_KINDS",
			ty: "MfeKind",
			items: MfeKind::iter().map(MfeKind::as_str).collect(),
		},
		Ts::types::<MfeManifest>(),
	]
}
