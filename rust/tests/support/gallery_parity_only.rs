// Components the visual board leaves out, so its baselines stay put; the parity test renders these too.
#[rustfmt::skip]
#[allow(clippy::type_complexity)] // a `(label, demo fn)` table reads fine as-is
const PARITY_ONLY: &[(&str, fn() -> Element)] = &[
	("AlertDialogTrigger", p_alert_dialog_trigger), ("AuthWall", p_auth_wall), ("AvatarImage", p_avatar_image), ("Band", p_band),
	("BreadcrumbEllipsis", p_breadcrumb_ellipsis), ("ButtonGroupParts", p_button_group_parts), ("CardAction", p_card_action),
	("CarouselEdgeFade", p_carousel_edge_fade), ("CommandParts", p_command_parts), ("CommandSiblings", p_command_siblings), ("ContextMenuParts", p_context_menu_parts),
	("DialogParts", p_dialog_parts), ("DrawerParts", p_drawer_parts), ("DropdownMenuParts", p_dropdown_menu_parts),
	("FieldParts", p_field_parts), ("Fonts", p_fonts), ("Footer", p_footer), ("FormMessage", p_form_message), ("InfoTip", p_info_tip),
	("InputGroupParts", p_input_group_parts), ("ItemParts", p_item_parts), ("Logo", p_logo), ("MenubarParts", p_menubar_parts),
	("NavigationMenuParts", p_navigation_menu_parts), ("PopoverAnchor", p_popover_anchor), ("ScrollBar", p_scroll_bar),
	("SectionDescriptor", p_section_descriptor), ("SelectParts", p_select_parts), ("SheetParts", p_sheet_parts),
	("SidebarParts", p_sidebar_parts), ("StatusScreen", p_status_screen), ("TableParts", p_table_parts), ("Toaster", p_toaster),
	("TradesTapeRow", p_trades_tape_row),
	("CollapsibleSiblings", p_collapsible_siblings), ("AccordionSiblings", p_accordion_siblings), ("TabsSiblings", p_tabs_siblings),
	("RadioGroupSiblings", p_radio_group_siblings), ("ToggleGroupSiblings", p_toggle_group_siblings), ("CarouselSiblings", p_carousel_siblings),
	("InputOTPSiblings", p_input_otp_siblings), ("SidebarSiblings", p_sidebar_siblings),
	("SelectSiblings", p_select_siblings), ("DialogSiblings", p_dialog_siblings), ("PopupSiblings", p_popup_siblings), ("MenuSiblings", p_menu_siblings),
];

fn p_alert_dialog_trigger() -> Element {
	rsx! {
		AlertDialog { default_open: true,
			AlertDialogTrigger { "Delete account" }
			AlertDialogContent {
				AlertDialogHeader {
					AlertDialogTitle { "Are you absolutely sure?" }
					AlertDialogDescription { "This action cannot be undone." }
				}
				AlertDialogFooter {
					AlertDialogCancel { "Cancel" }
					AlertDialogAction { "Continue" }
				}
			}
		}
	}
}

fn p_auth_wall() -> Element {
	rsx! {
		AuthWall { title: "Sign in required", description: "Log in to see your portfolio.",
			Button { "Sign in" }
		}
	}
}

fn p_avatar_image() -> Element {
	rsx! {
		Avatar {
			AvatarImage { src: "/avatar.png", alt: "Daneel Olivaw" }
			AvatarFallback { "DO" }
		}
	}
}

fn p_band() -> Element {
	rsx! {
		Section { id: "thesis".to_string(),
			SectionHead { eyebrow: "Thesis", title: "Coastal development", lede: "Emerging Vietnam, institutional grade.".to_string() }
			Eyebrow { "Overview" }
			Display { "Quy Nhon Fund" }
			Prose { "Projected IRR 18.4% over a 5-year horizon." }
			Stat { figure: "18.4%", label: "Target IRR" }
			Check {}
		}
	}
}

fn p_breadcrumb_ellipsis() -> Element {
	rsx! {
		Breadcrumb {
			BreadcrumbList {
				BreadcrumbItem {
					BreadcrumbLink { href: "/".to_string(), "Home" }
				}
				BreadcrumbSeparator {}
				BreadcrumbItem {
					BreadcrumbEllipsis {}
				}
				BreadcrumbSeparator {}
				BreadcrumbItem {
					BreadcrumbPage { "Quy Nhon" }
				}
			}
		}
	}
}

