//! The kit's framework-free core: no Dioxus, no Leptos, no DOM. One plain struct
//! per stateful component, driven by `&mut self` methods over plain values and
//! read back as view data; each port holds it in one reactive cell, turns its own
//! events into core inputs and renders the markup. See docs/ARCHITECTURE.md,
//! "The `uikit` library".

mod command;
mod form;
mod key;
mod order;
mod primitives;

pub(crate) use command::{Command, CommandKey, Row};
pub use form::FormControlContext;
pub(crate) use key::KeyInput;
#[cfg(test)]
pub(crate) use order::Unplaced;
pub(crate) use order::{Placement, sort_by_position};
pub use primitives::{DismissReason, RovingOrientation, Side};
pub(crate) use primitives::{LayerStack, Roving};
