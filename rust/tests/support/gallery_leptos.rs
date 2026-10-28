// The Leptos twin of gallery.rs (and gallery_parity_only.rs): the same labels,
// each demo a 1:1 translation of the Dioxus one. `include!`d by the parity test
// (tests/parity.rs), which renders both sides and compares the HTML. Porting a
// component = its labels here, in the same order as on the Dioxus side.

use ev_lib::uikit_leptos::*;
use leptos::prelude::*;

#[rustfmt::skip]
#[allow(clippy::type_complexity)] // a `(label, demo fn)` table reads fine as-is
const GALLERY: &[(&str, fn() -> AnyView)] = &[
	("Button", d_button), ("ButtonAccent", d_button_accent), ("Badge", d_badge), ("Label", d_label),
	("Kbd", d_kbd), ("Avatar", d_avatar), ("Separator", d_separator), ("Skeleton", d_skeleton),
	("Spinner", d_spinner), ("Progress", d_progress), ("Alert", d_alert), ("Card", d_card),
	("Table", d_table), ("TableCard", d_table_card), ("Breadcrumb", d_breadcrumb), ("Pagination", d_pagination),
	("Empty", d_empty), ("Item", d_item), ("ButtonGroup", d_button_group), ("Input", d_input),
	("Textarea", d_textarea), ("Command", d_command), ("ScrollArea", d_scroll_area), ("Container", d_container),
	("Terminal", d_terminal),
	("Checkbox", d_checkbox), ("Switch", d_switch), ("RadioGroup", d_radio_group), ("Slider", d_slider),
	("Toggle", d_toggle), ("ToggleGroup", d_toggle_group), ("Tabs", d_tabs), ("Accordion", d_accordion),
	("Collapsible", d_collapsible), ("InputOTP", d_input_otp), ("Carousel", d_carousel), ("Sidebar", d_sidebar),
	("Tooltip", d_tooltip), ("Popover", d_popover), ("HoverCard", d_hover_card), ("DropdownMenu", d_dropdown_menu),
	("ContextMenu", d_context_menu), ("Menubar", d_menubar), ("Select", d_select), ("Dialog", d_dialog),
	("AlertDialog", d_alert_dialog), ("Sheet", d_sheet), ("Drawer", d_drawer), ("NavigationMenu", d_navigation_menu),
	("Field", d_field), ("InputGroup", d_input_group), ("Calendar", d_calendar), ("DateTimePicker", d_date_time_picker), ("Form", d_form),
];

#[rustfmt::skip]
#[allow(clippy::type_complexity)] // a `(label, demo fn)` table reads fine as-is
const PARITY_ONLY: &[(&str, fn() -> AnyView)] = &[
	("AuthWall", p_auth_wall), ("AvatarImage", p_avatar_image), ("Band", p_band), ("BreadcrumbEllipsis", p_breadcrumb_ellipsis),
	("ButtonGroupParts", p_button_group_parts), ("CardAction", p_card_action), ("Fonts", p_fonts), ("Footer", p_footer),
	("ItemParts", p_item_parts), ("Logo", p_logo), ("ScrollBar", p_scroll_bar), ("SectionDescriptor", p_section_descriptor),
	("StatusScreen", p_status_screen), ("TableParts", p_table_parts), ("TradesTapeRow", p_trades_tape_row),
	("CarouselEdgeFade", p_carousel_edge_fade), ("SidebarParts", p_sidebar_parts),
	("CollapsibleSiblings", p_collapsible_siblings), ("AccordionSiblings", p_accordion_siblings), ("TabsSiblings", p_tabs_siblings),
	("RadioGroupSiblings", p_radio_group_siblings), ("ToggleGroupSiblings", p_toggle_group_siblings), ("CarouselSiblings", p_carousel_siblings),
	("InputOTPSiblings", p_input_otp_siblings), ("SidebarSiblings", p_sidebar_siblings),
	("AlertDialogTrigger", p_alert_dialog_trigger), ("ContextMenuParts", p_context_menu_parts), ("DialogParts", p_dialog_parts),
	("DrawerParts", p_drawer_parts), ("DropdownMenuParts", p_dropdown_menu_parts), ("InfoTip", p_info_tip), ("MenubarParts", p_menubar_parts),
	("NavigationMenuParts", p_navigation_menu_parts), ("PopoverAnchor", p_popover_anchor), ("SelectParts", p_select_parts), ("SheetParts", p_sheet_parts),
	("FieldParts", p_field_parts), ("FormMessage", p_form_message), ("InputGroupParts", p_input_group_parts), ("Toaster", p_toaster),
	("CommandParts", p_command_parts), ("CommandSiblings", p_command_siblings),
	("SelectSiblings", p_select_siblings), ("DialogSiblings", p_dialog_siblings), ("PopupSiblings", p_popup_siblings), ("MenuSiblings", p_menu_siblings),
];

// ── Tier A — static ────────────────────────────────────────────────────────

fn d_button() -> AnyView {
	view! {
		<Button>"Default"</Button>
		<Button variant=ButtonVariant::Secondary>"Secondary"</Button>
		<Button variant=ButtonVariant::Outline>"Outline"</Button>
		<Button variant=ButtonVariant::Ghost>"Ghost"</Button>
		<Button variant=ButtonVariant::Destructive>"Destructive"</Button>
		<Button variant=ButtonVariant::Link>"Link"</Button>
		<Button size=Size::Sm>"Small"</Button>
		<Button size=Size::Lg>"Large"</Button>
		<Button size=Size::Md icon=true>
			"+"
		</Button>
		<Button disabled=true title="Unavailable">
			"Disabled"
		</Button>
	}
	.into_any()
}

fn d_button_accent() -> AnyView {
	const ACCENTS: [Accent; 5] = [Accent::Trace, Accent::Debug, Accent::Info, Accent::Warn, Accent::Error];
	view! {
		{ACCENTS.map(|accent| view! { <Button accent>{accent.as_ref().to_owned()}</Button> })}
		{ACCENTS
			.map(|accent| {
				view! {
					<Button variant=ButtonVariant::Outline accent>
						{accent.as_ref().to_owned()}
					</Button>
				}
			})}
	}
	.into_any()
}

fn d_kbd() -> AnyView {
	view! {
		<KbdGroup>
			<Kbd>"⌘"</Kbd>
			<Kbd>"K"</Kbd>
		</KbdGroup>
	}
	.into_any()
}

fn d_badge() -> AnyView {
	view! {
		<Badge>"Default"</Badge>
		<Badge variant=BadgeVariant::Secondary>"Secondary"</Badge>
		<Badge variant=BadgeVariant::Destructive>"Destructive"</Badge>
		<Badge variant=BadgeVariant::Outline>"Outline"</Badge>
		<Badge variant=BadgeVariant::Success>"Success"</Badge>
	}
	.into_any()
}

fn d_label() -> AnyView {
	view! { <Label>"Email address"</Label> }.into_any()
}

fn d_avatar() -> AnyView {
	view! {
		<Avatar>
			<AvatarFallback>"EV"</AvatarFallback>
		</Avatar>
		<Avatar>
			<AvatarFallback>"AB"</AvatarFallback>
		</Avatar>
	}
	.into_any()
}

fn d_separator() -> AnyView {
	view! { <div class="flex flex-col gap-2 w-40">"Above" <Separator /> "Below"</div> }.into_any()
}

fn d_skeleton() -> AnyView {
	view! {
		<div class="flex flex-col gap-2 w-48">
			<Skeleton class="h-4 w-full" />
			<Skeleton class="h-4 w-2/3" />
			<Skeleton class="h-10 w-10 rounded-full" />
		</div>
	}
	.into_any()
}

fn d_spinner() -> AnyView {
	view! { <Spinner /> }.into_any()
}

fn d_progress() -> AnyView {
	view! {
		<div class="flex w-56 flex-col gap-3">
			<Progress value=30.0 />
			<Progress value=66.0 />
		</div>
	}
	.into_any()
}

