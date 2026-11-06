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

pub use account_settings_dialog::AccountSettingsDialogLabels;
pub use aside::{AsideCollapse, AsideHeaderLabels, AsidePanelMode};
pub use button::button_classes;
pub use calendar::CalendarDate;
pub(crate) use calendar::{Day, Grid, NEXT_MONTH, NO_CLOCK_MONTH, PREVIOUS_MONTH, WEEKDAYS};
pub(crate) use carousel::Carousel;
pub use carousel::CarouselOrientation;
pub(crate) use chat::enter_sends;
pub use chat::{ChatAttachmentState, ChatAttachmentType, ChatComposerLabels, ChatMessageRole, ModelOption};
pub(crate) use command::{Command, CommandKey, Row};
pub use command_palette::CommandPaletteLabels;
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) use command_palette::{ShortcutHit, ShortcutKey, shortcut_hit};
pub use confirm_dialog::ConfirmDialogLabels;
pub(crate) use confirm_dialog::ConfirmFlow;
pub(crate) use date_time_picker::Picker;
pub use date_time_picker::{DateTimePickerLabels, LocalDateTime};
pub(crate) use drawer::Presence;
pub use field::FieldLegendVariant;
pub use field_context::FormControlContext;
pub use footer::{FooterLink, FooterLinkGroup, FooterOffice};
pub(crate) use form::FormItemContext;
pub(crate) use header_controls::CTRL_K;
pub use header_controls::HeaderNotification;
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub(crate) use header_controls::platform_shortcut;
pub(crate) use input_otp::InputOtp;
pub(crate) use key::KeyInput;
pub use key_value::KeyValueLabels;
pub use menu::{ContextMenuItemVariant, DropdownMenuItemVariant, MenubarItemVariant, navigation_menu_trigger_style};
#[cfg(test)]
pub(crate) use order::Unplaced;
pub(crate) use order::{Placement, sort_by_position};
pub(crate) use popover::popover_placement;
pub use popover_form::PopoverFormLabels;
pub use primitives::{DismissReason, RovingOrientation, Side};
pub(crate) use primitives::{LayerStack, Roving};
pub use section_label::SectionLabelAs;
pub(crate) use select::{Select, SelectKey, SelectOption};
pub(crate) use selection::{ACCORDION_CHEVRON, TOGGLE_GROUP_ITEM, TOGGLE_GROUP_ROOT, toggled};
pub use selection::{AccordionType, ToggleGroupType};
pub(crate) use sheet::sheet_side_class;
pub(crate) use sidebar::{SIDEBAR_STYLE, sidebar_state};
pub use sidebar::{SidebarCollapsible, SidebarSide, SidebarVariant};
pub use slider::SliderOrientation;
pub(crate) use slider::{Drag, Slider, Track};
pub use smart_header::SmartHeaderState;
pub(crate) use sonner::{DEFAULT_DURATION_MS, ToastQueue, ToastState, ToastView, toaster_style, y_position};
pub use status_screen::{StatusLinkData, status_cta_class};
pub use system_banner::SystemBannerLabels;
pub use tabs::TabsOrientation;
pub use toggle::toggle_classes;
