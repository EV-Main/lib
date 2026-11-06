//! Running a service: settings, error monitoring, telemetry, alerts (docs/ARCHITECTURE.md, "Scopes").

#[cfg(all(feature = "alerts", not(target_arch = "wasm32")))]
pub mod alerts;

#[cfg(feature = "error_monitoring")]
pub mod error_monitoring;

#[cfg(feature = "otel")]
pub mod otel;

#[cfg(feature = "settings")]
pub mod settings;