fn d_alert() -> AnyView {
	view! {
		<div class="flex w-full max-w-md flex-col gap-3">
			<Alert>
				<AlertTitle>"Heads up"</AlertTitle>
				<AlertDescription>"This is a neutral informational alert."</AlertDescription>
			</Alert>
			<Alert variant=AlertVariant::Destructive>
				<AlertTitle>"Something failed"</AlertTitle>
				<AlertDescription>"Your changes could not be saved."</AlertDescription>
			</Alert>
			<Alert variant=AlertVariant::Success>
				<AlertTitle>"Order filled"</AlertTitle>
				<AlertDescription>"120 units bought at 1.024."</AlertDescription>
			</Alert>
			<Alert variant=AlertVariant::Info>
				<AlertTitle>"Valuation pending"</AlertTitle>
				<AlertDescription>"The next NAV mark lands on Monday."</AlertDescription>
			</Alert>
		</div>
	}
	.into_any()
}

fn d_card() -> AnyView {
	view! {
		<Card class="w-72">
			<CardHeader>
				<CardTitle>"Quy Nhon Fund"</CardTitle>
				<CardDescription>"Coastal development thesis"</CardDescription>
			</CardHeader>
			<CardContent>"Projected IRR 18.4% over a 5-year horizon."</CardContent>
			<CardFooter>
				<Button size=Size::Sm>"Invest"</Button>
			</CardFooter>
		</Card>
	}
	.into_any()
}

fn d_table() -> AnyView {
	view! {
		<Table class="w-full max-w-md">
			<TableHeader>
				<TableRow>
					<TableHead>"Asset"</TableHead>
					<TableHead>"Tier"</TableHead>
					<TableHead>"Yield"</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody>
				<TableRow>
					<TableCell>"Ha Long"</TableCell>
					<TableCell>"t1"</TableCell>
					<TableCell>"12.0%"</TableCell>
				</TableRow>
				<TableRow>
					<TableCell>"Jungle"</TableCell>
					<TableCell>"t2"</TableCell>
					<TableCell>"18.4%"</TableCell>
				</TableRow>
			</TableBody>
		</Table>
	}
	.into_any()
}

fn d_table_card() -> AnyView {
	view! {
		<div class="flex w-full max-w-md flex-col gap-4">
			{[TableDensity::Default, TableDensity::Compact]
				.map(|density| {
					view! {
						<TableCard>
							<Table variant=TableVariant::Card density>
								<TableHeader>
									<TableRow>
										<TableHead>"Asset"</TableHead>
										<TableHead>"Tier"</TableHead>
										<TableHead align=TableAlign::End>"Yield"</TableHead>
									</TableRow>
								</TableHeader>
								<TableBody>
									<TableRow>
										<TableCell>"Ha Long"</TableCell>
										<TableCell>"t1"</TableCell>
										<TableCell align=TableAlign::End>"12.0%"</TableCell>
									</TableRow>
									<TableRow>
										<TableCell>"Jungle"</TableCell>
										<TableCell>"t2"</TableCell>
										<TableCell align=TableAlign::End>"118.4%"</TableCell>
									</TableRow>
								</TableBody>
							</Table>
						</TableCard>
					}
				})}
		</div>
	}
	.into_any()
}

fn d_breadcrumb() -> AnyView {
	view! {
		<Breadcrumb>
			<BreadcrumbList>
				<BreadcrumbItem>
					<BreadcrumbLink href="/">"Home"</BreadcrumbLink>
				</BreadcrumbItem>
				<BreadcrumbSeparator />
				<BreadcrumbItem>
					<BreadcrumbLink href="/funds">"Funds"</BreadcrumbLink>
				</BreadcrumbItem>
				<BreadcrumbSeparator />
				<BreadcrumbItem>
					<BreadcrumbPage>"Quy Nhon"</BreadcrumbPage>
				</BreadcrumbItem>
			</BreadcrumbList>
		</Breadcrumb>
	}
	.into_any()
}

fn d_pagination() -> AnyView {
	view! {
		<Pagination>
			<PaginationContent>
				<PaginationItem>
					<PaginationPrevious href="?page=1" />
				</PaginationItem>
				<PaginationItem>
					<PaginationLink href="?page=1" is_active=true>
						"1"
					</PaginationLink>
				</PaginationItem>
				<PaginationItem>
					<PaginationLink href="?page=2">"2"</PaginationLink>
				</PaginationItem>
				<PaginationItem>
					<PaginationEllipsis />
				</PaginationItem>
				<PaginationItem>
					<PaginationNext href="?page=2" />
				</PaginationItem>
			</PaginationContent>
		</Pagination>
	}
	.into_any()
}

fn d_empty() -> AnyView {
	view! {
		<Empty class="max-w-sm">
			<EmptyHeader>
				<EmptyMedia variant=EmptyMediaVariant::Icon>"★"</EmptyMedia>
				<EmptyTitle>"No investments yet"</EmptyTitle>
				<EmptyDescription>"Add your first position to see it here."</EmptyDescription>
			</EmptyHeader>
			<EmptyContent>
				<Button size=Size::Sm>"Add position"</Button>
			</EmptyContent>
		</Empty>
	}
	.into_any()
}

fn d_item() -> AnyView {
	view! {
		<div class="flex w-80 flex-col gap-2">
			<Item>
				<ItemMedia variant=ItemMediaVariant::Icon>"▲"</ItemMedia>
				<ItemContent>
					<ItemTitle>"Default item"</ItemTitle>
					<ItemDescription>"Transparent surface"</ItemDescription>
				</ItemContent>
			</Item>
			<Item variant=ItemVariant::Outline>
				<ItemContent>
					<ItemTitle>"Outline item"</ItemTitle>
				</ItemContent>
			</Item>
			<Item variant=ItemVariant::Muted>
				<ItemContent>
					<ItemTitle>"Muted item"</ItemTitle>
				</ItemContent>
			</Item>
		</div>
	}
	.into_any()
}

fn d_button_group() -> AnyView {
	view! {
		<ButtonGroup>
			<Button variant=ButtonVariant::Outline>"Day"</Button>
			<Button variant=ButtonVariant::Outline>"Week"</Button>
			<Button variant=ButtonVariant::Outline>"Month"</Button>
		</ButtonGroup>
	}
	.into_any()
}

// ── Tier B — interactive ───────────────────────────────────────────────────

fn d_input() -> AnyView {
	view! {
		<div class="flex w-64 flex-col gap-2">
			<Input placeholder="Default" />
			<Input disabled=true placeholder="Disabled" />
			<Input size=Size::Lg placeholder="Large" />
		</div>
	}
	.into_any()
}

fn d_textarea() -> AnyView {
	view! {
		<div class="flex w-64 flex-col gap-2">
			<Textarea placeholder="Investment thesis…" />
			<Textarea size=Size::Lg placeholder="Large" />
		</div>
	}
	.into_any()
}

fn d_command() -> AnyView {
	view! {
		<Command class="w-72 rounded-lg border border-border">
			<CommandInput placeholder="Type a command…" />
			<CommandList>
				<CommandEmpty>"No results."</CommandEmpty>
				<CommandGroup heading="Suggestions">
					<CommandItem value="calendar">"Calendar"</CommandItem>
					<CommandItem value="search">"Search funds"</CommandItem>
				</CommandGroup>
			</CommandList>
		</Command>
	}
	.into_any()
}

// ── Tier C — overlays ──────────────────────────────────────────────────────

fn d_tooltip() -> AnyView {
	view! {
		<TooltipProvider>
			<Tooltip default_open=true>
				<TooltipTrigger>
					<Button variant=ButtonVariant::Outline>"Hover me"</Button>
				</TooltipTrigger>
				<TooltipContent>"Tooltip content"</TooltipContent>
			</Tooltip>
		</TooltipProvider>
	}
	.into_any()
}

