/// Whether a `FieldLegend` reads as a fieldset legend or as a field label.
#[derive(Clone, Default, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum FieldLegendVariant {
	#[default]
	Legend,
	Label,
}
