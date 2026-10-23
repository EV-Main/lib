import { useState } from "react";
import {
  Avatar,
  AvatarFallback,
  Badge,
  Button,
  Command,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuSeparator,
  ContextMenuShortcut,
  ContextMenuTrigger,
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
  DropdownMenuTrigger,
  Popover,
  PopoverContent,
  PopoverTrigger,
  Switch,
} from "@evinvest/uikit";
import {
  ActivityIcon,
  BellIcon,
  ChartIcon,
  CheckIcon,
  CopyIcon,
  CreditCardIcon,
  ExternalIcon,
  EyeIcon,
  GlobeIcon,
  HelpIcon,
  LayoutIcon,
  LinkIcon,
  LogOutIcon,
  PencilIcon,
  SlidersIcon,
  StarIcon,
  TrashIcon,
  TrendingUpIcon,
  UsersIcon,
} from "../../gallery/icons";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

function Profile() {
  const [dark, setDark] = useState(true);
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="secondary">Profile menu</Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-[300px]">
        <div className="flex items-center gap-2.5 p-2">
          <Avatar shape="circle" size="lg">
            <AvatarFallback>J</AvatarFallback>
          </Avatar>
          <div className="flex flex-col">
            <span className="text-body text-ink font-medium">JD</span>
            <span className="text-caption text-ink-soft">Free</span>
          </div>
        </div>
        <DropdownMenuSeparator />
        <DropdownMenuItem><SlidersIcon />Account settings</DropdownMenuItem>
        <DropdownMenuItem><CreditCardIcon />Billing</DropdownMenuItem>
        <DropdownMenuItem><LinkIcon />Integrations</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem><UsersIcon />Security</DropdownMenuItem>
        <DropdownMenuItem>
          <ChartIcon />Documents<Badge variant="neutral" className="ml-auto">New</Badge>
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem><HelpIcon />Support center</DropdownMenuItem>
        <DropdownMenuItem>
          <GlobeIcon />Language<Badge variant="neutral" className="ml-auto">EN</Badge>
        </DropdownMenuItem>
        <DropdownMenuCheckboxItem checked={dark} onClick={() => setDark((d) => !d)}>
          Dark theme
          <Switch checked={dark} aria-hidden tabIndex={-1} className="pointer-events-none ml-auto" />
        </DropdownMenuCheckboxItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem variant="danger"><LogOutIcon />Sign out</DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

function Action() {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="secondary">Action menu</Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-[220px]">
        <DropdownMenuItem><PencilIcon />Edit<DropdownMenuShortcut>⌘E</DropdownMenuShortcut></DropdownMenuItem>
        <DropdownMenuItem><CopyIcon />Duplicate<DropdownMenuShortcut>⌘D</DropdownMenuShortcut></DropdownMenuItem>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger><ExternalIcon />Share</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem>Copy link</DropdownMenuItem>
            <DropdownMenuItem>Email</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuItem disabled><StarIcon />Pin to top</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem variant="danger"><TrashIcon />Delete<DropdownMenuShortcut>⌫</DropdownMenuShortcut></DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

const FRAMES = ["1 minute", "5 minutes", "15 minutes", "1 hour", "4 hours", "1 day"];

function SelectMenu() {
  const [frame, setFrame] = useState("1 hour");
  return (
    <Popover>
      <PopoverTrigger asChild>
        <Button variant="secondary">Select menu · {frame}</Button>
      </PopoverTrigger>
      <PopoverContent className="w-[220px] p-1.5">
        <Command>
          <CommandInput placeholder="Search…" aria-label="Search timeframes" />
          <CommandList>
            <CommandGroup heading="Timeframe">
              {FRAMES.map((f) => (
                <CommandItem key={f} value={f} onSelect={() => setFrame(f)} className={f === frame ? "bg-active" : ""}>
                  {f}
                  {f === frame ? <CheckIcon className="ml-auto size-4" /> : null}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}

function Context() {
  return (
    <ContextMenu>
      <ContextMenuTrigger asChild>
        <div className="border-input text-body text-ink-soft flex h-24 w-64 items-center justify-center rounded-lg border border-dashed">Right-click the chart</div>
      </ContextMenuTrigger>
      <ContextMenuContent className="w-[260px]">
        <ContextMenuItem><BellIcon />Add alert at 2403.95<ContextMenuShortcut>Alt A</ContextMenuShortcut></ContextMenuItem>
        <ContextMenuItem><TrendingUpIcon />Buy limit @ 2403.95<ContextMenuShortcut>Alt B</ContextMenuShortcut></ContextMenuItem>
        <ContextMenuItem><ActivityIcon />Sell limit @ 2403.95<ContextMenuShortcut>Alt S</ContextMenuShortcut></ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem><PencilIcon />Draw horizontal line<ContextMenuShortcut>Alt H</ContextMenuShortcut></ContextMenuItem>
        <ContextMenuItem><EyeIcon />Hide all drawings</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem variant="danger"><TrashIcon />Remove all drawings</ContextMenuItem>
      </ContextMenuContent>
    </ContextMenu>
  );
}

const INDICATORS = ["Moving average", "Bollinger bands", "RSI", "VWAP"];

function Submenu() {
  const [on, setOn] = useState<readonly string[]>(["Moving average", "VWAP"]);
  const toggle = (name: string) => setOn((cur) => (cur.includes(name) ? cur.filter((n) => n !== name) : [...cur, name]));
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="secondary">Submenu</Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-[200px]">
        <DropdownMenuSub>
          <DropdownMenuSubTrigger><ChartIcon />Indicators</DropdownMenuSubTrigger>
          <DropdownMenuSubContent className="w-[200px]">
            {INDICATORS.map((name) => (
              <DropdownMenuCheckboxItem key={name} checked={on.includes(name)} onClick={() => toggle(name)}>
                {name}
              </DropdownMenuCheckboxItem>
            ))}
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger><PencilIcon />Drawings</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem>Trend line</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuSub>
          <DropdownMenuSubTrigger><LayoutIcon />Layouts</DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuItem>Default</DropdownMenuItem>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
        <DropdownMenuSeparator />
        <DropdownMenuItem><SlidersIcon />Settings</DropdownMenuItem>
        <DropdownMenuLabel className="text-caption text-ink-soft">{on.length} active</DropdownMenuLabel>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}

export default function MenuRecipesPage() {
  return (
    <PageRoot
      title="Menu recipes"
      lede="The five menu shapes from the design, built from the kit's menu parts: profile, action, select, context and submenu. Each opens in a layer and closes one layer at a time."
    >
      <Block title="Profile" note="A profile card, grouped rows, badges, a switch and a danger row.">
        <Specimen><Profile /></Specimen>
      </Block>
      <Block title="Action" note="Icons, shortcuts, a submenu row, a disabled row and a danger row.">
        <Specimen><Action /></Specimen>
      </Block>
      <Block title="Select" note="One choice: the selected row has the active fill and a check.">
        <Specimen><SelectMenu /></Specimen>
      </Block>
      <Block title="Context" note="Opens at the pointer, from the right button or the menu key.">
        <Specimen><Context /></Specimen>
      </Block>
      <Block title="Submenu" note="Opens on hover or ArrowRight. The checks keep the menu open, so several indicators can be on at once; Escape closes one level.">
        <Specimen><Submenu /></Specimen>
      </Block>
    </PageRoot>
  );
}