fn p_button_group_parts() -> Element {
	rsx! {
		ButtonGroup {
			ButtonGroupText { "Range" }
			ButtonGroupSeparator {}
			Button { variant: ButtonVariant::Outline, "Day" }
			Button { variant: ButtonVariant::Outline, "Week" }
		}
	}
}

fn p_card_action() -> Element {
	rsx! {
		Card { class: "w-72",
			CardHeader {
				CardTitle { "Quy Nhon Fund" }
				CardDescription { "Coastal development thesis" }
				CardAction {
					Button { size: Size::Sm, "Invest" }
				}
			}
			CardContent { "Projected IRR 18.4% over a 5-year horizon." }
		}
	}
}

fn p_carousel_edge_fade() -> Element {
	rsx! {
		Carousel { class: "w-64",
			CarouselContent {
				CarouselItem { "Slide 1" }
				CarouselItem { "Slide 2" }
			}
			CarouselEdgeFade {}
		}
	}
}

fn p_command_parts() -> Element {
	rsx! {
		CommandDialog { default_open: true,
			CommandInput { placeholder: "Type a command…" }
			CommandList {
				CommandGroup { heading: "Suggestions",
					CommandItem { value: "calendar", "Calendar" }
					CommandSeparator {}
					CommandItem { value: "search", "Search funds"
						CommandShortcut { "⌘K" }
					}
				}
			}
		}
	}
}

// Two instances side by side: each one's parts must find their own Command and group.
fn p_command_siblings() -> Element {
	rsx! {
		for (search, rows) in [("cle", ["cleaning", "plumbing"]), ("lyo", ["Paris", "Lyon"])] {
			Command { default_search: search,
				CommandInput {}
				CommandList {
					CommandEmpty { "None." }
					CommandGroup { heading: search,
						for row in rows {
							CommandItem { value: row, {row} }
						}
					}
				}
			}
		}
	}
}

fn p_context_menu_parts() -> Element {
	rsx! {
		ContextMenu { default_open: true,
			ContextMenuTrigger { "Right-click area" }
			ContextMenuContent {
				ContextMenuGroup {
					ContextMenuLabel { "View" }
					ContextMenuCheckboxItem { checked: true, "Show grid" }
				}
				ContextMenuSeparator {}
				ContextMenuRadioGroup { default_value: "day",
					ContextMenuRadioItem { value: "day", "Day" }
					ContextMenuRadioItem { value: "week", "Week" }
				}
				ContextMenuItem { "Reload"
					ContextMenuShortcut { "⌘R" }
				}
				ContextMenuSub { default_open: true,
					ContextMenuSubTrigger { "More" }
					ContextMenuSubContent {
						ContextMenuItem { "Export" }
					}
				}
			}
		}
	}
}

fn p_dialog_parts() -> Element {
	rsx! {
		Dialog { default_open: true,
			DialogTrigger { "Edit profile" }
			DialogPortal {
				DialogContent {
					DialogHeader {
						DialogTitle { "Edit profile" }
						DialogDescription { "Update your account details." }
					}
					DialogFooter {
						DialogClose { "Cancel" }
					}
				}
			}
		}
	}
}

fn p_drawer_parts() -> Element {
	rsx! {
		Drawer { default_open: true,
			DrawerTrigger { "Open drawer" }
			DrawerOverlay {}
			DrawerContent {
				DrawerHeader {
					DrawerTitle { "Direction" }
					DrawerDescription { "Slide-up drawer." }
				}
				DrawerFooter {
					DrawerClose { "Close" }
				}
			}
		}
	}
}

fn p_dropdown_menu_parts() -> Element {
	rsx! {
		DropdownMenu { default_open: true,
			DropdownMenuTrigger { "Menu" }
			DropdownMenuContent {
				DropdownMenuGroup {
					DropdownMenuLabel { "View" }
					DropdownMenuCheckboxItem { checked: true, "Show grid" }
				}
				DropdownMenuSeparator {}
				DropdownMenuRadioGroup { default_value: "day",
					DropdownMenuRadioItem { value: "day", "Day" }
					DropdownMenuRadioItem { value: "week", "Week" }
				}
				DropdownMenuItem { "Reload"
					DropdownMenuShortcut { "⌘R" }
				}
				DropdownMenuSub { default_open: true,
					DropdownMenuSubTrigger { "More" }
					DropdownMenuSubContent {
						DropdownMenuItem { "Export" }
					}
				}
			}
		}
	}
}

