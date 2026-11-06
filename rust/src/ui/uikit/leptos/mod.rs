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

mod atomic;
mod derived;
mod shell;
use atomic::*;
use derived::*;
use shell::*;

pub mod primitives;

// Styling data lives in the framework-free `ev_lib_classes` crate, the single
// source of truth the TS codegen reads too. Only the types a caller has to *name*
// to build a prop are re-exported.
pub(crate) use ev_lib_classes::*;
pub use ev_lib_classes::{
	Accent, AlertVariant, AsidePanelSide, BadgeVariant, ButtonGroupOrientation, ButtonVariant, ChatBubbleVariant, ChatStatusState, ChatToolState, ConfirmDialogTone, DrawerDirection,
	EmptyMediaVariant, FieldOrientation, HeaderBalanceTone, IconButtonVariant, InputGroupAddonAlign, InputGroupButtonSize, ItemMediaVariant, ItemSize, ItemVariant, Orientation, Polarity,
	ScrollBarOrientation, SectionLabelTone, SettingsSectionTone, Size, StatusTone, Surface, SystemBannerTone, TableAlign, TableDensity, TableVariant, ToastPosition, ToastVariant,
	ToggleVariant,
};

pub use crate::ui::uikit::core::FormControlContext;

mod fonts;

// The u2 app chrome: header, settings, account, chat and command parts.

// Tier B — interactive, over the core's roving, selection, slider, OTP and
// carousel state.
pub use accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
pub use account_list::{AccountList, AccountListItem};
pub use account_settings_dialog::{AccountSettingsDialog, AccountSettingsDialogLabels, AccountSettingsSection};
pub use alert::{Alert, AlertDescription, AlertTitle};
// Tier C — overlay/portal components. Glob-exported (each module's public
// surface is its components + variant enums). Rust overlays are dep-light:
// inline fixed positioning + a backdrop, no portal/floating/focus-trap — see
// the package README "Limitations".
pub use alert_dialog::*;
pub use aside::{AsideCollapse, AsideHeader, AsideHeaderLabels, AsidePanel, AsidePanelMode, AsideRail, AsideRailItem};
pub use auth_wall::AuthWall;
pub use avatar::{Avatar, AvatarFallback, AvatarImage};
pub use badge::Badge;
pub use band::{Check, Display, Eyebrow, Prose, Section, SectionHead, Stat};
pub use breadcrumb::{Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator};
pub use button::{Button, button_classes};
pub use button_group::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText};
// Tier D — heavy engines, hand-rolled dep-light (calendar with manual date
// math, date-time-picker over it without a native `datetime-local`, sonner
// toaster, form without rhf).
// See the package README "Limitations" for the fidelity gaps vs the originals.
pub use calendar::*;
pub use card::{Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use carousel::{Carousel, CarouselContent, CarouselEdgeFade, CarouselItem, CarouselNext, CarouselPrevious};
pub use chat_composer::{ChatComposer, ChatComposerLabels, ModelChip, ModelOption};
pub use chat_message::{ChatAttachment, ChatAttachmentState, ChatAttachmentType, ChatBubble, ChatMarker, ChatMarkerKind, ChatMessage, ChatMessageRole};
pub use chat_typing::ChatTyping;
pub use checkbox::Checkbox;
pub use collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
pub use command::*;
pub use command_palette::{CommandPalette, CommandPaletteGroup, CommandPaletteItem, CommandPaletteLabels, use_command_palette_shortcut};
pub use confirm_dialog::{ConfirmDialog, ConfirmDialogLabels};
pub use container::Container;
pub use context_menu::*;
pub use date_time_picker::*;
pub use dialog::*;
pub use drawer::*;
pub use dropdown_menu::*;
pub use empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};
pub use field::{Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend, FieldLegendVariant, FieldSeparator, FieldSet, FieldTitle};
pub use fonts::{FONT_FILES, Fonts};
// Site chrome — the footer grid and the 404/403/500 status pages. Both take
// their copy and their mark from the caller; the kit carries no brand.
pub use footer::{Footer, FooterLink, FooterLinkGroup, FooterOffice};
pub use form::*;
pub use form_row::FormRow;
pub use header_controls::{HeaderBalance, HeaderNotification, HeaderSearch, NotificationBell, NotificationBellLabels};
pub use hover_card::*;
pub use icon_button::IconButton;
pub use info_tip::*;
pub use input::Input;
pub use input_group::{InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput, InputGroupText, InputGroupTextarea};
pub use input_otp::{InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot};
pub use item::{Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia, ItemSeparator, ItemTitle};
pub use kbd::{Kbd, KbdGroup};
pub use key_value::{KeyValue, KeyValueLabel, KeyValueLabels};
pub use label::Label;
pub use logo::Logo;
pub use menubar::*;
pub use navigation_menu::*;
pub use pagination::{Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious};
pub use popover::*;
pub use popover_form::{PopoverForm, PopoverFormLabels};
pub use progress::Progress;
pub use radio_group::{RadioGroup, RadioGroupItem};
pub use scroll_area::{ScrollArea, ScrollBar};
pub use section_descriptor::SectionDescriptor;
pub use section_label::{SectionLabel, SectionLabelAs};
pub use select::*;
pub use separator::Separator;
pub use settings::{SettingsLayout, SettingsRow, SettingsSection};
pub use sheet::*;
pub use sidebar::{
	Sidebar, SidebarContent, SidebarContext, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton,
	SidebarMenuItem, SidebarProvider, SidebarRail, SidebarSeparator, SidebarTrigger, use_sidebar,
};
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use smart_header::{SmartHeader, SmartHeaderState};
pub use sonner::*;
pub use spinner::Spinner;
pub use status_item::{StatusDot, StatusItem};
pub use status_screen::{Forbidden, NotFound, ServerError, StatusLinkData, StatusScreen, status_cta_class};
pub use switch::Switch;
pub use system_banner::{SystemBanner, SystemBannerLabels};
pub use table::{Table, TableBody, TableCaption, TableCard, TableCell, TableFooter, TableHead, TableHeader, TableRow};
pub use tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
pub use textarea::Textarea;
pub use toggle::Toggle;
pub use toggle_group::{ToggleGroup, ToggleGroupItem};
pub use tooltip::*;
pub use top_bar::TopBar;
pub use workspace_tabs::{WorkspaceTab, WorkspaceTabs};

pub use crate::ui::uikit::core::{AccordionType, CarouselOrientation, SidebarCollapsible, SidebarSide, SidebarVariant, SliderOrientation, TabsOrientation, ToggleGroupType, toggle_classes};
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub use crate::ui::uikit::host::write_clipboard;
