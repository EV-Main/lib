//! The event model and PostHog payload builder — the framework-agnostic,
//! I/O-free, `wasm32`-safe core shared by the native client and the Dioxus
//! frontend. Mirrors the TS `@evinvest/analytics` capture semantics.

use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{Value, json};

/// PostHog's default US ingestion host, used when no host is configured (mirrors
/// the TS default).
pub const DEFAULT_HOST: &str = "https://us.i.posthog.com";
/// The `$lib` property [`capture_body`] stamps on every event.
pub const LIB_MARKER: &str = "ev-analytics";

/// A single event property value. Restricted to primitives by construction: event
/// properties are a dashboard contract and must never carry PII or nested objects
/// (mirrors the TS "primitives only, never PII" rule).
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum PropValue {
	/// A boolean flag.
	Bool(bool),
	/// An integer count.
	Int(i64),
	/// A floating-point measure.
	Num(f64),
	/// A short, non-PII string (an enum-like label, not free text).
	Str(String),
}

impl From<bool> for PropValue {
	fn from(v: bool) -> Self {
		PropValue::Bool(v)
	}
}
impl From<i64> for PropValue {
	fn from(v: i64) -> Self {
		PropValue::Int(v)
	}
}
impl From<i32> for PropValue {
	fn from(v: i32) -> Self {
		PropValue::Int(v as i64)
	}
}
impl From<f64> for PropValue {
	fn from(v: f64) -> Self {
		PropValue::Num(v)
	}
}
impl From<&str> for PropValue {
	fn from(v: &str) -> Self {
		PropValue::Str(v.to_string())
	}
}
impl From<String> for PropValue {
	fn from(v: String) -> Self {
		PropValue::Str(v)
	}
}

/// A captured event: a snake_case `<surface>_<thing>_<action>` name plus primitive
/// properties. Build it fluently:
///
/// ```
/// use ev_lib::analytics::Event;
/// let e = Event::new("calculator_submitted").prop("amount", 1000).prop("currency", "usd");
/// assert_eq!(e.name, "calculator_submitted");
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Event {
	/// The stable, snake_case event name.
	pub name: String,
	/// Primitive, non-PII properties keyed by name (ordered for stable output).
	pub props: BTreeMap<String, PropValue>,
}

impl Event {
	/// Starts an event with the given name and no properties.
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			props: BTreeMap::new(),
		}
	}

	/// Adds (or overwrites) a property, returning `self` for chaining.
	pub fn prop(mut self, key: impl Into<String>, value: impl Into<PropValue>) -> Self {
		self.props.insert(key.into(), value.into());
		self
	}
}

/// Builds the JSON body for PostHog's `POST /capture/` endpoint: the project
/// `api_key`, the `event` name, the `distinct_id`, and the `properties` (with a
/// `$lib` marker appended). Pure and network-free, so it is unit-tested without a
/// server; the timestamp is left to PostHog's receive time.
///
/// ```
/// use ev_lib::analytics::{Event, capture_body};
/// let body = capture_body("phc_key", "anon-1", &Event::new("hero_cta_clicked"));
/// assert_eq!(body["event"], "hero_cta_clicked");
/// assert_eq!(body["distinct_id"], "anon-1");
/// assert_eq!(body["properties"]["$lib"], "ev-analytics");
/// ```
pub fn capture_body(api_key: &str, distinct_id: &str, event: &Event) -> Value {
	let mut properties = serde_json::Map::new();
	for (key, value) in &event.props {
		properties.insert(key.clone(), serde_json::to_value(value).unwrap_or(Value::Null));
	}
	properties.insert("$lib".to_string(), Value::String(LIB_MARKER.to_string()));
	json!({
		"api_key": api_key,
		"event": event.name,
		"distinct_id": distinct_id,
		"properties": Value::Object(properties),
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn prop_value_from_impls_pick_the_right_variant() {
		assert_eq!(PropValue::from(true), PropValue::Bool(true));
		assert_eq!(PropValue::from(9_i64), PropValue::Int(9));
		assert_eq!(PropValue::from(9_i32), PropValue::Int(9));
		assert_eq!(PropValue::from(-3_i32), PropValue::Int(-3));
		assert_eq!(PropValue::from(2.5_f64), PropValue::Num(2.5));
		assert_eq!(PropValue::from("s"), PropValue::Str("s".to_string()));
		assert_eq!(PropValue::from(String::from("s")), PropValue::Str("s".to_string()));
	}

	#[test]
	fn event_new_and_prop_chaining() {
		let event = Event::new("calculator_submitted").prop("amount", 1000).prop("currency", "usd");
		assert_eq!(event.name, "calculator_submitted");
		assert_eq!(event.props.len(), 2);
		assert_eq!(event.props.get("amount"), Some(&PropValue::Int(1000)));
		assert_eq!(event.props.get("currency"), Some(&PropValue::Str("usd".to_string())));
	}

	#[test]
	fn event_default_is_empty() {
		let event = Event::default();
		assert_eq!(event.name, "");
		assert!(event.props.is_empty());
	}

	#[test]
	fn prop_overwrites_by_key() {
		let event = Event::new("e").prop("k", 1).prop("k", 2);
		assert_eq!(event.props.get("k"), Some(&PropValue::Int(2)));
		assert_eq!(event.props.len(), 1);
	}

	#[test]
	fn prop_overwrite_can_change_value_type() {
		let event = Event::new("e").prop("k", 1_i64).prop("k", "now-a-string");
		assert_eq!(event.props.get("k"), Some(&PropValue::Str("now-a-string".to_string())));
	}
}
