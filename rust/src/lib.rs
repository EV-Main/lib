#![allow(unused_features)]
#![feature(default_field_values)]
//! EV-invest's shared Rust libraries, one per Cargo feature.
//!
//! Each library is a self-contained module gated behind a feature flag, so a
//! consumer pulls in only what it asks for and nothing else is compiled:
//!
//! ```toml
//! ev_lib = { version = "0.31", default-features = false, features = ["architecture"] }
//! ```
//!
//! # Available features
//!
//! - **`architecture`** — generic, I/O-free, `wasm32`-safe DDD tactical kernel
//!   (typed ids, entities, aggregate roots, repositories, gateways, the unit of
//!   work, domain events, specifications). See [`architecture`].
//! - **`uikit_dioxus`** / **`uikit_leptos`** — the dep-light UI kit (mirrors
//!   `@evinvest/uikit`), one port per renderer over one framework-free core; both
//!   render the same HTML. See [`uikit_dioxus`] and [`uikit_leptos`].
//! - **`analytics`** — PostHog product analytics (mirrors `@evinvest/analytics`):
//!   native async capture over `reqwest`; pure-Rust fetch on the Dioxus/wasm
//!   frontend. Does network I/O — opt-in, not the kernel. See [`analytics`].
//! - **`error_monitoring`** — Sentry error monitoring (mirrors
//!   `@evinvest/error-monitoring`): native backend on the `sentry` crate; pure-Rust
//!   envelope POST + panic hook on the Dioxus/wasm frontend. See [`error_monitoring`].
//! - **`experiments`** — frontend-only A/B testing (mirrors `@evinvest/experiments`):
//!   cookie-bucketed variant assignment; exposure is reported through an injected
//!   sink, so it never imports `analytics`. See [`experiments`].
//! - **`settings`** — typed env settings (mirrors `@evinvest/settings`): the
//!   `settings!` macro reads env vars into a validated struct with aggregate
//!   error reporting and secret redaction. Zero deps; sops stays at the shell/CI
//!   boundary. See [`settings`](mod@settings).
//! - **`types`** — shared domain TypeObjects (mirrors `@evinvest/types`):
//!   [`PhoneNumber`](types::PhoneNumber) with E.164 validation and formatting, plus
//!   the generic [`Branded<T, B>`](types::Branded) newtype for future types (Email,
//!   Currency, …). Zero deps, wasm-safe. See [`types`](mod@types).
//! - **`otel`** — OpenTelemetry logs + traces over OTLP (native backends only;
//!   inert on wasm): two `tracing` layers + a flush guard, and tonic
//!   interceptors for W3C trace propagation. See [`otel`].
//! - **`alerts`** — WARN/ERROR log lines with the artifacts they point at
//!   (`[<path>]` under an [`Artifacts`](alerts::Artifacts) root, pruned after 7 days),
//!   posted to Discord webhooks. Native-only. See [`alerts`].
//! - **`i18n`** — five-locale internationalisation (mirrors `@evinvest/i18n`):
//!   the locale registry, the `/<locale>` URL contract, `Accept-Language`
//!   negotiation, an ICU-subset message formatter, the translation policy that
//!   refuses a translation whose English source has moved, and the [`t!`] macro
//!   — English written where it renders, with the catalogue generated back out
//!   of the code. Wasm-safe and zero-dep there; natively it carries `inventory`,
//!   which is what turns `t!` sites into that catalogue. Reads the same
//!   `messages/<locale>/*.json` as the TypeScript half, so a catalogue is
//!   portable between them. See [`i18n`].
//! - **`ts_gen`** — TypeScript emitted from Rust types (through `ts-rs`) and
//!   values (through serde), for a repo's own `gen` step. Build-time only. See
//!   [`ts_gen`].
//! - **`marketing`** — outbound contact links (the Rust twin of
//!   `@evinvest/marketing`'s contact core): `tel:` / `wa.me` hrefs and which
//!   channel a link opens. No I/O, wasm-safe. See [`marketing`].

#![cfg_attr(docsrs, feature(doc_cfg))]

mod codegen;
mod kernel;
mod ops;
mod product;
mod ui;

// Public names stay flat; the scopes are internal (docs/ARCHITECTURE.md, "Scopes").
#[cfg(feature = "ts_gen")]
pub use codegen::ts_gen;
// The `cn!` fuse macro and styling data live in the framework-free `ev_lib_classes`
// crate; re-export so the in-crate `crate::cn` path keeps resolving.
#[cfg(any(feature = "uikit_dioxus", feature = "uikit_leptos"))]
pub use ev_lib_classes::cn;
#[cfg(feature = "architecture")]
pub use kernel::architecture;
#[cfg(feature = "i18n")]
pub use kernel::i18n;
#[cfg(feature = "types")]
pub use kernel::types;
#[cfg(all(feature = "alerts", not(target_arch = "wasm32")))]
pub use ops::alerts;
#[cfg(feature = "error_monitoring")]
pub use ops::error_monitoring;
#[cfg(feature = "otel")]
pub use ops::otel;
#[cfg(feature = "settings")]
pub use ops::settings;
#[cfg(feature = "analytics")]
pub use product::analytics;
#[cfg(feature = "experiments")]
pub use product::experiments;
#[cfg(feature = "marketing")]
pub use ui::active::contact as marketing;
#[cfg(all(feature = "mfe", target_arch = "wasm32"))]
pub use ui::active::mfe;
#[cfg(feature = "uikit_dioxus")]
pub use ui::uikit::dioxus as uikit_dioxus;
#[cfg(feature = "uikit_leptos")]
pub use ui::uikit::leptos as uikit_leptos;
