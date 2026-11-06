//! The i18n vectors `@evinvest/i18n` runs too.
#![cfg(feature = "i18n")]

use ev_lib::i18n::{
	LOCALES, Locale, LocaleRegistry, LocaleRegistryConfig, MessageValue, MessageValues, Messages, RegistryError, Translator, format_message, locale_alternates, locale_path, negotiate,
	policy::{MissingContentPolicy, Rejection, TranslatedCatalogue, TranslatedEntry, audit, available_in, resolve_catalogue},
	split_locale_path,
};
use serde_json::Value;

fn fixture(name: &str) -> Value {
	let path = format!("{}/../tests/fixtures/i18n/{name}.json", env!("CARGO_MANIFEST_DIR"));
	serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"))).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn cases<'a>(fixture: &'a Value, key: &str) -> &'a [Value] {
	fixture[key].as_array().unwrap_or_else(|| panic!("fixture has no `{key}` list"))
}

fn str_of(v: &Value) -> &str {
	v.as_str().unwrap_or_else(|| panic!("{v} is not a string"))
}

fn locale(v: &Value) -> Locale {
	Locale::parse(str_of(v)).unwrap_or_else(|| panic!("{v} is not a published locale"))
}

fn locales(v: &Value) -> Vec<Locale> {
	v.as_array().expect("a locale list").iter().map(locale).collect()
}

fn messages(v: &Value) -> Messages {
	v.as_object().expect("a catalogue").iter().map(|(k, v)| (k.clone(), str_of(v).to_owned())).collect()
}

fn values(v: &Value) -> MessageValues {
	v.as_object()
		.expect("message values")
		.iter()
		.map(|(k, v)| {
			let value = match v {
				Value::Number(n) => MessageValue::Num(n.as_f64().expect("finite")),
				Value::String(s) => MessageValue::Str(s.clone()),
				other => panic!("{other} is not a message value"),
			};
			(k.clone(), value)
		})
		.collect()
}

// ── format ───────────────────────────────────────────────────────────────────

#[test]
fn formats() {
	let fixture = fixture("format");
	for case in cases(&fixture, "format") {
		assert_eq!(format_message(str_of(&case[0]), locale(&case[1]), &values(&case[2])), str_of(&case[3]), "{case}");
	}
}

#[test]
fn plural_categories() {
	let fixture = fixture("format");
	for case in cases(&fixture, "plural") {
		let n = case[1].as_f64().expect("count");
		assert_eq!(locale(&case[0]).select_plural(n).as_str(), str_of(&case[2]), "{case}");
	}
}

#[test]
fn numbers() {
	let fixture = fixture("format");
	for case in cases(&fixture, "number") {
		let n = case[1].as_f64().expect("number");
		assert_eq!(locale(&case[0]).format_number(n), str_of(&case[2]), "{case}");
	}
}

#[test]
fn translates() {
	let fixture = fixture("format");
	for case in cases(&fixture, "translate") {
		let tr = Translator::new(messages(&case[1]), locale(&case[0]));
		assert_eq!(tr.tv(str_of(&case[2]), str_of(&case[3]), &values(&case[4])), str_of(&case[5]), "{case}");
	}
}

// ── policy ───────────────────────────────────────────────────────────────────

fn translated(v: &Value) -> TranslatedCatalogue {
	v.as_object()
		.expect("a translated catalogue")
		.iter()
		.map(|(k, e)| {
			(
				k.clone(),
				TranslatedEntry {
					en: str_of(&e["en"]).to_owned(),
					t: str_of(&e["t"]).to_owned(),
				},
			)
		})
		.collect()
}

fn rejection(r: &Rejection) -> Value {
	serde_json::json!([r.key, r.reason.as_str(), r.detail])
}

