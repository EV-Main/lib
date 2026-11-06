pub const KEY_VALUE_BASE: &str = "flex min-w-0 flex-col gap-0.5";

pub const KEY_VALUE_LABEL: &str = "text-body-s text-ink-mid";

pub const KEY_VALUE_ROW: &str = "flex min-w-0 items-center gap-1.5";

pub const KEY_VALUE_VALUE: &str = "min-w-0 truncate text-body-l text-ink";

pub const KEY_VALUE_COPY: &str =
	"inline-flex size-6 shrink-0 items-center justify-center rounded-control text-ink-soft outline-none hover:bg-hover hover:text-ink focus-visible:ring-2 focus-visible:ring-ring";

pub const KEY_VALUE_STATUS: &str = "sr-only";

/// How long (ms) a copy's check mark and "Copied" announcement stay.
pub const KEY_VALUE_COPIED_MS: i32 = 2000;

/// The `KeyValue`'s own strings. English by default; a caller translates by overriding fields.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(serde::Serialize), serde(rename_all = "camelCase"))]
pub struct KeyValueLabels {
	/// The copy button; unset, it is "Copy {label}", or "Copy" when the label is not plain text.
	#[cfg_attr(feature = "ts_gen", serde(skip_serializing_if = "Option::is_none"))]
	pub copy: Option<String>,
	/// The polite announcement after a copy.
	pub copied: String,
}
impl Default for KeyValueLabels {
	fn default() -> Self {
		Self {
			copy: None,
			copied: "Copied".to_owned(),
		}
	}
}
