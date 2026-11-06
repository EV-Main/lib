//! The kit: one framework-free core, a port per renderer (docs/ARCHITECTURE.md, "The `uikit` library").

pub(crate) mod core;
pub(crate) mod host;

#[cfg(feature = "uikit_dioxus")]
pub mod dioxus;

#[cfg(feature = "uikit_leptos")]
pub mod leptos;
