//! The parsing-contract vectors `@evinvest/settings` runs too.
#![cfg(feature = "settings")]

use ev_lib::settings::FromEnvValue;
use serde_json::Value;

fn fixture() -> Value {
	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/settings/contract.json");
	serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))).expect("contract.json is JSON")
}

fn strs(list: &Value) -> impl Iterator<Item = &str> {
	list.as_array().expect("a list").iter().map(|s| s.as_str().expect("a string"))
}

fn pairs<T: serde::de::DeserializeOwned>(list: &Value) -> Vec<(String, T)> {
	serde_json::from_value(list.clone()).expect("[raw, parsed] pairs")
}

#[test]
fn bools() {
	let vectors = &fixture()["bool"];
	for raw in strs(&vectors["true"]) {
		assert_eq!(bool::from_env_value(raw), Ok(true), "{raw:?}");
	}
	for raw in strs(&vectors["false"]) {
		assert_eq!(bool::from_env_value(raw), Ok(false), "{raw:?}");
	}
	let error = vectors["error"].as_str().expect("bool error text");
	for raw in strs(&vectors["invalid"]) {
		assert_eq!(bool::from_env_value(raw), Err(error.to_string()), "{raw:?}");
	}
}

#[test]
fn scalars() {
	let fixture = fixture();
	for raw in strs(&fixture["string"]) {
		assert_eq!(String::from_env_value(raw).as_deref(), Ok(raw));
	}
	for (raw, parsed) in pairs::<i64>(&fixture["int"]["accept"]) {
		assert_eq!(i64::from_env_value(&raw), Ok(parsed), "{raw:?}");
	}
	for raw in strs(&fixture["int"]["reject"]) {
		assert!(i64::from_env_value(raw).is_err(), "{raw:?} must be invalid for an integer");
	}
	for (raw, parsed) in pairs::<f64>(&fixture["float"]["accept"]) {
		assert_eq!(f64::from_env_value(&raw), Ok(parsed), "{raw:?}");
	}
	for raw in strs(&fixture["float"]["reject"]) {
		assert!(f64::from_env_value(raw).is_err(), "{raw:?} must be invalid for a float");
	}
}

#[test]
fn lists() {
	let fixture = fixture();
	for (raw, parsed) in pairs::<Vec<String>>(&fixture["list"]) {
		assert_eq!(Vec::<String>::from_env_value(&raw), Ok(parsed), "{raw:?}");
	}
	for (raw, parsed) in pairs::<Vec<i64>>(&fixture["int_list"]) {
		assert_eq!(Vec::<i64>::from_env_value(&raw), Ok(parsed), "{raw:?}");
	}
	let case = &fixture["int_list_error"];
	let error = Vec::<i64>::from_env_value(case["raw"].as_str().expect("raw")).expect_err("an unparsable item fails the list");
	assert!(error.starts_with(case["prefix"].as_str().expect("prefix")), "{error:?}");
	assert!(!error.contains(case["item"].as_str().expect("item")), "the item value leaked: {error:?}");
}
