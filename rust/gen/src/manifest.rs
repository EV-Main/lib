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
		("accent", vec![
			accent_table("accentTextClasses", "Accent", accent_text_class),
			accent_table("accentFillClasses", "AccentFill", accent_fill_class),
			accent_table("accentOutlineClasses", "AccentOutline", accent_outline_class),
		]),
		("alert", vec![table::<AlertVariant>("alertVariants", "AlertVariant")]),
		("aside", vec![table::<AsidePanelSide>("asidePanelSideClasses", "AsidePanelSide")]),
		("badge", vec![table::<BadgeVariant>("badgeVariants", "BadgeVariant")]),
		("band", vec![table::<Polarity>("polarityClasses", "Polarity"), table::<Surface>("surfaceClasses", "Surface")]),
		("button", vec![
			table::<ButtonVariant>("buttonVariantClasses", "ButtonVariant"),
			// Size and shape are two axes, so they are two tables over the same keys —
			// not one table whose keys fuse them (`"icon-sm"`) and elide the default
			// (`"icon"` meaning icon-at-Md). See docs/spec/variants.md.
			size_table("buttonSizeClasses", "ButtonSize", |s| button_size_class(s, false)),
			size_table("buttonIconSizeClasses", "ButtonIconSize", |s| button_size_class(s, true)),
		]),
		("button-group", vec![table::<ButtonGroupOrientation>("buttonGroupOrientationClasses", "ButtonGroupOrientation")]),
		("chat-message", vec![
			table::<ChatBubbleVariant>("chatBubbleVariantClasses", "ChatBubbleVariant"),
			table::<ChatStatusState>("chatStatusDotClasses", "ChatStatusState"),
			table::<ChatToolState>("chatToolMetaClasses", "ChatToolState"),
		]),
		("confirm-dialog", vec![table::<ConfirmDialogTone>("confirmDialogIconToneClasses", "ConfirmDialogTone")]),
		("drawer", vec![table::<DrawerDirection>("drawerDirectionClasses", "DrawerDirection")]),
		("empty", vec![table::<EmptyMediaVariant>("emptyMediaVariants", "EmptyMediaVariant")]),
		("field", vec![table::<FieldOrientation>("fieldOrientation", "FieldOrientation")]),
		("header-controls", vec![table::<HeaderBalanceTone>("headerBalanceToneClasses", "HeaderBalanceTone")]),
		("icon-button", vec![Ts::Table {
			name: "iconButtonSizeClasses",
			ty: "IconButtonSize",
			entries: [Size::Sm, Size::Md, Size::Touch].into_iter().map(|s| (s.as_ref().to_string(), icon_button_size_class(s).to_string())).collect(),
		}]),
		("input", vec![form_size_table("inputSizeClasses", "InputSize", input_size_class)]),
		("input-group", vec![
			table::<InputGroupAddonAlign>("inputGroupAddonAligns", "InputGroupAddonAlign"),
			// Size × icon → shadcn's flat `xs`/`sm`/`icon-xs`/`icon-sm` keys.
			Ts::Table {
				name: "inputGroupButtonSizes",
				ty: "InputGroupButtonSize",
				entries: [(InputGroupButtonSize::Xs, false, "xs"), (InputGroupButtonSize::Sm, false, "sm"), (InputGroupButtonSize::Xs, true, "icon-xs"), (InputGroupButtonSize::Sm, true, "icon-sm")]
					.into_iter()
					.map(|(size, icon, key)| (key.to_string(), input_group_button_size_class(size, icon).to_string()))
					.collect(),
			},
		]),
		("item", vec![
			table::<ItemVariant>("itemVariants", "ItemVariant"),
			table::<ItemSize>("itemSizes", "ItemSize"),
			table::<ItemMediaVariant>("itemMediaVariants", "ItemMediaVariant"),
		]),
		("scroll-area", vec![table::<ScrollBarOrientation>("scrollBarOrientations", "ScrollBarOrientation")]),
		("section-label", vec![table::<SectionLabelTone>("sectionLabelToneClasses", "SectionLabelTone")]),
		("select", vec![
			form_size_table("selectTriggerSizeClasses", "SelectTriggerSize", select_trigger_size_class),
			form_size_table("nativeSelectSizeClasses", "NativeSelectSize", native_select_size_class),
		]),
		("separator", vec![table::<Orientation>("separatorOrientations", "SeparatorOrientation")]),
		("settings", vec![table::<SettingsSectionTone>("settingsSectionToneClasses", "SettingsSectionTone")]),
		("sidebar", vec![Ts::Table {
			name: "sidebarMenuButtonSizeClasses",
			ty: "SidebarMenuButtonSize",
			entries: [(Size::Xs, "xs"), (Size::Sm, "sm"), (Size::Lg, "lg")].into_iter().map(|(s, k)| (k.to_string(), sidebar_menu_button_size_class(s).to_string())).collect(),
		}]),
		("sonner", vec![table::<ToastVariant>("toastVariantClasses", "ToastVariant"), table::<ToastPosition>("positionClasses", "ToastPosition")]),
		("status-item", vec![table::<StatusTone>("statusDotClasses", "StatusTone")]),
		("system-banner", vec![
			table::<SystemBannerTone>("systemBannerIconToneClasses", "SystemBannerTone"),
			Ts::Table {
				name: "systemBannerToneClasses",
				ty: "SystemBannerFill",
				entries: SystemBannerTone::iter().map(|t| (t.as_ref().to_string(), system_banner_tone_class(&t).to_string())).collect(),
			},
		]),
		("table", vec![
			table::<TableVariant>("tableVariants", "TableVariant"),
			table::<TableDensity>("tableDensities", "TableDensity"),
			table::<TableAlign>("tableAligns", "TableAlign"),
		]),
		("textarea", vec![form_size_table("textareaSizeClasses", "TextareaSize", textarea_size_class)]),
		("toggle", vec![
			table::<ToggleVariant>("toggleVariantClasses", "ToggleVariant"),
			Ts::Table {
				name: "toggleSizeClasses",
				ty: "ToggleSize",
				entries: [(Size::Md, "md"), (Size::Xs, "xs"), (Size::Sm, "sm"), (Size::Lg, "lg"), (Size::Xl, "xl")]
					.into_iter()
					.map(|(s, k)| (k.to_string(), toggle_size_class(s).to_string()))
					.collect(),
			},
		]),
	]
}

/// Registries shared by more than the uikit: `(package, module, exports)` into
/// `ts/{package}/src/generated/{module}.ts`. Not class tables — these are the
/// same data the Rust libraries own, which the TS ports used to re-type by hand.
pub fn shared() -> Vec<(&'static str, &'static str, Vec<Ts>)> {
	vec![
		("i18n", "locales", locales()),
		("experiments", "contract", experiments()),
		("settings", "contract", settings()),
		("analytics", "host", analytics()),
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

/// Any `fn(Size) -> &str` over the whole control scale, keyed by the size name.
fn size_table(name: &'static str, ty: &'static str, class: fn(Size) -> &'static str) -> Ts {
	Ts::Table {
		name,
		ty,
		entries: Size::iter().map(|s| (s.as_ref().to_string(), class(s).to_string())).collect(),
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
