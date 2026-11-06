//! The hash-bucketing vectors `@evinvest/experiments` runs too.
#![cfg(feature = "experiments")]

use ev_lib::experiments::{Experiment, fnv1a32, hash_to_unit, pick_variant_for};
use serde_json::Value;

fn fixture() -> Value {
	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/experiments/hash.json");
	serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))).expect("hash.json is JSON")
}

#[test]
fn fnv1a32_and_hash_to_unit() {
	for case in fixture()["fnv1a32"].as_array().expect("fnv1a32 cases") {
		let seed = case[0].as_str().expect("seed");
		let hash = u32::try_from(case[1].as_u64().expect("hash")).expect("a u32 hash");
		assert_eq!(fnv1a32(seed.as_bytes()), hash, "{seed:?}");
		assert_eq!(hash_to_unit(seed), f64::from(hash) / 2f64.powi(32), "{seed:?}");
	}
}

#[test]
fn pick_variant_for_vectors() {
	for case in fixture()["pick_variant_for"].as_array().expect("pick_variant_for cases") {
		let spec = &case[0];
		let exp = Experiment::new(spec["variants"].as_array().expect("variants").iter().map(|v| v.as_str().expect("a variant")));
		let exp = match spec.get("holdout") {
			Some(h) => exp.with_holdout(h.as_f64().expect("holdout is a number")),
			None => exp,
		};
		let [key, subject, variant] = [1, 2, 3].map(|i| case[i].as_str().expect("a string"));
		assert_eq!(pick_variant_for(&exp, key, subject), variant, "{case}");
	}
}
