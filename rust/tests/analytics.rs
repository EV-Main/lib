//! The capture-body vectors `@evinvest/analytics` runs too.
#![cfg(feature = "analytics")]

use ev_lib::analytics::{Event, PropValue, capture_body};
use serde_json::Value;

fn prop(v: &Value) -> PropValue {
	match v {
		Value::Bool(b) => PropValue::Bool(*b),
		Value::Number(n) => n.as_i64().map_or_else(|| PropValue::Num(n.as_f64().expect("a JSON number")), PropValue::Int),
		Value::String(s) => PropValue::Str(s.clone()),
		Value::Object(o) => PropValue::Num(o["f64"].as_str().expect("{\"f64\": s}").parse().expect("a float literal")),
		other => panic!("{other} is not a prop value"),
	}
}

#[test]
fn capture_bodies() {
	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/fixtures/analytics/beacon.json");
	let fixture: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {path}: {e}"))).expect("beacon.json is JSON");
	for case in fixture["capture"].as_array().expect("beacon.json has a `capture` list") {
		let s = |i: usize| case[i].as_str().unwrap_or_else(|| panic!("{case}: field {i} is not a string"));
		let event = case[3].as_object().expect("props").iter().fold(Event::new(s(2)), |e, (k, v)| e.prop(k.as_str(), prop(v)));
		assert_eq!(serde_json::to_string(&capture_body(s(0), s(1), &event)).expect("serializable"), s(4), "{case}");
	}
}
