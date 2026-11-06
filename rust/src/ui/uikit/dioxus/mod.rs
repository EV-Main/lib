//! `uikit_dioxus` — EV-invest's dep-light UI kit, the Dioxus port.
//!
//! Renderer-agnostic RSX components over the kit's framework-free core, rendering
//! the same HTML as [`uikit_leptos`](crate::uikit_leptos) and mirrored semantically
//! by the `@evinvest/uikit` TypeScript package. Styling is Tailwind-utility based; every class references
//! a design token from `tokens.css` (repo root), the theme contract a
//! consumer must `@import` into its Tailwind v4 entrypoint.
//!
//! Variants are plain `enum`s matched to class strings (no `cva`); the `class`
//! prop is fused last via [`cn!`](crate::cn) so a caller override wins. Behaviour
//! shared by interactive components lives in [`primitives`]; see the package
//! README for the Rust↔TS map and the overlay limitations.
//!
//! ```toml
//! [target.'cfg(target_arch = "wasm32")'.dependencies]
//! ev_lib = { version = "0.29", default-features = false, features = ["uikit_dioxus", "modern", "wasm"] }
//! ```

mod atomic;
mod derived;
mod shell;
use atomic::*;
use derived::*;
use shell::*;

pub mod primitives;

#[cfg(test)]
mod test_util;

// Styling data lives in the Dioxus-free `ev_lib_classes` crate, the single source
// of truth the TS codegen reads too. The class strings are an implementation
// detail of the components — only the types a caller has to *name* to build a
// prop are re-exported.
pub(crate) use ev_lib_classes::*;
pub use ev_lib_classes::{
	Accent, AsidePanelSide, ButtonGroupOrientation, ButtonVariant, ChatBubbleVariant, ChatStatusState, ChatToolState, DrawerDirection, EmptyMediaVariant, FieldOrientation,
	IconButtonVariant, InputGroupAddonAlign, InputGroupButtonSize, ItemMediaVariant, ItemSize, ItemVariant, Orientation, Polarity, ScrollBarOrientation, SectionLabelTone, Size, StatusTone,
	Surface, TableAlign, TableDensity, TableVariant, ToastPosition, ToggleVariant, Tone,
};

mod fonts;

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
pub use command::{Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandSeparator, CommandShortcut, *};
pub use command_palette::{CommandPalette, CommandPaletteGroup, CommandPaletteItem, CommandPaletteLabels, use_command_palette_shortcut};
pub use confirm_dialog::{ConfirmDialog, ConfirmDialogLabels};
pub use container::Container;
pub use context_menu::*;
pub use date_time_picker::*;
pub use dialog::*;
pub use drawer::{Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerOverlay, DrawerTitle, DrawerTrigger, *};
pub use dropdown_menu::*;
pub use empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};
pub use ev_lib_classes::{AccordionType, SidebarCollapsible, SidebarSide, SidebarVariant, ToggleGroupType};
pub use field::{Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend, FieldLegendVariant, FieldSeparator, FieldSet, FieldTitle};
pub use fonts::Fonts;
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
pub use key_value::{KeyValue, KeyValueLabels};
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
pub use select::{Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectSeparator, SelectTrigger, SelectValue, *};
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

pub use crate::ui::uikit::core::{FormControlContext, toggle_classes};
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub use crate::ui::uikit::host::write_clipboard;
