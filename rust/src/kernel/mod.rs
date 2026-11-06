//! I/O-free domain vocabulary every other scope may build on (docs/ARCHITECTURE.md, "Scopes").

#[cfg(feature = "architecture")]
pub mod architecture;

#[cfg(feature = "i18n")]
pub mod i18n;

#[cfg(feature = "types")]
pub mod types;
