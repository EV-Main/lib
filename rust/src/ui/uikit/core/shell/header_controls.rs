use ev_lib_classes::copy;
/// One entry of a `NotificationBell`. Mirrors the TS `HeaderNotification`;
/// the TS fields take any node, these take text, like `CommandPaletteItem`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HeaderNotification {
	/// Reported to `on_select`.
	pub id: String,
	pub title: String,
	pub description: Option<String>,
	/// Already formatted: "2m ago".
	pub time: Option<String>,
	pub unread: bool,
}

/// The TS `APPLE` test: Mac, iPhone, iPad and iPod name their platform so.
#[cfg(any(test, all(target_arch = "wasm32", feature = "wasm")))]
pub(crate) fn platform_shortcut(platform: &str) -> &'static str {
	if ["Mac", "iPhone", "iPad", "iPod"].iter().any(|p| platform.contains(p)) {
		copy::COMMAND_K
	} else {
		copy::CTRL_K
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn apple_platforms_get_the_command_key() {
		assert_eq!(platform_shortcut("MacIntel"), "⌘K");
		assert_eq!(platform_shortcut("iPhone"), "⌘K");
		assert_eq!(platform_shortcut("Win32"), "Ctrl K");
		assert_eq!(platform_shortcut("Linux x86_64"), "Ctrl K");
		assert_eq!(platform_shortcut(""), "Ctrl K");
	}
}