fn p_field_parts() -> Element {
	rsx! {
		FieldSet { class: "w-72",
			FieldGroup {
				Field {
					FieldContent {
						FieldTitle { "Two-factor" }
						FieldDescription { "Protects your account." }
					}
					Switch { default_checked: true }
				}
				FieldSeparator { content: true, "or" }
				Field {
					FieldLabel { "Email" }
					Input { placeholder: "daneel@example.com" }
					FieldError { "Enter a valid email." }
				}
			}
		}
	}
}

fn p_fonts() -> Element {
	rsx! { Fonts {} }
}

fn p_footer() -> Element {
	let nav = vec![FooterLinkGroup {
		heading: "Company".to_string(),
		links: vec![FooterLink {
			label: "Team".to_string(),
			href: "/team".to_string(),
		}],
	}];
	rsx! {
		Footer { brand: "ACME", description: "We do a thing.", copyright: "© 2026 ACME", nav }
	}
}

fn p_form_message() -> Element {
	rsx! {
		Form { class: "w-72",
			FormItem {
				FormLabel { error: true, "Username" }
				FormControl { error: true,
					Input { placeholder: "daneel" }
				}
				FormMessage { error: true, "Username is taken." }
			}
			// A sibling item: its ids must be its own.
			FormItem {
				FormLabel { "Email" }
				FormControl {
					Input { placeholder: "daneel@example.com" }
				}
				FormDescription { "We never share it." }
				FormMessage { "Unused while valid." }
			}
		}
	}
}

fn p_info_tip() -> Element {
	rsx! {
		InfoTip { default_open: true,
			InfoTipTrigger {}
			InfoTipContent { "Net of fees." }
		}
	}
}

fn p_input_group_parts() -> Element {
	rsx! {
		div { class: "flex w-72 flex-col gap-2",
			InputGroup {
				InputGroupText { "https://" }
				InputGroupInput { placeholder: "example.com" }
				InputGroupAddon { align: InputGroupAddonAlign::InlineEnd,
					InputGroupButton { "Copy" }
				}
			}
			InputGroup {
				InputGroupTextarea { placeholder: "Notes…" }
			}
		}
	}
}

fn p_item_parts() -> Element {
	rsx! {
		ItemGroup { class: "w-80",
			Item { variant: ItemVariant::Outline,
				ItemHeader { "Position" }
				ItemContent {
					ItemTitle { "Ha Long" }
					ItemDescription { "Tier 1" }
				}
				ItemActions {
					Button { size: Size::Sm, "Open" }
				}
				ItemFooter { "Updated Monday" }
			}
			ItemSeparator {}
			Item {
				ItemContent {
					ItemTitle { "Jungle" }
				}
			}
		}
	}
}

fn p_logo() -> Element {
	rsx! { Logo { class: "size-8" } }
}

fn p_menubar_parts() -> Element {
	rsx! {
		Menubar {
			MenubarMenu { default_open: true,
				MenubarTrigger { "View" }
				MenubarContent {
					MenubarGroup {
						MenubarLabel { "Panels" }
						MenubarCheckboxItem { checked: true, "Show grid" }
					}
					MenubarSeparator {}
					MenubarRadioGroup { default_value: "day",
						MenubarRadioItem { value: "day", "Day" }
						MenubarRadioItem { value: "week", "Week" }
					}
					MenubarItem { "Reload"
						MenubarShortcut { "⌘R" }
					}
					MenubarSub { default_open: true,
						MenubarSubTrigger { "More" }
						MenubarSubContent {
							MenubarItem { "Export" }
						}
					}
				}
			}
		}
	}
}

fn p_navigation_menu_parts() -> Element {
	rsx! {
		NavigationMenu {
			NavigationMenuList {
				NavigationMenuItem { default_open: true,
					NavigationMenuTrigger { "Funds" }
					NavigationMenuContent {
						NavigationMenuLink { "Coastal" }
					}
				}
			}
			NavigationMenuIndicator {}
			NavigationMenuViewport { "Viewport" }
		}
	}
}

