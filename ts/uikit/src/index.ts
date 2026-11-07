/**
 * `@evinvest/uikit` — EV-invest's dep-light React UI kit.
 *
 * shadcn-semantics components with **no** `@radix-ui/*` and **no** `cva`: variant
 * maps are plain objects fused with {@link cn} (`clsx` + `tailwind-merge`), and
 * overlay behaviour (portals, floating, focus traps) is hand-rolled in
 * `./primitives`. The package mirrors the `ev_lib::uikit` Rust feature semantically.
 *
 * Styling depends on the design tokens in `@evinvest/uikit/styles/tokens.css` — a
 * consumer must `@import` that file into its Tailwind v4 entrypoint.
 */

export { cn } from "./lib/cn";

export { Slot } from "./primitives/slot";
export type { SlotProps } from "./primitives/slot";
export { useControllableState } from "./primitives/use-controllable-state";
export { Portal, PortalProvider } from "./primitives/portal";
export type { PortalProps, PortalProviderProps } from "./primitives/portal";
export { useDismissableLayer, DismissableLayerScope, LAYER_IGNORE_ATTR } from "./primitives/dismissable-layer";
export type { DismissEvent } from "./primitives/dismissable-layer";
export { useFloating } from "./primitives/use-floating";
export type { Side, Align, FloatingResult } from "./primitives/use-floating";
export { useFocusScope } from "./primitives/focus-scope";
export { usePresence } from "./primitives/presence";
export { useRovingFocus } from "./primitives/use-roving-focus";
export { useHoverIntent } from "./primitives/use-hover-intent";
export type { HoverIntentHandlers } from "./primitives/use-hover-intent";
export { mergeRefs } from "./primitives/merge-refs";

export { Alert, AlertTitle, AlertDescription } from "./components/atomic/alert";
export type { Tone } from "./generated/tone";
export type { AlertProps } from "./components/atomic/alert";

export { Avatar, AvatarImage, AvatarFallback } from "./components/atomic/avatar";

export { Badge } from "./components/atomic/badge";
export type { BadgeProps } from "./components/atomic/badge";

export {
  Breadcrumb,
  BreadcrumbList,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbPage,
  BreadcrumbSeparator,
  BreadcrumbEllipsis,
} from "./components/atomic/breadcrumb";
export type { BreadcrumbLinkProps } from "./components/atomic/breadcrumb";

export { Button, buttonVariants } from "./components/atomic/button";
export type {
  Accent,
  ButtonProps,
  ButtonVariant,
  ButtonSize,
  ButtonVariantsOptions,
} from "./components/atomic/button";

export {
  ButtonGroup,
  ButtonGroupText,
  ButtonGroupSeparator,
} from "./components/atomic/button-group";
export type {
  ButtonGroupOrientation,
  ButtonGroupProps,
  ButtonGroupTextProps,
} from "./components/atomic/button-group";

export {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardAction,
  CardContent,
  CardFooter,
} from "./components/atomic/card";

export {
  Empty,
  EmptyHeader,
  EmptyMedia,
  EmptyTitle,
  EmptyDescription,
  EmptyContent,
} from "./components/atomic/empty";
export type { EmptyMediaVariant, EmptyMediaProps } from "./components/atomic/empty";

export { AuthWall } from "./components/shell/auth-wall";
export type { AuthWallProps } from "./components/shell/auth-wall";

export {
  Field,
  FieldSet,
  FieldLegend,
  FieldGroup,
  FieldContent,
  FieldLabel,
  FieldTitle,
  FieldDescription,
  FieldSeparator,
  FieldError,
} from "./components/derived/field";
export type { FieldOrientation } from "./components/derived/field";

export { SectionDescriptor } from "./components/atomic/section-descriptor";
export type { SectionDescriptorProps } from "./components/atomic/section-descriptor";

export { Footer } from "./components/shell/footer";
export type {
  FooterProps,
  FooterLink,
  FooterLinkGroup,
  FooterOffice,
} from "./components/shell/footer";

export { Input } from "./components/atomic/input";
export type { InputProps, InputSize } from "./components/atomic/input";
export { Textarea } from "./components/atomic/textarea";
export type { TextareaProps, TextareaSize } from "./components/atomic/textarea";
export { Label } from "./components/atomic/label";

export { Logo } from "./components/shell/logo";
export type { LogoProps } from "./components/shell/logo";

