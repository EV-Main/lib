//! Producer SDK for landing-host microfrontends.
//!
//! A microfrontend producer ships a self-registering ESM bundle: it defines a
//! custom element (`customElements.define("mfe-…", …)`) whose `connectedCallback`
//! mounts a Dioxus app into the host node (light DOM). The landing host curates a
//! registry of [`MfeManifest`]s and decides where to render each `<tag>`.
//!
//! This module is the entire Rust side of that contract. A producer crate writes
//! ~5 lines via `mfe!` and gets: the custom-element registration, the origin
//! self-derivation (so the bundle works cross-origin with no baked env), the
//! server-fn base override, the `{name,tag,kind}` manifest, and the host's
//! locale resolved into a [`Translator`](crate::i18n::Translator) above its root
//! — none of which it can get wrong, because the macro owns the naming
//! convention, the JSON shape, and where a locale comes from.
//!
//! The producer is wasm-only: the one cross-origin bundle is always a
//! `wasm32-unknown-unknown` cdylib. manganis/`dx` are unusable here — their asset
//! URLs are root-relative to the *serving* origin, i.e. inherently same-origin —
//! so producers hand-roll `wasm-bindgen --target web` and this SDK stays
//! manganis-free, deriving every URL from the bundle's own origin instead.
//!
//! The contract types compile on every target, and under `ts_gen` without `mfe`:
//! `mfe` cannot build natively (`dioxus/web`), and `ev_lib_gen` writes
//! `@evinvest/mfe`'s copy from them.

use serde::{Serialize, Serializer};

#[cfg(all(feature = "mfe", target_arch = "wasm32"))]
mod producer;
#[cfg(all(feature = "mfe", target_arch = "wasm32"))]
pub use producer::*;

/// Inline widget inside a host page, or the owner of a whole route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "ts_gen", derive(strum::EnumIter))]
pub enum MfeKind {
	Component,
	Page,
}

impl MfeKind {
	/// The word in `mfe.json`.
	pub const fn as_str(self) -> &'static str {
		match self {
			Self::Component => "component",
			Self::Page => "page",
		}
	}

	/// The kind a `mfe!` invocation names; const, so an unknown word fails the producer's build.
	///
	/// ```compile_fail
	/// const KIND: ev_lib::mfe::MfeKind = ev_lib::mfe::MfeKind::parse("widget");
	/// ```
	pub const fn parse(word: &str) -> Self {
		match word.as_bytes() {
			b"component" => Self::Component,
			b"page" => Self::Page,
			_ => panic!("mfe! `kind` is `component` or `page`"),
		}
	}
}

impl Serialize for MfeKind {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_str(self.as_str())
	}
}

/// What a producer's build emits (`mfe.json`), before the host gives it a URL.
///
/// ```
/// use ev_lib::mfe::{MfeKind, MfeManifest};
/// let manifest = MfeManifest { name: "review-archive.dashboard", tag: "mfe-review-archive-dashboard", kind: MfeKind::parse("page") };
/// assert_eq!(serde_json::to_string(&manifest).unwrap(), r#"{"name":"review-archive.dashboard","tag":"mfe-review-archive-dashboard","kind":"page"}"#);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "ts_gen", derive(ts_rs::TS))]
pub struct MfeManifest {
	/// Logical name, `{service}.{name}` for bundles built with `mfe!`.
	pub name: &'static str,
	/// Globally unique custom-element tag the bundle defines.
	pub tag: &'static str,
	#[cfg_attr(feature = "ts_gen", ts(type = "MfeKind"))]
	pub kind: MfeKind,
}