fn d_popover() -> AnyView {
	view! {
		<Popover default_open=true>
			<PopoverTrigger>
				<Button variant=ButtonVariant::Outline>"Open popover"</Button>
			</PopoverTrigger>
			<PopoverContent>
				<div class="flex flex-col gap-2">
					"Popover body" <Button size=Size::Sm>"Action"</Button>
				</div>
			</PopoverContent>
		</Popover>
	}
	.into_any()
}

fn d_hover_card() -> AnyView {
	view! {
		<HoverCard default_open=true>
			<HoverCardTrigger>
				<Button variant=ButtonVariant::Link>"@evinvest"</Button>
			</HoverCardTrigger>
			<HoverCardContent>"Institutional coastal real-estate fund."</HoverCardContent>
		</HoverCard>
	}
	.into_any()
}

fn d_dropdown_menu() -> AnyView {
	view! {
		<DropdownMenu default_open=true>
			<DropdownMenuTrigger>
				<Button variant=ButtonVariant::Outline>"Menu"</Button>
			</DropdownMenuTrigger>
			<DropdownMenuContent>
				<DropdownMenuLabel>"Account"</DropdownMenuLabel>
				<DropdownMenuSeparator />
				<DropdownMenuItem>"Profile"</DropdownMenuItem>
				<DropdownMenuItem>"Settings"</DropdownMenuItem>
				<DropdownMenuItem variant=DropdownMenuItemVariant::Destructive>"Delete"</DropdownMenuItem>
			</DropdownMenuContent>
		</DropdownMenu>
	}
	.into_any()
}

fn d_context_menu() -> AnyView {
	view! {
		<ContextMenu default_open=true>
			<ContextMenuTrigger>
				<div class="flex h-20 w-56 items-center justify-center rounded-md border border-dashed border-border">
					"Right-click area"
				</div>
			</ContextMenuTrigger>
			<ContextMenuContent>
				<ContextMenuItem>"Back"</ContextMenuItem>
				<ContextMenuItem>"Forward"</ContextMenuItem>
				<ContextMenuSeparator />
				<ContextMenuItem variant=ContextMenuItemVariant::Destructive>"Delete"</ContextMenuItem>
			</ContextMenuContent>
		</ContextMenu>
	}
	.into_any()
}

fn d_menubar() -> AnyView {
	view! {
		<Menubar>
			<MenubarMenu default_open=true>
				<MenubarTrigger>"File"</MenubarTrigger>
				<MenubarContent>
					<MenubarItem>"New"</MenubarItem>
					<MenubarItem>"Open"</MenubarItem>
					<MenubarSeparator />
					<MenubarItem variant=MenubarItemVariant::Destructive>"Quit"</MenubarItem>
				</MenubarContent>
			</MenubarMenu>
		</Menubar>
	}
	.into_any()
}

fn d_select() -> AnyView {
	view! {
		<Select default_open=true default_value="jungle" reset_value="rice" class="w-56">
			<SelectTrigger>
				<SelectValue placeholder="Pick a fund" />
			</SelectTrigger>
			<SelectContent>
				<SelectItem value="ha-long" text_value="Ha Long Teal">
					"Ha Long Teal"
				</SelectItem>
				<SelectItem value="jungle" text_value="Jungle Green">
					"Jungle Green"
				</SelectItem>
				<SelectItem value="rice" text_value="Rice Gold">
					"Rice Gold"
				</SelectItem>
			</SelectContent>
		</Select>
	}
	.into_any()
}

fn d_dialog() -> AnyView {
	view! {
		<Dialog default_open=true>
			<DialogContent>
				<DialogHeader>
					<DialogTitle>"Edit profile"</DialogTitle>
					<DialogDescription>"Update your account details."</DialogDescription>
				</DialogHeader>
				<DialogFooter>
					<Button variant=ButtonVariant::Outline>"Cancel"</Button>
					<Button>"Save"</Button>
				</DialogFooter>
			</DialogContent>
		</Dialog>
	}
	.into_any()
}

fn d_alert_dialog() -> AnyView {
	view! {
		<AlertDialog default_open=true>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
					<AlertDialogDescription>"This action cannot be undone."</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel>"Cancel"</AlertDialogCancel>
					<AlertDialogAction>"Continue"</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	}
	.into_any()
}

fn d_sheet() -> AnyView {
	view! {
		<Sheet default_open=true>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>"Filters"</SheetTitle>
					<SheetDescription>"Refine the fund list."</SheetDescription>
				</SheetHeader>
			</SheetContent>
		</Sheet>
	}
	.into_any()
}

fn d_drawer() -> AnyView {
	view! {
		<Drawer default_open=true>
			<DrawerContent>
				<DrawerHeader>
					<DrawerTitle>"Direction"</DrawerTitle>
					<DrawerDescription>"Slide-up drawer."</DrawerDescription>
				</DrawerHeader>
				<DrawerFooter>
					<Button>"Submit"</Button>
				</DrawerFooter>
			</DrawerContent>
		</Drawer>
	}
	.into_any()
}

fn d_navigation_menu() -> AnyView {
	view! {
		<NavigationMenu>
			<NavigationMenuList>
				<NavigationMenuItem default_open=true>
					<NavigationMenuTrigger>"Funds"</NavigationMenuTrigger>
					<NavigationMenuContent>
						<NavigationMenuLink>"Coastal"</NavigationMenuLink>
						<NavigationMenuLink>"Urban"</NavigationMenuLink>
					</NavigationMenuContent>
				</NavigationMenuItem>
			</NavigationMenuList>
		</NavigationMenu>
	}
	.into_any()
}

// ── Tier B — interactive ───────────────────────────────────────────────────

fn d_checkbox() -> AnyView {
	view! {
		<div class="flex items-center gap-2">
			<Checkbox default_checked=true />
			<Label>"Accredited investor"</Label>
		</div>
	}
	.into_any()
}

fn d_switch() -> AnyView {
	view! {
		<div class="flex items-center gap-2">
			<Switch default_checked=true />
			<Label>"Email alerts"</Label>
		</div>
	}
	.into_any()
}

fn d_radio_group() -> AnyView {
	view! {
		<RadioGroup default_value="monthly">
			<div class="flex items-center gap-2">
				<RadioGroupItem value="monthly" />
				<Label>"Monthly"</Label>
			</div>
			<div class="flex items-center gap-2">
				<RadioGroupItem value="annual" />
				<Label>"Annual"</Label>
			</div>
		</RadioGroup>
	}
	.into_any()
}

fn d_slider() -> AnyView {
	view! {
		<div class="w-64">
			<Slider default_value=40.0 />
		</div>
	}
	.into_any()
}

fn d_toggle() -> AnyView {
	view! {
		<Toggle>"Bold"</Toggle>
		<Toggle variant=ToggleVariant::Outline default_pressed=true>
			"Italic"
		</Toggle>
		<Toggle size=Size::Sm>"Sm"</Toggle>
		<Toggle size=Size::Lg>"Lg"</Toggle>
	}
	.into_any()
}

fn d_toggle_group() -> AnyView {
	view! {
		<ToggleGroup variant=ToggleVariant::Outline default_value=vec!["left".to_string()]>
			<ToggleGroupItem value="left">"Left"</ToggleGroupItem>
			<ToggleGroupItem value="center">"Center"</ToggleGroupItem>
			<ToggleGroupItem value="right">"Right"</ToggleGroupItem>
		</ToggleGroup>
	}
	.into_any()
}

fn d_tabs() -> AnyView {
	view! {
		<Tabs default_value="overview" class="w-72">
			<TabsList>
				<TabsTrigger value="overview">"Overview"</TabsTrigger>
				<TabsTrigger value="returns">"Returns"</TabsTrigger>
			</TabsList>
			<TabsContent value="overview">"Overview panel"</TabsContent>
			<TabsContent value="returns">"Returns panel"</TabsContent>
		</Tabs>
	}
	.into_any()
}

