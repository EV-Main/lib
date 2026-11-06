//! The kit's framework-free core: no Dioxus, no Leptos, no DOM. One plain struct
//! per stateful component, driven by `&mut self` methods over plain values and
//! read back as view data; each port holds it in one reactive cell, turns its own
//! events into core inputs and renders the markup. See docs/ARCHITECTURE.md,
//! "The `uikit` library".

mod atomic;
mod derived;
mod shell;
use atomic::*;
use derived::*;
use shell::*;

mod key;
mod order;
mod primitives;
mod selection;

pub use aside::AsideCollapse;
pub use button::button_classes;
pub use calendar::CalendarDate;
pub(crate) use calendar::{Day, Grid, NO_CLOCK_MONTH};
pub(crate) use carousel::{Carousel, carousel_transform};
pub use chat::ModelOption;
pub(crate) use chat::enter_sends;
pub(crate) use command::{Command, CommandKey, Row};
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) use command_palette::{ShortcutHit, ShortcutKey, shortcut_hit};
pub(crate) use confirm_dialog::ConfirmFlow;
pub use date_time_picker::LocalDateTime;
pub(crate) use date_time_picker::Picker;
pub(crate) use drawer::Presence;
pub use field_context::FormControlContext;
pub use footer::{FooterLink, FooterLinkGroup, FooterOffice};
pub(crate) use form::FormItemContext;
pub use header_controls::HeaderNotification;
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) use header_controls::platform_shortcut;
pub(crate) use input_otp::InputOtp;
pub(crate) use key::KeyInput;
pub use menu::navigation_menu_trigger_style;
#[cfg(test)]
pub(crate) use order::Unplaced;
pub(crate) use order::{Placement, sort_by_position};
pub(crate) use popover::popover_placement;
pub use primitives::DismissReason;
pub(crate) use primitives::{LayerStack, Roving};
pub use section_label::SectionLabelAs;
pub(crate) use select::{Select, SelectKey, SelectOption};
pub(crate) use selection::toggled;
pub(crate) use sheet::sheet_side_class;
pub(crate) use sidebar::{sidebar_collapsible_data, sidebar_state, sidebar_style};
pub(crate) use slider::{Drag, Slider, Track};
pub(crate) use sonner::{ToastQueue, ToastState, ToastView, toaster_style, y_position};
pub use status_screen::{StatusLinkData, status_cta_class};
pub use toggle::toggle_classes;