fn p_popover_anchor() -> Element {
	rsx! {
		Popover { default_open: true,
			PopoverAnchor {
				PopoverTrigger {
					Button { variant: ButtonVariant::Outline, "Open popover" }
				}
			}
			PopoverContent { "Popover body" }
		}
	}
}

fn p_scroll_bar() -> Element {
	rsx! {
		ScrollArea { class: "h-32 w-56 rounded-md border border-border p-4",
			div { "Row 1" }
			div { "Row 2" }
			ScrollBar {}
			ScrollBar { orientation: ScrollBarOrientation::Horizontal }
		}
	}
}

fn p_section_descriptor() -> Element {
	rsx! {
		SectionDescriptor { title: "Risks",
			"Currency, liquidity, and regulatory."
		}
		SectionDescriptor { title: "Thesis", collapsible: true, default_open: true,
			"Coastal development in emerging Vietnam."
		}
	}
}

fn p_select_parts() -> Element {
	rsx! {
		Select { default_open: true, default_value: "jungle".to_string(), class: "w-56",
			SelectTrigger {
				SelectValue { placeholder: "Pick a fund" }
			}
			SelectContent {
				SelectGroup {
					SelectLabel { "Coastal" }
					SelectItem { value: "ha-long", text_value: "Ha Long Teal", "Ha Long Teal" }
				}
				SelectSeparator {}
				SelectGroup {
					SelectLabel { "Inland" }
					SelectItem { value: "jungle", text_value: "Jungle Green", "Jungle Green" }
				}
			}
		}
	}
}

fn p_sheet_parts() -> Element {
	rsx! {
		Sheet { default_open: true,
			SheetTrigger { "Filters" }
			SheetContent {
				SheetHeader {
					SheetTitle { "Filters" }
					SheetDescription { "Refine the fund list." }
				}
				SheetFooter {
					SheetClose { "Done" }
				}
			}
		}
	}
}

fn p_sidebar_parts() -> Element {
	rsx! {
		SidebarProvider {
			Sidebar {
				SidebarHeader { "EV Invest" }
				SidebarContent {
					SidebarGroup {
						SidebarGroupContent {
							SidebarMenu {
								SidebarMenuItem {
									SidebarMenuButton { is_active: true, "Dashboard" }
								}
							}
						}
					}
				}
				SidebarSeparator {}
				SidebarFooter { "v1.0" }
				SidebarRail {}
			}
			SidebarInset {
				SidebarTrigger { "Toggle" }
				"Main content"
			}
		}
	}
}

fn p_status_screen() -> Element {
	rsx! {
		StatusScreen {
			accent: Accent::Debug,
			eyebrow: "Teapot",
			code: "418",
			headline_lead: "Short and ",
			headline_accent: "stout",
			subtext: "The server refuses to brew coffee.",
			links: vec![StatusLinkData { label: "Back to home".to_string(), href: "/".to_string(), variant: ButtonVariant::Primary, leading_arrow: true }],
		}
		NotFound {}
		Forbidden {}
		ServerError {}
	}
}

fn p_table_parts() -> Element {
	rsx! {
		Table { class: "w-full max-w-md",
			TableCaption { "Yields by asset" }
			TableHeader {
				TableRow {
					TableHead { "Asset" }
					TableHead { "Yield" }
				}
			}
			TableBody {
				TableRow {
					TableCell { "Ha Long" }
					TableCell { "12.0%" }
				}
			}
			TableFooter {
				TableRow {
					TableCell { "Total" }
					TableCell { "12.0%" }
				}
			}
		}
	}
}

#[component]
fn QueueToast() -> Element {
	let toaster = use_toaster();
	use_hook(|| toaster.show("Order filled", ToastVariant::Positive, None));
	rsx! {}
}

fn p_toaster() -> Element {
	rsx! {
		ToasterProvider {
			QueueToast {}
			Toaster {}
		}
		// A sibling provider keeps its own queue.
		ToasterProvider {
			QueueToast {}
			QueueToast {}
			Toaster { position: ToastPosition::TopLeft }
		}
	}
}

