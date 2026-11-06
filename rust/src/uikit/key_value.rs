/// The `KeyValue`'s own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct KeyValueLabels {
	/// The copy button; "Copy {label}", or just "Copy" when the label is not
	/// plain text.
	pub copy: Option<String>,
	/// The polite announcement after a copy; "Copied".
	pub copied: Option<String>,
}
