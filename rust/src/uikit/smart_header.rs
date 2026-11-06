/// Which rows a `SmartHeader` shows around its bar. Mirrors the TS
/// `SmartHeaderState`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, strum::AsRefStr)]
#[strum(serialize_all = "kebab-case")]
pub enum SmartHeaderState {
	/// The bar, the toolbar and the context line.
	#[default]
	Default,
	/// The toolbar folds into the bar as `collapsed`.
	Collapsed,
	/// A banner row above the bar.
	Banner,
	/// The bar with the `palette` under it.
	Search,
}