fn d_accordion() -> AnyView {
	view! {
		<Accordion class="w-72" collapsible=true default_value=vec!["a".to_string()]>
			<AccordionItem value="a">
				<AccordionTrigger>"What is the thesis?"</AccordionTrigger>
				<AccordionContent>"Coastal development in emerging Vietnam."</AccordionContent>
			</AccordionItem>
			<AccordionItem value="b">
				<AccordionTrigger>"What are the risks?"</AccordionTrigger>
				<AccordionContent>"Currency, liquidity, and regulatory."</AccordionContent>
			</AccordionItem>
		</Accordion>
	}
	.into_any()
}

fn d_collapsible() -> AnyView {
	view! {
		<Collapsible class="w-64" default_open=true>
			<CollapsibleTrigger>"Toggle details"</CollapsibleTrigger>
			<CollapsibleContent>"Hidden details revealed here."</CollapsibleContent>
		</Collapsible>
	}
	.into_any()
}

fn d_input_otp() -> AnyView {
	view! {
		<InputOTP max_length=6>
			<InputOTPGroup>
				<InputOTPSlot index=0 />
				<InputOTPSlot index=1 />
				<InputOTPSlot index=2 />
			</InputOTPGroup>
			<InputOTPSeparator />
			<InputOTPGroup>
				<InputOTPSlot index=3 />
				<InputOTPSlot index=4 />
				<InputOTPSlot index=5 />
			</InputOTPGroup>
		</InputOTP>
	}
	.into_any()
}

fn d_carousel() -> AnyView {
	view! {
		<Carousel class="w-64">
			<CarouselContent>
				<CarouselItem>
					<div class="flex h-32 items-center justify-center rounded-md bg-muted">"Slide 1"</div>
				</CarouselItem>
				<CarouselItem>
					<div class="flex h-32 items-center justify-center rounded-md bg-muted">"Slide 2"</div>
				</CarouselItem>
			</CarouselContent>
			<CarouselPrevious />
			<CarouselNext />
		</Carousel>
	}
	.into_any()
}

fn d_sidebar() -> AnyView {
	view! {
		<div class="h-72 w-full overflow-hidden rounded-lg border border-border">
			<SidebarProvider>
				<Sidebar>
					<SidebarHeader>"EV Invest"</SidebarHeader>
					<SidebarContent>
						<SidebarGroup>
							<SidebarGroupLabel>"Platform"</SidebarGroupLabel>
							<SidebarGroupContent>
								<SidebarMenu>
									<SidebarMenuItem>
										<SidebarMenuButton is_active=true>"Dashboard"</SidebarMenuButton>
									</SidebarMenuItem>
									<SidebarMenuItem>
										<SidebarMenuButton>"Portfolio"</SidebarMenuButton>
									</SidebarMenuItem>
								</SidebarMenu>
							</SidebarGroupContent>
						</SidebarGroup>
					</SidebarContent>
				</Sidebar>
			</SidebarProvider>
		</div>
	}
	.into_any()
}

fn p_carousel_edge_fade() -> AnyView {
	view! {
		<Carousel class="w-64">
			<CarouselContent>
				<CarouselItem>"Slide 1"</CarouselItem>
				<CarouselItem>"Slide 2"</CarouselItem>
			</CarouselContent>
			<CarouselEdgeFade />
		</Carousel>
	}
	.into_any()
}

fn p_collapsible_siblings() -> AnyView {
	view! {
		<Collapsible default_open=true>
			<CollapsibleTrigger>"Show"</CollapsibleTrigger>
			<CollapsibleContent>
				<CollapsibleTrigger>"Hide"</CollapsibleTrigger>
			</CollapsibleContent>
		</Collapsible>
		<Collapsible>
			<CollapsibleTrigger>"Show"</CollapsibleTrigger>
			<CollapsibleContent>"B"</CollapsibleContent>
		</Collapsible>
	}
	.into_any()
}

fn p_accordion_siblings() -> AnyView {
	view! {
		<Accordion default_value=vec!["a".to_string()]>
			<AccordionItem value="a">
				<AccordionTrigger>"A"</AccordionTrigger>
				<AccordionContent>"Body A"</AccordionContent>
			</AccordionItem>
		</Accordion>
		<Accordion default_value=vec!["b".to_string()]>
			<AccordionItem value="a">
				<AccordionTrigger>"A"</AccordionTrigger>
				<AccordionContent>"Body A"</AccordionContent>
			</AccordionItem>
		</Accordion>
	}
	.into_any()
}

fn p_tabs_siblings() -> AnyView {
	view! {
		<Tabs default_value="one">
			<TabsList>
				<TabsTrigger value="one">"One"</TabsTrigger>
				<TabsTrigger value="two">"Two"</TabsTrigger>
			</TabsList>
			<TabsContent value="one">"Panel one"</TabsContent>
		</Tabs>
		<Tabs default_value="two" orientation=TabsOrientation::Vertical>
			<TabsList>
				<TabsTrigger value="one">"One"</TabsTrigger>
				<TabsTrigger value="two">"Two"</TabsTrigger>
			</TabsList>
			<TabsContent value="two">"Panel two"</TabsContent>
		</Tabs>
	}
	.into_any()
}

fn p_radio_group_siblings() -> AnyView {
	view! {
		<RadioGroup default_value="a">
			<RadioGroupItem value="a" />
			<RadioGroupItem value="b" />
		</RadioGroup>
		<RadioGroup default_value="b">
			<RadioGroupItem value="a" />
			<RadioGroupItem value="b" />
		</RadioGroup>
	}
	.into_any()
}

fn p_toggle_group_siblings() -> AnyView {
	view! {
		<ToggleGroup variant=ToggleVariant::Outline default_value=vec!["x".to_string()]>
			<ToggleGroupItem value="x">"X"</ToggleGroupItem>
		</ToggleGroup>
		<ToggleGroupItem value="x">"Outside"</ToggleGroupItem>
	}
	.into_any()
}

fn p_carousel_siblings() -> AnyView {
	view! {
		<Carousel>
			<CarouselContent>
				<CarouselItem>"1"</CarouselItem>
				<CarouselItem>"2"</CarouselItem>
			</CarouselContent>
			<CarouselNext />
		</Carousel>
		<Carousel orientation=CarouselOrientation::Vertical>
			<CarouselContent>
				<CarouselItem>"1"</CarouselItem>
			</CarouselContent>
			<CarouselNext />
		</Carousel>
	}
	.into_any()
}

fn p_input_otp_siblings() -> AnyView {
	view! {
		<InputOTP max_length=2 value="1">
			<InputOTPSlot index=0 />
			<InputOTPSlot index=1 />
		</InputOTP>
		<InputOTP max_length=2>
			<InputOTPSlot index=0 />
			<InputOTPSlot index=1 />
		</InputOTP>
	}
	.into_any()
}

fn p_sidebar_siblings() -> AnyView {
	view! {
		<SidebarProvider>
			<Sidebar>"A"</Sidebar>
		</SidebarProvider>
		<SidebarProvider default_open=false>
			<Sidebar collapsible=SidebarCollapsible::Icon>"B"</Sidebar>
		</SidebarProvider>
	}
	.into_any()
}

fn p_sidebar_parts() -> AnyView {
	view! {
		<SidebarProvider>
			<Sidebar>
				<SidebarHeader>"EV Invest"</SidebarHeader>
				<SidebarContent>
					<SidebarGroup>
						<SidebarGroupContent>
							<SidebarMenu>
								<SidebarMenuItem>
									<SidebarMenuButton is_active=true>"Dashboard"</SidebarMenuButton>
								</SidebarMenuItem>
							</SidebarMenu>
						</SidebarGroupContent>
					</SidebarGroup>
				</SidebarContent>
				<SidebarSeparator />
				<SidebarFooter>"v1.0"</SidebarFooter>
				<SidebarRail />
			</Sidebar>
			<SidebarInset>
				<SidebarTrigger>"Toggle"</SidebarTrigger>
				"Main content"
			</SidebarInset>
		</SidebarProvider>
	}
	.into_any()
}

