//! What renders, and what runs in the page (docs/ARCHITECTURE.md, "Scopes").

pub mod active;

#[cfg(any(feature = "uikit_dioxus", feature = "uikit_leptos"))]
pub mod uikit;