fn p_trades_tape_row() -> Element {
	rsx! {
		div { class: "flex w-72 flex-col",
			TradesTapeRow { side: BookSide::Bid, price: "1.024", size: "140", time: "12:00:01" }
			TradesTapeRow { side: BookSide::Ask, price: "1.026", size: "90", time: "12:00:02" }
		}
	}
}

fn p_select_siblings() -> Element {
	rsx! {
		Select { default_open: true, default_value: "a".to_string(),
			SelectTrigger { SelectValue {} }
			SelectContent {
				SelectItem { value: "a", text_value: "A", "A" }
				SelectItem { value: "b", text_value: "B", "B" }
			}
		}
		Select { default_value: "b".to_string(),
			SelectTrigger { SelectValue {} }
			SelectContent {
				SelectItem { value: "a", text_value: "A", "A" }
				SelectItem { value: "b", text_value: "B", "B" }
			}
		}
	}
}

fn p_dialog_siblings() -> Element {
	rsx! {
		Dialog { default_open: true,
			DialogTrigger { "first" }
			DialogContent { "first dialog" }
		}
		Dialog {
			DialogTrigger { "second" }
			DialogContent { "second dialog" }
		}
		AlertDialog { default_open: true,
			AlertDialogContent { "first alert" }
		}
		AlertDialog {
			AlertDialogContent { "second alert" }
		}
		Sheet { default_open: true,
			SheetContent { "first sheet" }
		}
		Sheet {
			SheetContent { "second sheet" }
		}
		Drawer { default_open: true, direction: DrawerDirection::Right,
			DrawerContent { "first drawer" }
		}
		Drawer {
			DrawerContent { "second drawer" }
		}
	}
}

fn p_popup_siblings() -> Element {
	rsx! {
		Popover { default_open: true,
			PopoverTrigger { "first" }
			PopoverContent { "first popover" }
		}
		Popover {
			PopoverTrigger { "second" }
			PopoverContent { "second popover" }
		}
		Tooltip { default_open: true,
			TooltipTrigger { "first" }
			TooltipContent { "first tooltip" }
		}
		Tooltip {
			TooltipTrigger { "second" }
			TooltipContent { "second tooltip" }
		}
		HoverCard { default_open: true,
			HoverCardTrigger { "first" }
			HoverCardContent { "first card" }
		}
		HoverCard {
			HoverCardTrigger { "second" }
			HoverCardContent { "second card" }
		}
		InfoTip { default_open: true,
			InfoTipTrigger {}
			InfoTipContent { "first tip" }
		}
		InfoTip {
			InfoTipTrigger {}
			InfoTipContent { "second tip" }
		}
	}
}

fn p_menu_siblings() -> Element {
	rsx! {
		DropdownMenu { default_open: true,
			DropdownMenuTrigger { "first" }
			DropdownMenuContent {
				DropdownMenuRadioGroup { default_value: "day",
					DropdownMenuRadioItem { value: "day", "Day" }
				}
				DropdownMenuRadioGroup { default_value: "week",
					DropdownMenuRadioItem { value: "day", "Day" }
				}
				DropdownMenuSub { default_open: true,
					DropdownMenuSubTrigger { "open sub" }
					DropdownMenuSubContent { "first sub" }
				}
				DropdownMenuSub {
					DropdownMenuSubTrigger { "closed sub" }
					DropdownMenuSubContent { "second sub" }
				}
			}
		}
		DropdownMenu {
			DropdownMenuTrigger { "second" }
			DropdownMenuContent { "second menu" }
		}
		ContextMenu { default_open: true,
			ContextMenuTrigger { "first" }
			ContextMenuContent {
				ContextMenuRadioGroup { default_value: "day",
					ContextMenuRadioItem { value: "day", "Day" }
				}
				ContextMenuRadioGroup { default_value: "week",
					ContextMenuRadioItem { value: "day", "Day" }
				}
				ContextMenuSub { default_open: true,
					ContextMenuSubTrigger { "open sub" }
					ContextMenuSubContent { "first sub" }
				}
				ContextMenuSub {
					ContextMenuSubTrigger { "closed sub" }
					ContextMenuSubContent { "second sub" }
				}
			}
		}
		ContextMenu {
			ContextMenuTrigger { "second" }
			ContextMenuContent { "second menu" }
		}
		Menubar {
			MenubarMenu { default_open: true,
				MenubarTrigger { "File" }
				MenubarContent {
					MenubarRadioGroup { default_value: "day",
						MenubarRadioItem { value: "day", "Day" }
					}
					MenubarRadioGroup { default_value: "week",
						MenubarRadioItem { value: "day", "Day" }
					}
					MenubarSub { default_open: true,
						MenubarSubTrigger { "open sub" }
						MenubarSubContent { "first sub" }
					}
					MenubarSub {
						MenubarSubTrigger { "closed sub" }
						MenubarSubContent { "second sub" }
					}
				}
			}
			MenubarMenu {
				MenubarTrigger { "Edit" }
				MenubarContent { "second menu" }
			}
		}
		NavigationMenu {
			NavigationMenuList {
				NavigationMenuItem { default_open: true,
					NavigationMenuTrigger { "Funds" }
					NavigationMenuContent { "first panel" }
				}
				NavigationMenuItem {
					NavigationMenuTrigger { "About" }
					NavigationMenuContent { "second panel" }
				}
			}
		}
	}
}