// ── Tier D — engines / layout ──────────────────────────────────────────────

fn d_scroll_area() -> AnyView {
	view! {
		<ScrollArea class="h-32 w-56 rounded-md border border-border p-4">
			<div class="flex flex-col gap-2">
				{(1..=12).map(|i| view! { <div>"Row " {i}</div> }).collect_view()}
			</div>
		</ScrollArea>
	}
	.into_any()
}

fn d_container() -> AnyView {
	view! {
		<Container class="border border-dashed border-border py-4">
			"Centered max-width container"
		</Container>
	}
	.into_any()
}

fn d_terminal() -> AnyView {
	view! {
		<Terminal>
			<TerminalTicker>
				<TickerStat label="Ha Long" value="1.024" />
				<TickerStat label="24h" value="+2.4%" class="text-positive" />
				<TickerStat label="Volume" value="12 400" />
			</TerminalTicker>
			<TerminalPane area=TerminalArea::Chart>
				<TerminalPaneHeader>"Price"</TerminalPaneHeader>
				<TerminalPaneBody>
					<TerminalChart />
				</TerminalPaneBody>
			</TerminalPane>
			<TerminalPane area=TerminalArea::Book>
				<TerminalPaneHeader>"Order book"</TerminalPaneHeader>
				<TerminalPaneBody>
					<OrderBook>
						<OrderBookHead price="Price" size="Size" total="Total" />
						<OrderBookRow side=BookSide::Ask price="1.030" size="120" total="360" depth=0.9 />
						<OrderBookRow side=BookSide::Ask price="1.028" size="80" total="240" depth=0.6 />
						<OrderBookRow side=BookSide::Ask price="1.026" size="160" total="160" depth=0.4 />
						<OrderBookSpread>
							<span>"1.025"</span>
							<span>"0.002"</span>
						</OrderBookSpread>
						<OrderBookRow side=BookSide::Bid price="1.024" size="140" total="140" depth=0.35 />
						<OrderBookRow side=BookSide::Bid price="1.022" size="90" total="230" depth=0.55 />
						<OrderBookRow side=BookSide::Bid price="1.020" size="200" total="430" depth=1.0 />
					</OrderBook>
				</TerminalPaneBody>
			</TerminalPane>
			<TerminalPane area=TerminalArea::Form>
				<TerminalPaneHeader>"Buy / Sell"</TerminalPaneHeader>
				<TerminalPaneBody>
					<OrderForm>
						<Input placeholder="Price" />
						<Input placeholder="Size" />
						<OrderFormRow label="Cost" value="143.36" />
						<OrderFormRow label="Fee" value="0.14" />
						<OrderFormSubmit>"Buy"</OrderFormSubmit>
						<OrderFormSubmit side=OrderSide::Sell>"Sell"</OrderFormSubmit>
					</OrderForm>
				</TerminalPaneBody>
			</TerminalPane>
			<TerminalPane area=TerminalArea::Orders>
				<TerminalPaneHeader>"Open orders"</TerminalPaneHeader>
				<TerminalPaneBody>
					<OpenOrdersEmpty>"No open orders"</OpenOrdersEmpty>
				</TerminalPaneBody>
			</TerminalPane>
		</Terminal>
	}
	.into_any()
}

// ── Parity only ────────────────────────────────────────────────────────────

fn p_auth_wall() -> AnyView {
	view! {
		<AuthWall title="Sign in required" description="Log in to see your portfolio.">
			<Button>"Sign in"</Button>
		</AuthWall>
	}
	.into_any()
}

fn p_avatar_image() -> AnyView {
	view! {
		<Avatar>
			<AvatarImage src="/avatar.png" alt="Daneel Olivaw" />
			<AvatarFallback>"DO"</AvatarFallback>
		</Avatar>
	}
	.into_any()
}

fn p_band() -> AnyView {
	view! {
		<Section id="thesis">
			<SectionHead
				eyebrow="Thesis"
				title="Coastal development"
				lede="Emerging Vietnam, institutional grade."
			/>
			<Eyebrow>"Overview"</Eyebrow>
			<Display>"Quy Nhon Fund"</Display>
			<Prose>"Projected IRR 18.4% over a 5-year horizon."</Prose>
			<Stat figure="18.4%" label="Target IRR" />
			<Check />
		</Section>
	}
	.into_any()
}

fn p_breadcrumb_ellipsis() -> AnyView {
	view! {
		<Breadcrumb>
			<BreadcrumbList>
				<BreadcrumbItem>
					<BreadcrumbLink href="/">"Home"</BreadcrumbLink>
				</BreadcrumbItem>
				<BreadcrumbSeparator />
				<BreadcrumbItem>
					<BreadcrumbEllipsis />
				</BreadcrumbItem>
				<BreadcrumbSeparator />
				<BreadcrumbItem>
					<BreadcrumbPage>"Quy Nhon"</BreadcrumbPage>
				</BreadcrumbItem>
			</BreadcrumbList>
		</Breadcrumb>
	}
	.into_any()
}

fn p_button_group_parts() -> AnyView {
	view! {
		<ButtonGroup>
			<ButtonGroupText>"Range"</ButtonGroupText>
			<ButtonGroupSeparator />
			<Button variant=ButtonVariant::Outline>"Day"</Button>
			<Button variant=ButtonVariant::Outline>"Week"</Button>
		</ButtonGroup>
	}
	.into_any()
}

fn p_card_action() -> AnyView {
	view! {
		<Card class="w-72">
			<CardHeader>
				<CardTitle>"Quy Nhon Fund"</CardTitle>
				<CardDescription>"Coastal development thesis"</CardDescription>
				<CardAction>
					<Button size=Size::Sm>"Invest"</Button>
				</CardAction>
			</CardHeader>
			<CardContent>"Projected IRR 18.4% over a 5-year horizon."</CardContent>
		</Card>
	}
	.into_any()
}

fn p_fonts() -> AnyView {
	view! { <Fonts base="/assets" /> }.into_any()
}

fn p_footer() -> AnyView {
	let nav = vec![FooterLinkGroup {
		heading: "Company".to_string(),
		links: vec![FooterLink {
			label: "Team".to_string(),
			href: "/team".to_string(),
		}],
	}];
	view! { <Footer brand="ACME" description="We do a thing." copyright="© 2026 ACME" nav /> }.into_any()
}

fn p_item_parts() -> AnyView {
	view! {
		<ItemGroup class="w-80">
			<Item variant=ItemVariant::Outline>
				<ItemHeader>"Position"</ItemHeader>
				<ItemContent>
					<ItemTitle>"Ha Long"</ItemTitle>
					<ItemDescription>"Tier 1"</ItemDescription>
				</ItemContent>
				<ItemActions>
					<Button size=Size::Sm>"Open"</Button>
				</ItemActions>
				<ItemFooter>"Updated Monday"</ItemFooter>
			</Item>
			<ItemSeparator />
			<Item>
				<ItemContent>
					<ItemTitle>"Jungle"</ItemTitle>
				</ItemContent>
			</Item>
		</ItemGroup>
	}
	.into_any()
}

fn p_logo() -> AnyView {
	view! { <Logo class="size-8" /> }.into_any()
}

fn p_scroll_bar() -> AnyView {
	view! {
		<ScrollArea class="h-32 w-56 rounded-md border border-border p-4">
			<div>"Row 1"</div>
			<div>"Row 2"</div>
			<ScrollBar />
			<ScrollBar orientation=ScrollBarOrientation::Horizontal />
		</ScrollArea>
	}
	.into_any()
}

fn p_section_descriptor() -> AnyView {
	view! {
		<SectionDescriptor title="Risks">"Currency, liquidity, and regulatory."</SectionDescriptor>
		<SectionDescriptor title="Thesis" collapsible=true default_open=true>
			"Coastal development in emerging Vietnam."
		</SectionDescriptor>
	}
	.into_any()
}

