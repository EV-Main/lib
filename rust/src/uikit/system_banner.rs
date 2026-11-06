/// The `SystemBanner`'s own strings; every one defaults to English.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SystemBannerLabels {
	/// `aria-label` of the close button; "Dismiss".
	pub dismiss: Option<String>,
}
