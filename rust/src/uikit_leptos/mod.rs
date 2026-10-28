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

mod alert;
mod auth_wall;
mod avatar;
mod badge;
mod band;
mod breadcrumb;
mod button;
mod button_group;
mod card;
mod command;
mod container;
mod empty;
mod fonts;
mod input;
mod item;
mod kbd;
mod label;
mod logo;
mod pagination;
mod progress;
mod scroll_area;
mod section_descriptor;
mod separator;
mod skeleton;
mod spinner;
mod table;
mod terminal;
mod textarea;

pub use alert::{Alert, AlertDescription, AlertTitle};
pub use auth_wall::AuthWall;
pub use avatar::{Avatar, AvatarFallback, AvatarImage};
pub use badge::Badge;
pub use band::{Check, Display, Eyebrow, Prose, Section, SectionHead, Stat};
pub use breadcrumb::{Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator};
pub use button::{Button, button_classes};
pub use button_group::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText};
pub use card::{Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use command::*;
pub use container::Container;
pub use empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyTitle};
pub use fonts::{FONT_FILES, Fonts};
pub use input::Input;
pub use item::{Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia, ItemSeparator, ItemTitle};
pub use kbd::{Kbd, KbdGroup};
pub use label::Label;
pub use logo::Logo;
pub use pagination::{Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink, PaginationNext, PaginationPrevious};
pub use progress::Progress;
pub use scroll_area::{ScrollArea, ScrollBar};
pub use section_descriptor::SectionDescriptor;
pub use separator::Separator;
pub use skeleton::Skeleton;
pub use spinner::Spinner;
pub use table::{Table, TableBody, TableCaption, TableCard, TableCell, TableFooter, TableHead, TableHeader, TableRow};
pub use terminal::{
	OpenOrdersEmpty, OrderBook, OrderBookHead, OrderBookRow, OrderBookSpread, OrderForm, OrderFormRow, OrderFormSubmit, Terminal, TerminalChart, TerminalPane, TerminalPaneBody,
	TerminalPaneHeader, TerminalTicker, TickerStat, TradesTapeRow,
};
pub use textarea::Textarea;

// Tier B — interactive, over the core's roving, selection, slider, OTP and
// carousel state.
mod accordion;
mod carousel;
mod checkbox;
mod collapsible;
mod input_otp;
mod radio_group;
mod sidebar;
mod slider;
mod switch;
mod tabs;
mod toggle;
mod toggle_group;

pub use accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
pub use carousel::{Carousel, CarouselContent, CarouselEdgeFade, CarouselItem, CarouselNext, CarouselPrevious};
pub use checkbox::Checkbox;
pub use collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger};
pub use input_otp::{InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot};
pub use radio_group::{RadioGroup, RadioGroupItem};
pub use sidebar::{
	Sidebar, SidebarContent, SidebarContext, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton,
	SidebarMenuItem, SidebarProvider, SidebarRail, SidebarSeparator, SidebarTrigger, use_sidebar,
};
pub use slider::Slider;
pub use switch::Switch;
pub use tabs::{Tabs, TabsContent, TabsList, TabsTrigger};
pub use toggle::Toggle;
pub use toggle_group::{ToggleGroup, ToggleGroupItem};

pub use crate::uikit::{AccordionType, CarouselOrientation, SidebarCollapsible, SidebarSide, SidebarVariant, SliderOrientation, TabsOrientation, ToggleGroupType, toggle_classes};

mod field;
mod input_group;

pub use field::{Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend, FieldLegendVariant, FieldSeparator, FieldSet, FieldTitle};
pub use input_group::{InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput, InputGroupText, InputGroupTextarea};

// Tier C — overlay/portal components. Glob-exported (each module's public
// surface is its components + variant enums). Rust overlays are dep-light:
// inline fixed positioning + a backdrop, no portal/floating/focus-trap — see
// the package README "Limitations".
mod alert_dialog;
mod context_menu;
mod dialog;
mod drawer;
mod dropdown_menu;
mod hover_card;
mod info_tip;
mod menubar;
mod navigation_menu;
mod popover;
mod select;
mod sheet;
mod tooltip;

pub use alert_dialog::*;
pub use context_menu::*;
pub use dialog::*;
pub use drawer::*;
pub use dropdown_menu::*;
pub use hover_card::*;
pub use info_tip::*;
pub use menubar::*;
pub use navigation_menu::*;
pub use popover::*;
pub use select::*;
pub use sheet::*;
pub use tooltip::*;

// Tier D — heavy engines, hand-rolled dep-light (calendar with manual date
// math, date-time-picker over it without a native `datetime-local`, sonner
// toaster, form without rhf).
// See the package README "Limitations" for the fidelity gaps vs the originals.
mod calendar;
mod date_time_picker;
mod form;
mod sonner;

pub use calendar::*;
pub use date_time_picker::*;
pub use form::*;
pub use sonner::*;

// Site chrome — the footer grid and the 404/403/500 status pages. Both take
// their copy and their mark from the caller; the kit carries no brand.
mod footer;
mod status_screen;

pub use footer::{Footer, FooterLink, FooterLinkGroup, FooterOffice};
pub use status_screen::{Forbidden, NotFound, ServerError, StatusLinkData, StatusScreen, status_cta_class};