fn p_status_screen() -> AnyView {
	view! {
		<StatusScreen
			accent=Accent::Debug
			eyebrow="Teapot"
			code="418"
			headline_lead="Short and "
			headline_accent="stout"
			subtext="The server refuses to brew coffee."
			links=vec![
				StatusLinkData {
					label: "Back to home".to_string(),
					href: "/".to_string(),
					variant: ButtonVariant::Primary,
					leading_arrow: true,
				},
			]
		/>
		<NotFound />
		<Forbidden />
		<ServerError />
	}
	.into_any()
}

fn p_table_parts() -> AnyView {
	view! {
		<Table class="w-full max-w-md">
			<TableCaption>"Yields by asset"</TableCaption>
			<TableHeader>
				<TableRow>
					<TableHead>"Asset"</TableHead>
					<TableHead>"Yield"</TableHead>
				</TableRow>
			</TableHeader>
			<TableBody>
				<TableRow>
					<TableCell>"Ha Long"</TableCell>
					<TableCell>"12.0%"</TableCell>
				</TableRow>
			</TableBody>
			<TableFooter>
				<TableRow>
					<TableCell>"Total"</TableCell>
					<TableCell>"12.0%"</TableCell>
				</TableRow>
			</TableFooter>
		</Table>
	}
	.into_any()
}

fn p_trades_tape_row() -> AnyView {
	view! {
		<div class="flex w-72 flex-col">
			<TradesTapeRow side=BookSide::Bid price="1.024" size="140" time="12:00:01" />
			<TradesTapeRow side=BookSide::Ask price="1.026" size="90" time="12:00:02" />
		</div>
	}
	.into_any()
}

fn p_alert_dialog_trigger() -> AnyView {
	view! {
		<AlertDialog default_open=true>
			<AlertDialogTrigger>"Delete account"</AlertDialogTrigger>
			<AlertDialogContent>
				<AlertDialogHeader>
					<AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>
					<AlertDialogDescription>"This action cannot be undone."</AlertDialogDescription>
				</AlertDialogHeader>
				<AlertDialogFooter>
					<AlertDialogCancel>"Cancel"</AlertDialogCancel>
					<AlertDialogAction>"Continue"</AlertDialogAction>
				</AlertDialogFooter>
			</AlertDialogContent>
		</AlertDialog>
	}
	.into_any()
}

fn p_context_menu_parts() -> AnyView {
	view! {
		<ContextMenu default_open=true>
			<ContextMenuTrigger>"Right-click area"</ContextMenuTrigger>
			<ContextMenuContent>
				<ContextMenuGroup>
					<ContextMenuLabel>"View"</ContextMenuLabel>
					<ContextMenuCheckboxItem checked=true>"Show grid"</ContextMenuCheckboxItem>
				</ContextMenuGroup>
				<ContextMenuSeparator />
				<ContextMenuRadioGroup default_value="day">
					<ContextMenuRadioItem value="day">"Day"</ContextMenuRadioItem>
					<ContextMenuRadioItem value="week">"Week"</ContextMenuRadioItem>
				</ContextMenuRadioGroup>
				<ContextMenuItem>"Reload"<ContextMenuShortcut>"⌘R"</ContextMenuShortcut></ContextMenuItem>
				<ContextMenuSub default_open=true>
					<ContextMenuSubTrigger>"More"</ContextMenuSubTrigger>
					<ContextMenuSubContent>
						<ContextMenuItem>"Export"</ContextMenuItem>
					</ContextMenuSubContent>
				</ContextMenuSub>
			</ContextMenuContent>
		</ContextMenu>
	}
	.into_any()
}

fn p_dialog_parts() -> AnyView {
	view! {
		<Dialog default_open=true>
			<DialogTrigger>"Edit profile"</DialogTrigger>
			<DialogPortal>
				<DialogContent>
					<DialogHeader>
						<DialogTitle>"Edit profile"</DialogTitle>
						<DialogDescription>"Update your account details."</DialogDescription>
					</DialogHeader>
					<DialogFooter>
						<DialogClose>"Cancel"</DialogClose>
					</DialogFooter>
				</DialogContent>
			</DialogPortal>
		</Dialog>
	}
	.into_any()
}

fn p_drawer_parts() -> AnyView {
	view! {
		<Drawer default_open=true>
			<DrawerTrigger>"Open drawer"</DrawerTrigger>
			<DrawerOverlay />
			<DrawerContent>
				<DrawerHeader>
					<DrawerTitle>"Direction"</DrawerTitle>
					<DrawerDescription>"Slide-up drawer."</DrawerDescription>
				</DrawerHeader>
				<DrawerFooter>
					<DrawerClose>"Close"</DrawerClose>
				</DrawerFooter>
			</DrawerContent>
		</Drawer>
	}
	.into_any()
}

fn p_dropdown_menu_parts() -> AnyView {
	view! {
		<DropdownMenu default_open=true>
			<DropdownMenuTrigger>"Menu"</DropdownMenuTrigger>
			<DropdownMenuContent>
				<DropdownMenuGroup>
					<DropdownMenuLabel>"View"</DropdownMenuLabel>
					<DropdownMenuCheckboxItem checked=true>"Show grid"</DropdownMenuCheckboxItem>
				</DropdownMenuGroup>
				<DropdownMenuSeparator />
				<DropdownMenuRadioGroup default_value="day">
					<DropdownMenuRadioItem value="day">"Day"</DropdownMenuRadioItem>
					<DropdownMenuRadioItem value="week">"Week"</DropdownMenuRadioItem>
				</DropdownMenuRadioGroup>
				<DropdownMenuItem>
					"Reload"<DropdownMenuShortcut>"⌘R"</DropdownMenuShortcut>
				</DropdownMenuItem>
				<DropdownMenuSub default_open=true>
					<DropdownMenuSubTrigger>"More"</DropdownMenuSubTrigger>
					<DropdownMenuSubContent>
						<DropdownMenuItem>"Export"</DropdownMenuItem>
					</DropdownMenuSubContent>
				</DropdownMenuSub>
			</DropdownMenuContent>
		</DropdownMenu>
	}
	.into_any()
}

fn p_info_tip() -> AnyView {
	view! {
		<InfoTip default_open=true>
			<InfoTipTrigger />
			<InfoTipContent>"Net of fees."</InfoTipContent>
		</InfoTip>
	}
	.into_any()
}

fn p_menubar_parts() -> AnyView {
	view! {
		<Menubar>
			<MenubarMenu default_open=true>
				<MenubarTrigger>"View"</MenubarTrigger>
				<MenubarContent>
					<MenubarGroup>
						<MenubarLabel>"Panels"</MenubarLabel>
						<MenubarCheckboxItem checked=true>"Show grid"</MenubarCheckboxItem>
					</MenubarGroup>
					<MenubarSeparator />
					<MenubarRadioGroup default_value="day">
						<MenubarRadioItem value="day">"Day"</MenubarRadioItem>
						<MenubarRadioItem value="week">"Week"</MenubarRadioItem>
					</MenubarRadioGroup>
					<MenubarItem>"Reload"<MenubarShortcut>"⌘R"</MenubarShortcut></MenubarItem>
					<MenubarSub default_open=true>
						<MenubarSubTrigger>"More"</MenubarSubTrigger>
						<MenubarSubContent>
							<MenubarItem>"Export"</MenubarItem>
						</MenubarSubContent>
					</MenubarSub>
				</MenubarContent>
			</MenubarMenu>
		</Menubar>
	}
	.into_any()
}

fn p_navigation_menu_parts() -> AnyView {
	view! {
		<NavigationMenu>
			<NavigationMenuList>
				<NavigationMenuItem default_open=true>
					<NavigationMenuTrigger>"Funds"</NavigationMenuTrigger>
					<NavigationMenuContent>
						<NavigationMenuLink>"Coastal"</NavigationMenuLink>
					</NavigationMenuContent>
				</NavigationMenuItem>
			</NavigationMenuList>
			<NavigationMenuIndicator />
			<NavigationMenuViewport>"Viewport"</NavigationMenuViewport>
		</NavigationMenu>
	}
	.into_any()
}