export {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupText,
  InputGroupInput,
  InputGroupTextarea,
} from "./components/derived/input-group";
export type {
  InputGroupAddonAlign,
  InputGroupButtonSize,
  InputGroupAddonProps,
  InputGroupButtonProps,
} from "./components/derived/input-group";

export {
  Item,
  ItemMedia,
  ItemContent,
  ItemTitle,
  ItemDescription,
  ItemActions,
  ItemHeader,
  ItemFooter,
  ItemGroup,
  ItemSeparator,
} from "./components/derived/item";
export type {
  ItemVariant,
  ItemSize,
  ItemMediaVariant,
  ItemProps,
  ItemMediaProps,
} from "./components/derived/item";

export { Kbd, KbdGroup } from "./components/atomic/kbd";


export {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationPrevious,
  PaginationNext,
  PaginationEllipsis,
} from "./components/atomic/pagination";
export type { PaginationLinkProps } from "./components/atomic/pagination";

export { Progress } from "./components/atomic/progress";
export type { ProgressProps } from "./components/atomic/progress";

export { Separator } from "./components/atomic/separator";
export type { SeparatorOrientation, SeparatorProps } from "./components/atomic/separator";

export { Skeleton } from "./components/atomic/skeleton";
export { Spinner } from "./components/atomic/spinner";

export {
  StatusScreen,
  NotFound,
  Forbidden,
  ServerError,
  statusCtaClass,
} from "./components/shell/status-screen";
export type {
  StatusLinkData,
  StatusScreenProps,
  StatusPageProps,
  ServerErrorProps,
} from "./components/shell/status-screen";

export {
  Table,
  TableHeader,
  TableBody,
  TableFooter,
  TableRow,
  TableHead,
  TableCell,
  TableCaption,
  TableCard,
} from "./components/atomic/table";
export type {
  TableProps,
  TableHeadProps,
  TableCellProps,
  TableVariant,
  TableDensity,
  TableAlign,
} from "./components/atomic/table";

// TS-only: the phone form of a table.
export {
  ListRows,
  ListRow,
  ListRowLabel,
  ListRowValue,
} from "./components/shell/list-rows";
export type {
  ListRowsProps,
  ListRowsVariant,
  ListRowProps,
  ListRowLabelProps,
} from "./components/shell/list-rows";