#[test]
fn resolves_catalogues() {
	let fixture = fixture("policy");
	for case in cases(&fixture, "resolve") {
		let resolved = resolve_catalogue(locale(&case["locale"]), &messages(&case["source"]), &translated(&case["translated"]));
		assert_eq!(resolved.messages, messages(&case["messages"]), "{case}");
		assert_eq!(Value::from(resolved.rejected.iter().map(rejection).collect::<Vec<_>>()), case["rejected"], "{case}");
		assert_eq!(Value::from(resolved.missing), case["missing"], "{case}");
		assert_eq!(resolved.coverage, case["coverage"].as_f64().expect("coverage"), "{case}");
	}
}

#[test]
fn audits() {
	let fixture = fixture("policy");
	for case in cases(&fixture, "audit") {
		let resolved: Vec<_> = case["catalogues"]
			.as_array()
			.expect("catalogues")
			.iter()
			.map(|c| resolve_catalogue(locale(&c[0]), &messages(&c[1]), &translated(&c[2])))
			.collect();
		let (ok, report) = audit(&resolved, case["floor"].as_f64().expect("floor"));
		assert_eq!(report, str_of(&case["report"]), "{case}");
		assert_eq!(Some(ok), case["ok"].as_bool(), "{case}");
	}
}

#[test]
fn hides_untranslated_content() {
	let fixture = fixture("policy");
	for case in cases(&fixture, "available") {
		let items: Vec<(&str, Vec<Locale>)> = case[1].as_array().expect("items").iter().map(|i| (str_of(&i[0]), locales(&i[1]))).collect();
		let policy = match str_of(&case[2]) {
			"hide" => MissingContentPolicy::Hide,
			"fallback" => MissingContentPolicy::Fallback,
			other => panic!("unknown policy {other}"),
		};
		let shown: Vec<&str> = available_in(locale(&case[0]), &items, |i| i.1.as_slice(), policy).into_iter().map(|i| i.0).collect();
		assert_eq!(Value::from(shown), case[3], "{case}");
	}
}

// ── registry ─────────────────────────────────────────────────────────────────

fn config(v: &Value) -> LocaleRegistryConfig {
	let labels = &v["labels"];
	LocaleRegistryConfig {
		locales: v["locales"]
			.as_array()
			.expect("locales")
			.iter()
			.map(|l| (str_of(l).to_owned(), str_of(&labels[str_of(l)]).to_owned()))
			.collect(),
		default: str_of(&v["default"]).to_owned(),
		prefix_default_locale: v["prefixDefaultLocale"].as_bool().expect("prefixDefaultLocale"),
		hreflang: v["hreflang"].as_object().expect("hreflang").iter().map(|(l, t)| (l.clone(), str_of(t).to_owned())).collect(),
	}
}

/// `ev` is the generated five locales: the free functions, and a registry over
/// the same list so the two Rust copies of the URL contract are held to one set.
fn registry(fixture: &Value, name: &str) -> LocaleRegistry {
	let config = if name == "ev" {
		LocaleRegistryConfig {
			locales: LOCALES.iter().map(|l| (l.code().to_owned(), l.label().to_owned())).collect(),
			default: "en".to_owned(),
			prefix_default_locale: false,
			hreflang: Vec::new(),
		}
	} else {
		config(&fixture["registries"][name])
	};
	LocaleRegistry::try_new(config).unwrap_or_else(|e| panic!("registry {name}: {e}"))
}

fn pairs(v: &Value) -> Vec<(String, String)> {
	v.as_array().expect("pairs").iter().map(|p| (str_of(&p[0]).to_owned(), str_of(&p[1]).to_owned())).collect()
}

#[test]
fn recognises_locales() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "isLocale") {
		let (name, value, expected) = (str_of(&case[0]), str_of(&case[1]), case[2].as_bool());
		if name == "ev" {
			assert_eq!(Some(Locale::parse(value).is_some()), expected, "{case}");
		}
		assert_eq!(Some(registry(&fixture, name).is_locale(value)), expected, "{case}");
	}
}

#[test]
fn locale_paths() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "localePath") {
		let (name, code, path, expected) = (str_of(&case[0]), str_of(&case[1]), str_of(&case[2]), str_of(&case[3]));
		if name == "ev" {
			assert_eq!(locale_path(locale(&case[1]), path), expected, "{case}");
		}
		assert_eq!(registry(&fixture, name).locale_path(code, path), expected, "{case}");
	}
}