fn p_popover_anchor() -> AnyView {
	view! {
		<Popover default_open=true>
			<PopoverAnchor>
				<PopoverTrigger>
					<Button variant=ButtonVariant::Outline>"Open popover"</Button>
				</PopoverTrigger>
			</PopoverAnchor>
			<PopoverContent>"Popover body"</PopoverContent>
		</Popover>
	}
	.into_any()
}

fn p_select_parts() -> AnyView {
	view! {
		<Select default_open=true default_value="jungle" class="w-56">
			<SelectTrigger>
				<SelectValue placeholder="Pick a fund" />
			</SelectTrigger>
			<SelectContent>
				<SelectGroup>
					<SelectLabel>"Coastal"</SelectLabel>
					<SelectItem value="ha-long" text_value="Ha Long Teal">
						"Ha Long Teal"
					</SelectItem>
				</SelectGroup>
				<SelectSeparator />
				<SelectGroup>
					<SelectLabel>"Inland"</SelectLabel>
					<SelectItem value="jungle" text_value="Jungle Green">
						"Jungle Green"
					</SelectItem>
				</SelectGroup>
			</SelectContent>
		</Select>
	}
	.into_any()
}

fn p_sheet_parts() -> AnyView {
	view! {
		<Sheet default_open=true>
			<SheetTrigger>"Filters"</SheetTrigger>
			<SheetContent>
				<SheetHeader>
					<SheetTitle>"Filters"</SheetTitle>
					<SheetDescription>"Refine the fund list."</SheetDescription>
				</SheetHeader>
				<SheetFooter>
					<SheetClose>"Done"</SheetClose>
				</SheetFooter>
			</SheetContent>
		</Sheet>
	}
	.into_any()
}

// ── Field, input group, engines, form ──────────────────────────────────────

fn d_field() -> AnyView {
	view! {
		<FieldSet class="w-72">
			<FieldLegend>"Profile"</FieldLegend>
			<FieldGroup>
				<Field>
					<FieldLabel>"Name"</FieldLabel>
					<Input placeholder="Daneel Olivaw" />
					<FieldDescription>"Shown on your public profile."</FieldDescription>
				</Field>
			</FieldGroup>
		</FieldSet>
	}
	.into_any()
}

fn d_input_group() -> AnyView {
	view! {
		<InputGroup class="w-64">
			<InputGroupAddon>"$"</InputGroupAddon>
			<InputGroupInput placeholder="0.00" />
			<InputGroupAddon align=InputGroupAddonAlign::InlineEnd>"USD"</InputGroupAddon>
		</InputGroup>
	}
	.into_any()
}

fn d_calendar() -> AnyView {
	view! {
		<div class="flex flex-wrap items-start gap-3">
			<Calendar default_month=CalendarDate::new(2026, 6, 1) />
			<Calendar
				default_month=CalendarDate::new(2026, 6, 1)
				selected=CalendarDate::new(2026, 6, 15)
				today=CalendarDate::new(2026, 6, 10)
				min=CalendarDate::new(2026, 6, 3)
				max=CalendarDate::new(2026, 6, 25)
			/>
			<Calendar default_month=CalendarDate::new(2026, 6, 1) disabled=true />
		</div>
	}
	.into_any()
}

fn d_date_time_picker() -> AnyView {
	let value = LocalDateTime::new(CalendarDate::new(2026, 6, 15), 9, 30);
	view! {
		<div class="flex flex-col">
			<div class="flex items-start gap-3">
				<div class="w-56">
					<DateTimePicker placeholder="Pick a moment" />
				</div>
				<div class="w-56">
					<DateTimePicker value=value />
				</div>
				<div class="w-56">
					<DateTimePicker
						default_open=true
						value=value
						today=CalendarDate::new(2026, 6, 10)
						min=LocalDateTime::new(CalendarDate::new(2026, 6, 3), 0, 0)
						max=LocalDateTime::new(CalendarDate::new(2026, 6, 25), 23, 59)
					/>
				</div>
			</div>
			<div class="h-96"></div>
		</div>
	}
	.into_any()
}

fn d_form() -> AnyView {
	view! {
		<Form class="w-72">
			<FormItem>
				<FormLabel>"Username"</FormLabel>
				<FormControl>
					<Input placeholder="daneel" />
				</FormControl>
				<FormDescription>"This is your public display name."</FormDescription>
			</FormItem>
		</Form>
	}
	.into_any()
}

fn p_field_parts() -> AnyView {
	view! {
		<FieldSet class="w-72">
			<FieldGroup>
				<Field>
					<FieldContent>
						<FieldTitle>"Two-factor"</FieldTitle>
						<FieldDescription>"Protects your account."</FieldDescription>
					</FieldContent>
					<Switch default_checked=true />
				</Field>
				<FieldSeparator content=true>"or"</FieldSeparator>
				<Field>
					<FieldLabel>"Email"</FieldLabel>
					<Input placeholder="daneel@example.com" />
					<FieldError>"Enter a valid email."</FieldError>
				</Field>
			</FieldGroup>
		</FieldSet>
	}
	.into_any()
}

fn p_form_message() -> AnyView {
	view! {
		<Form class="w-72">
			<FormItem>
				<FormLabel error=true>"Username"</FormLabel>
				// Mounted lazily, after the sibling item below has run: it must still see its own item.
				{move || {
					view! {
						<FormControl error=true>
							<Input placeholder="daneel" />
						</FormControl>
					}
				}}
				<FormMessage error=true>"Username is taken."</FormMessage>
			</FormItem>
			// A sibling item: its ids must be its own.
			<FormItem>
				<FormLabel>"Email"</FormLabel>
				<FormControl>
					<Input placeholder="daneel@example.com" />
				</FormControl>
				<FormDescription>"We never share it."</FormDescription>
				<FormMessage>"Unused while valid."</FormMessage>
			</FormItem>
		</Form>
	}
	.into_any()
}

fn p_input_group_parts() -> AnyView {
	view! {
		<div class="flex w-72 flex-col gap-2">
			<InputGroup>
				<InputGroupText>"https://"</InputGroupText>
				<InputGroupInput placeholder="example.com" />
				<InputGroupAddon align=InputGroupAddonAlign::InlineEnd>
					<InputGroupButton>"Copy"</InputGroupButton>
				</InputGroupAddon>
			</InputGroup>
			<InputGroup>
				<InputGroupTextarea placeholder="Notes…" />
			</InputGroup>
		</div>
	}
	.into_any()
}

#[component]
fn QueueToast() -> impl IntoView {
	use_toaster().show("Order filled", ToastVariant::Positive, None);
}

fn p_toaster() -> AnyView {
	view! {
		<ToasterProvider>
			<QueueToast />
			// Mounted lazily, after the sibling provider below has run: it must still see its own queue.
			{move || view! { <Toaster /> }}
		</ToasterProvider>
		// A sibling provider keeps its own queue.
		<ToasterProvider>
			<QueueToast />
			<QueueToast />
			<Toaster position=ToastPosition::TopLeft />
		</ToasterProvider>
	}
	.into_any()
}

fn p_command_siblings() -> AnyView {
	[("cle", ["cleaning", "plumbing"]), ("lyo", ["Paris", "Lyon"])]
		.map(|(search, rows)| {
			view! {
				<Command default_search=search>
					<CommandInput />
					<CommandList>
						<CommandEmpty>"None."</CommandEmpty>
						<CommandGroup heading=search>
							{rows.map(|row| view! { <CommandItem value=row>{row}</CommandItem> }).collect_view()}
						</CommandGroup>
					</CommandList>
				</Command>
			}
		})
		.collect_view()
		.into_any()
}

