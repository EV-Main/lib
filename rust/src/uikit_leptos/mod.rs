//! `uikit_leptos` — EV-invest's dep-light UI kit, the Leptos port.
//!
//! The same components, props and HTML as [`uikit_dioxus`](crate::uikit_dioxus),
//! over the kit's framework-free core. Never enables `ssr`, `hydrate` or `csr`:
//! the consumer's own `leptos` picks one.
//!
//! ```toml
//! ev_lib = { version = "0.29", default-features = false, features = ["uikit_leptos"] }
//! [target.'cfg(target_arch = "wasm32")'.dependencies]
//! ev_lib = { version = "0.29", default-features = false, features = ["uikit_leptos", "wasm"] }
//! ```

pub mod primitives;

// Styling data lives in the framework-free `ev_lib_classes` crate, the single
// source of truth the TS codegen reads too. Only the types a caller has to *name*
// to build a prop are re-exported.
pub(crate) use ev_lib_classes::*;
pub use ev_lib_classes::{
	Accent, AlertVariant, BadgeVariant, BookSide, ButtonGroupOrientation, ButtonVariant, DrawerDirection, EmptyMediaVariant, FieldOrientation, InputGroupAddonAlign, InputGroupButtonSize,
	ItemMediaVariant, ItemSize, ItemVariant, OrderSide, Orientation, Polarity, ScrollBarOrientation, Size, Surface, TableAlign, TableDensity, TableVariant, TerminalArea, ToastPosition,
	ToastVariant, ToggleVariant,
};

pub use crate::uikit::FormControlContext;

mod button;
mod command;
mod input;
mod kbd;

pub use button::{Button, button_classes};
pub use command::*;
pub use input::Input;
pub use kbd::{Kbd, KbdGroup};
