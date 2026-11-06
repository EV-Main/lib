//! The contact-link vectors `@evinvest/marketing` runs too.
#![cfg(feature = "marketing")]

use ev_lib::marketing::{ContactChannel, contact_channel, tel_href, whatsapp_href};
use serde_json::Value;

fn fixture() -> Value {
	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/marketing/contact.json");
	serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))).expect("contact.json is JSON")
}

fn cases<'a>(fixture: &'a Value, key: &str) -> &'a [Value] {
	fixture[key].as_array().unwrap_or_else(|| panic!("contact.json has no `{key}` list"))
}

#[test]
fn channels() {
	let fixture = fixture();
	for case in cases(&fixture, "channel") {
		let href = case[0].as_str().expect("href is a string");
		assert_eq!(contact_channel(href).map(ContactChannel::as_str), case[1].as_str(), "{href:?}");
	}
}

#[test]
fn tel_hrefs() {
	let fixture = fixture();
	for case in cases(&fixture, "tel") {
		assert_eq!(tel_href(case[0].as_str().expect("phone")), case[1].as_str().expect("href"), "{case}");
	}
}

#[test]
fn whatsapp_hrefs() {
	let fixture = fixture();
	for case in cases(&fixture, "whatsapp") {
		assert_eq!(whatsapp_href(case[0].as_str().expect("phone"), case[1].as_str()), case[2].as_str().expect("href"), "{case}");
	}
}