// Sibling instances of each component that provides context to its parts: the
// Leptos port must scope it per instance, not leak the last sibling's.
fn p_collapsible_siblings() -> Element {
	rsx! {
		Collapsible { default_open: true,
			CollapsibleTrigger { "Show" }
			CollapsibleContent {
				CollapsibleTrigger { "Hide" }
			}
		}
		Collapsible {
			CollapsibleTrigger { "Show" }
			CollapsibleContent { "B" }
		}
	}
}
fn p_accordion_siblings() -> Element {
	rsx! {
		Accordion { default_value: vec!["a".to_string()],
			AccordionItem { value: "a",
				AccordionTrigger { "A" }
				AccordionContent { "Body A" }
			}
		}
		Accordion { default_value: vec!["b".to_string()],
			AccordionItem { value: "a",
				AccordionTrigger { "A" }
				AccordionContent { "Body A" }
			}
		}
	}
}
fn p_tabs_siblings() -> Element {
	rsx! {
		Tabs { default_value: "one",
			TabsList {
				TabsTrigger { value: "one", "One" }
				TabsTrigger { value: "two", "Two" }
			}
			TabsContent { value: "one", "Panel one" }
		}
		Tabs { default_value: "two", orientation: TabsOrientation::Vertical,
			TabsList {
				TabsTrigger { value: "one", "One" }
				TabsTrigger { value: "two", "Two" }
			}
			TabsContent { value: "two", "Panel two" }
		}
	}
}
fn p_radio_group_siblings() -> Element {
	rsx! {
		RadioGroup { default_value: "a",
			RadioGroupItem { value: "a" }
			RadioGroupItem { value: "b" }
		}
		RadioGroup { default_value: "b",
			RadioGroupItem { value: "a" }
			RadioGroupItem { value: "b" }
		}
	}
}
fn p_toggle_group_siblings() -> Element {
	rsx! {
		ToggleGroup { variant: ToggleVariant::Outline, default_value: vec!["x".to_string()],
			ToggleGroupItem { value: "x", "X" }
		}
		ToggleGroupItem { value: "x", "Outside" }
	}
}
fn p_carousel_siblings() -> Element {
	rsx! {
		Carousel {
			CarouselContent {
				CarouselItem { "1" }
				CarouselItem { "2" }
			}
			CarouselNext {}
		}
		Carousel { orientation: CarouselOrientation::Vertical,
			CarouselContent {
				CarouselItem { "1" }
			}
			CarouselNext {}
		}
	}
}
fn p_input_otp_siblings() -> Element {
	rsx! {
		InputOTP { max_length: 2, value: "1".to_string(),
			InputOTPSlot { index: 0 }
			InputOTPSlot { index: 1 }
		}
		InputOTP { max_length: 2,
			InputOTPSlot { index: 0 }
			InputOTPSlot { index: 1 }
		}
	}
}
fn p_sidebar_siblings() -> Element {
	rsx! {
		SidebarProvider {
			Sidebar { "A" }
		}
		SidebarProvider { default_open: false,
			Sidebar { collapsible: SidebarCollapsible::Icon, "B" }
		}
	}
}