#[test]
fn splits_locale_paths() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "splitLocalePath") {
		let (name, pathname) = (str_of(&case[0]), str_of(&case[1]));
		let expected = (str_of(&case[2]), str_of(&case[3]).to_owned());
		if name == "ev" {
			let (l, path) = split_locale_path(pathname);
			assert_eq!((l.code(), path), expected, "{case}");
		}
		assert_eq!(registry(&fixture, name).split_locale_path(pathname), expected, "{case}");
	}
}

#[test]
fn split_inverts_locale_path() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "roundTrip") {
		let name = str_of(&case[0]);
		let reg = registry(&fixture, name);
		for path in case[1].as_array().expect("paths").iter().map(str_of) {
			for code in reg.locales() {
				assert_eq!(reg.split_locale_path(&reg.locale_path(code, path)), (code, path.to_owned()), "{name} {code} {path}");
			}
			if name == "ev" {
				for l in LOCALES {
					assert_eq!(split_locale_path(&locale_path(l, path)), (l, path.to_owned()), "{l} {path}");
				}
			}
		}
	}
}

#[test]
fn locale_alternate_sets() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "localeAlternates") {
		let (name, path, expected) = (str_of(&case[0]), str_of(&case[1]), pairs(&case[3]));
		if name == "ev" {
			let subset = if case[2].is_null() { LOCALES.to_vec() } else { locales(&case[2]) };
			let got: Vec<(String, String)> = locale_alternates(path, &subset).into_iter().map(|(l, p)| (l.code().to_owned(), p)).collect();
			assert_eq!(got, expected, "{case}");
		}
		if case[2].is_null() {
			let got: Vec<(String, String)> = registry(&fixture, name).locale_alternates(path).into_iter().map(|(l, p)| (l.to_owned(), p)).collect();
			assert_eq!(got, expected, "{case}");
		}
	}
}

#[test]
fn hreflang_tags() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "hreflangOf") {
		assert_eq!(registry(&fixture, str_of(&case[0])).hreflang_of(str_of(&case[1])), str_of(&case[2]), "{case}");
	}
}

#[test]
fn language_alternate_sets() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "languageAlternates") {
		let got = registry(&fixture, str_of(&case[0])).language_alternates(str_of(&case[1]), str_of(&case[2]));
		assert_eq!(got, pairs(&case[3]), "{case}");
	}
}

#[test]
fn negotiates() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "negotiate") {
		let (name, header, expected) = (str_of(&case[0]), case[1].as_str(), str_of(&case[3]));
		if name == "ev" {
			let subset = if case[2].is_null() { LOCALES.to_vec() } else { locales(&case[2]) };
			assert_eq!(negotiate(header, &subset).code(), expected, "{case}");
		}
		if case[2].is_null() {
			assert_eq!(registry(&fixture, name).negotiate(header), expected, "{case}");
		}
	}
}

#[test]
fn refuses_ambiguous_registries() {
	let fixture = fixture("registry");
	for case in cases(&fixture, "invalid") {
		let err = LocaleRegistry::try_new(config(&case[0])).expect_err(&case.to_string());
		let (kind, code) = match &err {
			RegistryError::NoLocales => ("no-locales", None),
			RegistryError::DefaultNotListed(c) => ("default-not-listed", Some(c)),
			RegistryError::ListedTwice(c) => ("listed-twice", Some(c)),
			RegistryError::UnknownHreflangLocale(c) => ("unknown-hreflang-locale", Some(c)),
			RegistryError::ReservedHreflang(c) => ("reserved-hreflang", Some(c)),
			RegistryError::HreflangCollision(t) => ("hreflang-collision", Some(t)),
		};
		assert_eq!((kind, code.map(String::as_str)), (str_of(&case[1]), case[2].as_str()), "{case}");
	}
}
