//! Measuring the product: analytics and experiments (docs/ARCHITECTURE.md, "Scopes").

#[cfg(feature = "analytics")]
pub mod analytics;

#[cfg(feature = "experiments")]
pub mod experiments;
