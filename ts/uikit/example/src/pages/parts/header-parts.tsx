import type { ReactNode } from "react";
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
  CommandShortcut,
  IconButton,
  Kbd,
  SmartHeader,
  SystemBanner,
  ToggleGroup,
  ToggleGroupItem,
  WorkspaceTab,
  WorkspaceTabs,
  type SmartHeaderState,
} from "@evinvest/uikit";
import {
  BellIcon,
  CameraIcon,
  ChartIcon,
  ChevronDownIcon,
  EyeIcon,
  LayoutIcon,
  MessageIcon,
  PlusIcon,
  SearchIcon,
  SidebarIcon,
  WalletIcon,
} from "../../gallery/icons";

function Separator() {
  return <span aria-hidden className="bg-border h-4 w-px shrink-0" />;
}

function Stat({ label, value, tone }: { label: string; value: string; tone?: string }) {
  return (
    <div className="flex flex-col gap-px">
      <span className="text-caption text-ink-soft">{label}</span>
      <span className={`text-body-s font-medium tabular-nums ${tone ?? "text-ink"}`}>{value}</span>
    </div>
  );
}

/** Brand, layout, workspace tabs: the left of the bar in every state. */
export function HeaderStart() {
  return (
    <>
      <span className="bg-secondary border-border text-body-s text-ink grid size-6 place-items-center rounded-(--control-radius) border font-medium">T</span>
      <span className="text-body text-ink font-medium">Terminal</span>
      <span className="flex items-center">
        <IconButton aria-label="Layout" size="md">
          <LayoutIcon />
        </IconButton>
        <IconButton aria-label="Layout options" size="sm">
          <ChevronDownIcon />
        </IconButton>
      </span>
      <WorkspaceTabs aria-label="Workspaces" defaultValue="w1">
        <WorkspaceTab value="w1" onClose={() => {}} closeLabel="Close Workspace 1">
          Workspace 1
        </WorkspaceTab>
      </WorkspaceTabs>
      <IconButton aria-label="New workspace" size="sm">
        <PlusIcon />
      </IconButton>
    </>
  );
}

export function HeaderEnd({ extra }: { extra?: ReactNode }) {
  return (
    <>
      {extra}
      <IconButton aria-label="Messages">
        <MessageIcon />
      </IconButton>
      <IconButton aria-label="Screenshot">
        <CameraIcon />
      </IconButton>
      <IconButton aria-label="Wallet">
        <WalletIcon />
      </IconButton>
      <Separator />
      <Avatar shape="circle" size="md">
        <AvatarFallback>J</AvatarFallback>
      </Avatar>
    </>
  );
}

export function InstrumentToolbar() {
  return (
    <>
      <IconButton aria-label="Search">
        <SearchIcon />
      </IconButton>
      <div className="border-border bg-secondary flex flex-col rounded-(--control-radius) border px-2.5 py-0.5">
        <span className="text-caption text-ink-soft">EXCHANGE</span>
        <span className="text-body text-ink font-medium">ABCUSD</span>
      </div>
      <span className="flex items-center gap-0.5">
        <ToggleGroup type="single" aria-label="Timeframe" defaultValue="1m">
          <ToggleGroupItem value="1m">1m</ToggleGroupItem>
          <ToggleGroupItem value="1h">1h</ToggleGroupItem>
        </ToggleGroup>
        <IconButton aria-label="More timeframes" size="sm">
          <ChevronDownIcon />
        </IconButton>
      </span>
      <IconButton aria-label="Chart type">
        <ChartIcon />
      </IconButton>
      <Separator />
      <span className="flex items-center gap-2">
        <span className="font-display text-figure-l text-ink font-light tabular-nums">2398.73</span>
        <Badge variant="error">-2.97%</Badge>
      </span>
      <div className="flex gap-4 max-xl:hidden">
        <Stat label="24h volume" value="$8.96B" />
        <Stat label="Open interest" value="$5.52B" />
        <Stat label="Funding / countdown" value="+0.0010% · 02:52:34" tone="text-positive-ink" />
      </div>
      <span className="ml-auto flex items-center gap-1">
        <Button variant="ghost">Indicators  1/3</Button>
        <IconButton aria-label="Alerts">
          <BellIcon />
        </IconButton>
        <IconButton aria-label="Watch">
          <EyeIcon />
        </IconButton>
        <IconButton aria-label="Panels sidebar">
          <SidebarIcon />
        </IconButton>
        <Button variant="secondary">Panels</Button>
      </span>
    </>
  );
}