// Tier B — self-contained interactive components.
export {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "./components/atomic/accordion";
export type {
  AccordionItemProps,
  AccordionProps,
} from "./components/atomic/accordion";
export {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "./components/atomic/collapsible";
export type {
  CollapsibleProps,
} from "./components/atomic/collapsible";
export {
  Container,
} from "./components/atomic/container";
export type {
  ContainerProps,
} from "./components/atomic/container";
export {
  Check,
  Display,
  Eyebrow,
  Prose,
  Section,
  SectionHead,
  Stat,
} from "./components/atomic/band";
export type {
  Polarity,
  SectionHeadProps,
  SectionProps,
  StatProps,
  Surface,
} from "./components/atomic/band";
export {
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from "./components/atomic/tabs";
export type {
  TabsContentProps,
  TabsProps,
  TabsTriggerProps,
} from "./components/atomic/tabs";
export {
  Toggle,
  toggleVariants,
} from "./components/atomic/toggle";
export type {
  ToggleProps,
  ToggleSize,
  ToggleVariant,
  ToggleVariantsOptions,
} from "./components/atomic/toggle";
export {
  ToggleGroup,
  ToggleGroupItem,
} from "./components/atomic/toggle-group";
export type {
  ToggleGroupItemProps,
  ToggleGroupProps,
} from "./components/atomic/toggle-group";
export {
  Switch,
} from "./components/atomic/switch";
export type {
  SwitchProps,
} from "./components/atomic/switch";
export {
  Checkbox,
} from "./components/atomic/checkbox";
export type {
  CheckboxProps,
} from "./components/atomic/checkbox";
export {
  RadioGroup,
  RadioGroupItem,
} from "./components/atomic/radio-group";
export type {
  RadioGroupItemProps,
  RadioGroupProps,
} from "./components/atomic/radio-group";
export {
  Slider,
} from "./components/atomic/slider";
export type {
  SliderProps,
} from "./components/atomic/slider";
export {
  ScrollArea,
  ScrollBar,
} from "./components/atomic/scroll-area";
export type {
  ScrollBarOrientation,
  ScrollBarProps,
} from "./components/atomic/scroll-area";
export {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarInset,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarProvider,
  SidebarRail,
  SidebarSeparator,
  SidebarTrigger,
  useSidebar,
} from "./components/atomic/sidebar";
export type {
  SidebarGroupLabelProps,
  SidebarMenuButtonProps,
  SidebarMenuButtonSize,
  SidebarProps,
  SidebarProviderProps,
} from "./components/atomic/sidebar";
export {
  Carousel,
  CarouselContent,
  CarouselEdgeFade,
  CarouselItem,
  CarouselNext,
  CarouselPrevious,
} from "./components/derived/carousel";
export type {
  CarouselProps,
} from "./components/derived/carousel";
export {
  InputOTP,
  InputOTPGroup,
  InputOTPSeparator,
  InputOTPSlot,
} from "./components/atomic/input-otp";
export type {
  InputOTPProps,
  InputOTPSlotProps,
} from "./components/atomic/input-otp";

// Tier C — overlay/portal components.
export {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "./components/atomic/tooltip";
export type {
  TooltipContentProps,
  TooltipProps,
  TooltipProviderProps,
  TooltipTriggerProps,
} from "./components/atomic/tooltip";
export {
  Popover,
  PopoverAnchor,
  PopoverContent,
  PopoverTrigger,
} from "./components/atomic/popover";
export type {
  PopoverAnchorProps,
  PopoverContentProps,
  PopoverProps,
  PopoverTriggerProps,
} from "./components/atomic/popover";
export {
  InfoTip,
  InfoTipContent,
  InfoTipTrigger,
} from "./components/atomic/info-tip";
export type {
  InfoTipContentProps,
  InfoTipProps,
  InfoTipTriggerProps,
} from "./components/atomic/info-tip";
export {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "./components/atomic/hover-card";
export type {
  HoverCardContentProps,
  HoverCardProps,
  HoverCardTriggerProps,
} from "./components/atomic/hover-card";
export {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuPortal,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
} from "./components/atomic/dropdown-menu";
export type {
  DropdownMenuCheckboxItemProps,
  DropdownMenuContentProps,
  DropdownMenuItemProps,
  DropdownMenuLabelProps,
  DropdownMenuProps,
  DropdownMenuRadioGroupProps,
  DropdownMenuRadioItemProps,
  DropdownMenuSubProps,
  DropdownMenuSubTriggerProps,
  DropdownMenuTriggerProps,
} from "./components/atomic/dropdown-menu";
export {
  ContextMenu,
  ContextMenuCheckboxItem,
  ContextMenuContent,
  ContextMenuGroup,
  ContextMenuItem,
  ContextMenuLabel,
  ContextMenuPortal,
  ContextMenuRadioGroup,
  ContextMenuRadioItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuSub,
  ContextMenuSubContent,
  ContextMenuSubTrigger,
  ContextMenuTrigger,
} from "./components/atomic/context-menu";
export type {
  ContextMenuCheckboxItemProps,
  ContextMenuItemProps,
  ContextMenuLabelProps,
  ContextMenuProps,
  ContextMenuRadioGroupProps,
  ContextMenuRadioItemProps,
  ContextMenuSubProps,
  ContextMenuSubTriggerProps,
  ContextMenuTriggerProps,
} from "./components/atomic/context-menu";
export {
  Menubar,
  MenubarCheckboxItem,
  MenubarContent,
  MenubarGroup,
  MenubarItem,
  MenubarLabel,
  MenubarMenu,
  MenubarRadioGroup,
  MenubarRadioItem,
  MenubarSeparator,
  MenubarShortcut,
  MenubarSub,
  MenubarSubContent,
  MenubarSubTrigger,
  MenubarTrigger,
} from "./components/atomic/menubar";
export type {
  MenubarMenuProps,
  MenubarRadioGroupProps,
  MenubarSubProps,
} from "./components/atomic/menubar";
export {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuIndicator,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
  NavigationMenuViewport,
  navigationMenuTriggerStyle,
} from "./components/atomic/navigation-menu";
export type {
  NavigationMenuItemProps,
} from "./components/atomic/navigation-menu";
export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
} from "./components/atomic/dialog";
export type {
  DialogCloseProps,
  DialogContentProps,
  DialogOverlayProps,
  DialogPortalProps,
  DialogProps,
  DialogTriggerProps,
} from "./components/atomic/dialog";
export {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogOverlay,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "./components/atomic/alert-dialog";
export type {
  AlertDialogActionProps,
  AlertDialogCancelProps,
  AlertDialogContentProps,
  AlertDialogOverlayProps,
  AlertDialogProps,
  AlertDialogTriggerProps,
} from "./components/atomic/alert-dialog";
export {
  Sheet,
  SheetClose,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetOverlay,
  SheetTitle,
  SheetTrigger,
} from "./components/atomic/sheet";
export type {
  SheetCloseProps,
  SheetContentProps,
  SheetOverlayProps,
  SheetProps,
  SheetTriggerProps,
} from "./components/atomic/sheet";
export {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerOverlay,
  DrawerTitle,
  DrawerTrigger,
} from "./components/atomic/drawer";
export type {
  DrawerCloseProps,
  DrawerDirection,
  DrawerProps,
  DrawerTriggerProps,
} from "./components/atomic/drawer";
export {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectSeparator,
  SelectTrigger,
  SelectValue,
} from "./components/atomic/select";
export type {
  SelectContentProps,
  SelectItemProps,
  SelectProps,
  SelectTriggerProps,
  SelectTriggerSize,
  SelectValueProps,
} from "./components/atomic/select";
export {
  NativeSelect,
  NativeSelectGroup,
  NativeSelectOption,
} from "./components/atomic/native-select";
export type {
  NativeSelectProps,
  NativeSelectSize,
} from "./components/atomic/native-select";
export {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut,
} from "./components/atomic/command";
export type {
  CommandDialogProps,
  CommandGroupProps,
  CommandItemProps,
  CommandProps,
} from "./components/atomic/command";

// Tier D — heavy engines (dep-light: see README Limitations).
export {
  Calendar,
} from "./components/atomic/calendar";
export type {
  CalendarProps,
} from "./components/atomic/calendar";
export {
  DateTimePicker,
} from "./components/derived/date-time-picker";
export type {
  DateTimePickerLabels,
  DateTimePickerProps,
} from "./components/derived/date-time-picker";
export {
  Toaster,
  toast,
} from "./components/atomic/sonner";
export type {
  Toast,
  ToastFn,
  ToastOptions,
  ToastPosition,
  ToastState,
  ToasterProps,
} from "./components/atomic/sonner";
export {
  Form,
  FormControl,
  FormDescription,
  FormItem,
  FormLabel,
  FormMessage,
} from "./components/derived/form";

// App shell — TS-only: the frame a signed-in app's screens sit in. Router-agnostic
// (`linkComponent` + `pathname`); motion is CSS in `motion.css`.
export { AppShell } from "./components/shell/app-shell";
export type { AppShellProps, ShellBreakpoint } from "./components/shell/app-shell";

export {
  isNavItemActive,
  isPlainLeftClick,
  navItemMatch,
  resolveActiveNavItem,
} from "./components/shell/nav-model";
export type { NavGroup, NavItem, NavItemState, NavMatch } from "./components/shell/nav-model";
export type { NavCollapseLabels, NavGroupCollapse } from "./components/shell/nav-collapse";
export { usePendingNavigation } from "./components/shell/use-shell-nav";

export { ShellNav } from "./components/shell/shell-nav";
export type { ShellNavLabels, ShellNavProps } from "./components/shell/shell-nav";

export { NavBadge, NavDot } from "./components/shell/nav-badge";
export type { NavBadgeProps, NavBadgeVariant, NavDotProps, NavDotTone } from "./components/shell/nav-badge";

export { BottomTabBar } from "./components/shell/bottom-tab-bar";
export type { BottomTabBarLabels, BottomTabBarProps } from "./components/shell/bottom-tab-bar";

export { Hotbar } from "./components/shell/hotbar";
export type { HotbarAction, HotbarLabels, HotbarProps } from "./components/shell/hotbar";

export { TopBar } from "./components/shell/top-bar";
export type { TopBarProps } from "./components/shell/top-bar";

export { AccountMenu } from "./components/shell/account-menu";
export type {
  AccountMenuLabels,
  AccountMenuProfile,
  AccountMenuProps,
  AccountMenuTab,
} from "./components/shell/account-menu";

export { MobileAppBar } from "./components/shell/mobile-app-bar";
export type { MobileAppBarBack, MobileAppBarLabels, MobileAppBarProps } from "./components/shell/mobile-app-bar";

export { PageFrame, PageHeading, SectionLabel } from "./components/shell/page-frame";
export type {
  PageFrameProps,
  PageFrameWidth,
  PageHeadingProps,
  SectionLabelProps,
  SectionLabelTone,
} from "./components/shell/page-frame";

export { SectionNav } from "./components/shell/section-nav";
export type {
  SectionNavGroup,
  SectionNavItem,
  SectionNavLabels,
  SectionNavProps,
} from "./components/shell/section-nav";

export { ResourceError } from "./components/shell/resource-error";
export type { ResourceErrorLabels, ResourceErrorProps } from "./components/shell/resource-error";

export { SystemBanner } from "./components/shell/system-banner";
export type { SystemBannerLabels, SystemBannerProps } from "./components/shell/system-banner";

export { Settled } from "./components/shell/settled";
export type { SettledProps } from "./components/shell/settled";
export { AnimatedNumber } from "./components/derived/animated-number";
export type { AnimatedNumberProps } from "./components/derived/animated-number";
export { useCountTween } from "./components/atomic/use-count-tween";
export type { CountTweenOptions } from "./components/atomic/use-count-tween";

// Uikit 2.0 elements and composed parts
export { IconButton } from "./components/derived/icon-button";
export type { IconButtonProps, IconButtonSize, IconButtonVariant } from "./components/derived/icon-button";
export { StatusItem, StatusDot } from "./components/atomic/status-item";
export type { StatusDotProps, StatusItemProps, StatusTone } from "./components/atomic/status-item";
export { WorkspaceTabs, WorkspaceTab } from "./components/shell/workspace-tab";
export type { WorkspaceTabProps, WorkspaceTabsProps } from "./components/shell/workspace-tab";
export { KeyValue } from "./components/atomic/key-value";
export type { KeyValueLabels, KeyValueProps } from "./components/atomic/key-value";
export { FormRow } from "./components/atomic/form-row";
export type { FormRowProps } from "./components/atomic/form-row";
export { SettingsLayout, SettingsSection, SettingsRow } from "./components/derived/settings";
export type { SettingsLayoutProps, SettingsRowProps, SettingsSectionProps } from "./components/derived/settings";
export { AccountSettingsDialog } from "./components/shell/account-settings-dialog";
export type {
  AccountSettingsDialogLabels,
  AccountSettingsDialogProps,
  AccountSettingsSection,
} from "./components/shell/account-settings-dialog";
export { ConfirmDialog } from "./components/derived/confirm-dialog";
export type { ConfirmDialogLabels, ConfirmDialogProps } from "./components/derived/confirm-dialog";
export { AccountList, AccountListItem } from "./components/shell/account-list";
export type { AccountListItemProps, AccountListProps } from "./components/shell/account-list";
export { PopoverForm } from "./components/derived/popover-form";
export type { PopoverFormLabels, PopoverFormProps } from "./components/derived/popover-form";
export { CommandPalette, useCommandPaletteShortcut } from "./components/derived/command-palette";
export type {
  CommandPaletteGroup,
  CommandPaletteItem,
  CommandPaletteLabels,
  CommandPaletteProps,
} from "./components/derived/command-palette";
export { HeaderBalance, HeaderSearch, NotificationBell } from "./components/shell/header-controls";
export type {
  HeaderBalanceProps,
  HeaderNotification,
  HeaderSearchProps,
  NotificationBellLabels,
  NotificationBellProps,
} from "./components/shell/header-controls";
export { SmartHeader } from "./components/shell/smart-header";
export type { SmartHeaderProps, SmartHeaderState } from "./components/shell/smart-header";
export { AsideHeader, AsidePanel, AsideRail } from "./components/shell/aside";
export type {
  AsideHeaderLabels,
  AsideHeaderProps,
  AsidePanelMode,
  AsidePanelProps,
  AsideRailItem,
  AsideRailProps,
} from "./components/shell/aside";
export { ChatAttachment, ChatBubble, ChatMarker, ChatMessage } from "./components/shell/chat-message";
export type {
  ChatAttachmentProps,
  ChatBubbleProps,
  ChatBubbleVariant,
  ChatMarkerProps,
  ChatMessageProps,
} from "./components/shell/chat-message";
export { ChatStreamText } from "./components/shell/chat-stream-text";
export type { ChatStreamTextProps } from "./components/shell/chat-stream-text";
export { ChatTyping } from "./components/shell/chat-typing";
export type { ChatTypingProps } from "./components/shell/chat-typing";
export { ChatComposer, ModelChip } from "./components/shell/chat-composer";
export type {
  ChatComposerLabels,
  ChatComposerProps,
  ModelChipProps,
  ModelOption,
} from "./components/shell/chat-composer";
export { ChatDock, ChatPanel } from "./components/shell/chat-panel";
export type { ChatDockLabels, ChatDockProps, ChatPanelLabels, ChatPanelProps } from "./components/shell/chat-panel";