fn p_command_parts() -> AnyView {
	view! {
		<CommandDialog default_open=true>
			<CommandInput placeholder="Type a command…" />
			<CommandList>
				<CommandGroup heading="Suggestions">
					<CommandItem value="calendar">"Calendar"</CommandItem>
					<CommandSeparator />
					<CommandItem value="search">
						"Search funds" <CommandShortcut>"⌘K"</CommandShortcut>
					</CommandItem>
				</CommandGroup>
			</CommandList>
		</CommandDialog>
	}
	.into_any()
}

fn p_select_siblings() -> AnyView {
	view! {
		<Select default_open=true default_value="a">
			<SelectTrigger>
				<SelectValue />
			</SelectTrigger>
			<SelectContent>
				<SelectItem value="a" text_value="A">
					"A"
				</SelectItem>
				<SelectItem value="b" text_value="B">
					"B"
				</SelectItem>
			</SelectContent>
		</Select>
		<Select default_value="b">
			<SelectTrigger>
				<SelectValue />
			</SelectTrigger>
			<SelectContent>
				<SelectItem value="a" text_value="A">
					"A"
				</SelectItem>
				<SelectItem value="b" text_value="B">
					"B"
				</SelectItem>
			</SelectContent>
		</Select>
	}
	.into_any()
}

fn p_dialog_siblings() -> AnyView {
	view! {
		<Dialog default_open=true>
			<DialogTrigger>"first"</DialogTrigger>
			<DialogContent>"first dialog"</DialogContent>
		</Dialog>
		<Dialog>
			<DialogTrigger>"second"</DialogTrigger>
			<DialogContent>"second dialog"</DialogContent>
		</Dialog>
		<AlertDialog default_open=true>
			<AlertDialogContent>"first alert"</AlertDialogContent>
		</AlertDialog>
		<AlertDialog>
			<AlertDialogContent>"second alert"</AlertDialogContent>
		</AlertDialog>
		<Sheet default_open=true>
			<SheetContent>"first sheet"</SheetContent>
		</Sheet>
		<Sheet>
			<SheetContent>"second sheet"</SheetContent>
		</Sheet>
		<Drawer default_open=true direction=DrawerDirection::Right>
			<DrawerContent>"first drawer"</DrawerContent>
		</Drawer>
		<Drawer>
			<DrawerContent>"second drawer"</DrawerContent>
		</Drawer>
	}
	.into_any()
}

fn p_popup_siblings() -> AnyView {
	view! {
		<Popover default_open=true>
			<PopoverTrigger>"first"</PopoverTrigger>
			<PopoverContent>"first popover"</PopoverContent>
		</Popover>
		<Popover>
			<PopoverTrigger>"second"</PopoverTrigger>
			<PopoverContent>"second popover"</PopoverContent>
		</Popover>
		<Tooltip default_open=true>
			<TooltipTrigger>"first"</TooltipTrigger>
			<TooltipContent>"first tooltip"</TooltipContent>
		</Tooltip>
		<Tooltip>
			<TooltipTrigger>"second"</TooltipTrigger>
			<TooltipContent>"second tooltip"</TooltipContent>
		</Tooltip>
		<HoverCard default_open=true>
			<HoverCardTrigger>"first"</HoverCardTrigger>
			<HoverCardContent>"first card"</HoverCardContent>
		</HoverCard>
		<HoverCard>
			<HoverCardTrigger>"second"</HoverCardTrigger>
			<HoverCardContent>"second card"</HoverCardContent>
		</HoverCard>
		<InfoTip default_open=true>
			<InfoTipTrigger />
			<InfoTipContent>"first tip"</InfoTipContent>
		</InfoTip>
		<InfoTip>
			<InfoTipTrigger />
			<InfoTipContent>"second tip"</InfoTipContent>
		</InfoTip>
	}
	.into_any()
}

fn p_menu_siblings() -> AnyView {
	view! {
		<DropdownMenu default_open=true>
			<DropdownMenuTrigger>"first"</DropdownMenuTrigger>
			<DropdownMenuContent>
				<DropdownMenuRadioGroup default_value="day">
					<DropdownMenuRadioItem value="day">"Day"</DropdownMenuRadioItem>
				</DropdownMenuRadioGroup>
				<DropdownMenuRadioGroup default_value="week">
					<DropdownMenuRadioItem value="day">"Day"</DropdownMenuRadioItem>
				</DropdownMenuRadioGroup>
				<DropdownMenuSub default_open=true>
					<DropdownMenuSubTrigger>"open sub"</DropdownMenuSubTrigger>
					<DropdownMenuSubContent>"first sub"</DropdownMenuSubContent>
				</DropdownMenuSub>
				<DropdownMenuSub>
					<DropdownMenuSubTrigger>"closed sub"</DropdownMenuSubTrigger>
					<DropdownMenuSubContent>"second sub"</DropdownMenuSubContent>
				</DropdownMenuSub>
			</DropdownMenuContent>
		</DropdownMenu>
		<DropdownMenu>
			<DropdownMenuTrigger>"second"</DropdownMenuTrigger>
			<DropdownMenuContent>"second menu"</DropdownMenuContent>
		</DropdownMenu>
		<ContextMenu default_open=true>
			<ContextMenuTrigger>"first"</ContextMenuTrigger>
			<ContextMenuContent>
				<ContextMenuRadioGroup default_value="day">
					<ContextMenuRadioItem value="day">"Day"</ContextMenuRadioItem>
				</ContextMenuRadioGroup>
				<ContextMenuRadioGroup default_value="week">
					<ContextMenuRadioItem value="day">"Day"</ContextMenuRadioItem>
				</ContextMenuRadioGroup>
				<ContextMenuSub default_open=true>
					<ContextMenuSubTrigger>"open sub"</ContextMenuSubTrigger>
					<ContextMenuSubContent>"first sub"</ContextMenuSubContent>
				</ContextMenuSub>
				<ContextMenuSub>
					<ContextMenuSubTrigger>"closed sub"</ContextMenuSubTrigger>
					<ContextMenuSubContent>"second sub"</ContextMenuSubContent>
				</ContextMenuSub>
			</ContextMenuContent>
		</ContextMenu>
		<ContextMenu>
			<ContextMenuTrigger>"second"</ContextMenuTrigger>
			<ContextMenuContent>"second menu"</ContextMenuContent>
		</ContextMenu>
		<Menubar>
			<MenubarMenu default_open=true>
				<MenubarTrigger>"File"</MenubarTrigger>
				<MenubarContent>
					<MenubarRadioGroup default_value="day">
						<MenubarRadioItem value="day">"Day"</MenubarRadioItem>
					</MenubarRadioGroup>
					<MenubarRadioGroup default_value="week">
						<MenubarRadioItem value="day">"Day"</MenubarRadioItem>
					</MenubarRadioGroup>
					<MenubarSub default_open=true>
						<MenubarSubTrigger>"open sub"</MenubarSubTrigger>
						<MenubarSubContent>"first sub"</MenubarSubContent>
					</MenubarSub>
					<MenubarSub>
						<MenubarSubTrigger>"closed sub"</MenubarSubTrigger>
						<MenubarSubContent>"second sub"</MenubarSubContent>
					</MenubarSub>
				</MenubarContent>
			</MenubarMenu>
			<MenubarMenu>
				<MenubarTrigger>"Edit"</MenubarTrigger>
				<MenubarContent>"second menu"</MenubarContent>
			</MenubarMenu>
		</Menubar>
		<NavigationMenu>
			<NavigationMenuList>
				<NavigationMenuItem default_open=true>
					<NavigationMenuTrigger>"Funds"</NavigationMenuTrigger>
					<NavigationMenuContent>"first panel"</NavigationMenuContent>
				</NavigationMenuItem>
				<NavigationMenuItem>
					<NavigationMenuTrigger>"About"</NavigationMenuTrigger>
					<NavigationMenuContent>"second panel"</NavigationMenuContent>
				</NavigationMenuItem>
			</NavigationMenuList>
		</NavigationMenu>
	}
	.into_any()
}