export function CollapsedInstrument() {
  return (
    <span className="flex items-center gap-3 px-3">
      <span className="text-body text-ink font-medium">ABCUSD</span>
      <span className="font-display text-figure-m text-ink font-light tabular-nums">2398.73</span>
      <Badge variant="error">-2.97%</Badge>
      <ToggleGroup type="single" aria-label="Timeframe" defaultValue="1m">
        <ToggleGroupItem value="1m">1m</ToggleGroupItem>
      </ToggleGroup>
    </span>
  );
}

function Ohlc({ label, value }: { label: string; value: string }) {
  return (
    <span className="flex gap-1">
      <span className="text-ink-soft">{label}</span>
      <span className="text-caption text-ink-mid tabular-nums">{value}</span>
    </span>
  );
}

export function ContextLine() {
  return (
    <>
      <Ohlc label="O" value="2400.17" />
      <Ohlc label="H" value="2402.25" />
      <Ohlc label="L" value="2399.77" />
      <Ohlc label="C" value="2402.24" />
      <span className="text-positive-ink tabular-nums">+2.07 (+0.09%)</span>
      <span>Vol 1.36K</span>
      <span>Order book heatmap · 1 indicator</span>
    </>
  );
}

const SUGGESTIONS = [
  { label: "New workspace", shortcut: "⌘N" },
  { label: "Search symbol", shortcut: "/" },
  { label: "Open order book", shortcut: "⌘O" },
] as const;

/** The palette as it drops from the bar: an inline panel, since the kit's CommandPalette is a dialog. */
export function InlinePalette() {
  return (
    <div className="px-2 pb-2 sm:pl-[420px]">
      <Command className="border-border bg-popover w-full max-w-[420px] rounded-lg border">
        <CommandInput placeholder="Type a command or search…" aria-label="Search commands" />
        <CommandList>
          <CommandGroup heading="Suggestions">
            {SUGGESTIONS.map((s) => (
              <CommandItem key={s.label} value={s.label}>
                {s.label}
                <CommandShortcut>{s.shortcut}</CommandShortcut>
              </CommandItem>
            ))}
          </CommandGroup>
          <CommandGroup heading="Settings">
            <CommandItem value="Profile">
              Profile<CommandShortcut>⌘,</CommandShortcut>
            </CommandItem>
            <CommandItem value="Billing">Billing</CommandItem>
          </CommandGroup>
        </CommandList>
        <div className="border-border-subtle text-caption text-ink-soft flex items-center gap-3 border-t px-2 py-1">
          <span className="flex items-center gap-1.5"><Kbd>↑↓</Kbd>navigate</span>
          <span className="flex items-center gap-1.5"><Kbd>↵</Kbd>select</span>
          <span className="flex items-center gap-1.5"><Kbd>esc</Kbd>close</span>
        </div>
      </Command>
    </div>
  );
}

export const HEADER_CAPTIONS: Record<SmartHeaderState, string> = {
  default: "Expanded · top bar, instrument bar, context line (page at scroll top)",
  collapsed: "Collapsed · on scroll the instrument and price merge into the top bar (one 40px row)",
  banner: "Alert · banner slot above the bar when the feed is degraded",
  search: "Search open · command palette drops from the bar, same menu family",
};

export function SmartHeaderDemo({ state }: { state: SmartHeaderState }) {
  return (
    <SmartHeader
      state={state}
      start={<HeaderStart />}
      end={<HeaderEnd />}
      collapsed={<CollapsedInstrument />}
      banner={
        <SystemBanner
          tone="warn"
          action={
            <Button size="sm" variant="link">
              Retry
            </Button>
          }
          onDismiss={() => {}}
        >
          Market data delayed — reconnecting to feed
        </SystemBanner>
      }
      toolbar={<InstrumentToolbar />}
      context={<ContextLine />}
      palette={<InlinePalette />}
    />
  );
}
