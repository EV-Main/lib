//! What only the Rust half has. The behaviour both halves share is pinned by
//! `tests/fixtures/i18n/`, run from `rust/tests/i18n.rs`.

use super::{LOCALES, Locale, Messages, Translator};

#[test]
fn every_locale_labels_itself_in_its_own_language() {
	// A reader who cannot read the current language cannot read "Russian" either.
	assert_eq!(Locale::Ru.label(), "Русский");
	assert_eq!(Locale::Vi.label(), "Tiếng Việt");
	assert_eq!(Locale::De.label(), "Deutsch");
}

#[test]
fn declared_categories_match_what_select_can_return() {
	// The policy's plural check is only sound if these two agree.
	for locale in LOCALES {
		let declared = locale.plural_categories();
		for n in 0..=130 {
			let selected = locale.select_plural(f64::from(n));
			assert!(declared.contains(&selected), "{locale} selected {selected} for {n}, which it does not declare");
		}
	}
}

#[test]
fn the_macro_registers_every_site_it_renders() {
	let t = Translator::new(Messages::new(), Locale::En);
	assert_eq!(crate::t!(t, "tests.macro.plain", "Plain"), "Plain");
	assert_eq!(crate::t!(t, "tests.macro.values", "{n, plural, one {# item} other {# items}}", n = 4), "4 items");

	// Extraction is linking, not parsing: the branch below never runs, and its
	// key is in the catalogue anyway. That is the whole reason for `inventory`.
	if std::hint::black_box(false) {
		let _ = crate::t!(t, "tests.macro.never_rendered", "Never rendered");
	}

	let catalogue = super::catalogue().expect("no key is registered with two different Englishes");
	assert_eq!(catalogue.get("tests.macro.plain").map(String::as_str), Some("Plain"));
	assert_eq!(catalogue.get("tests.macro.never_rendered").map(String::as_str), Some("Never rendered"));
}
