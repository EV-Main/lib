import { useState, type ReactNode } from "react";
import {
  AsideHeader,
  AsidePanel,
  AsideRail,
  Button,
  ChatPanel,
  Field,
  FieldLabel,
  IconButton,
  Input,
  KeyValue,
  SectionNav,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Switch,
  SmartHeader,
  SystemBanner,
  Tabs,
  TabsList,
  TabsTrigger,
  type AsideRailItem,
} from "@evinvest/uikit";
import {
  BellIcon,
  BookmarkIcon,
  BotIcon,
  HistoryIcon,
  HomeIcon,
  LayersIcon,
  LinkIcon,
  FileTextIcon,
  MoreIcon,
  PlusIcon,
  SlidersIcon,
  StarIcon,
  SwapIcon,
} from "../../gallery/icons";
import { AssistantComposer, DockThread, useAssistant } from "./chat-parts";
import { HeaderEnd, HeaderStart } from "./header-parts";

export type AsideId = "list" | "alerts" | "chat";

const RAIL: readonly AsideRailItem[] = [
  { id: "list", label: "Watchlist", icon: <StarIcon /> },
  { id: "alerts", label: "Alerts", icon: <BellIcon /> },
  { id: "chat", label: "Assistant", icon: <BotIcon /> },
];
const RAIL_END: readonly AsideRailItem[] = [{ id: "settings", label: "Panel settings", icon: <SlidersIcon /> }];

/** The app body is the page, not the kit: a neutral panel stands in for the chart. */
export function ContentPlaceholder() {
  return (
    <div className="bg-background text-body-s text-ink-soft flex h-full min-h-48 items-center justify-center p-4 text-center">
      Page content (not part of the kit)
    </div>
  );
}

/** Header bar, a content area, then the asides and the rail, in the shell's order. */
export function AppFrame({ aside, left, children }: { aside?: ReactNode; left?: ReactNode; children?: ReactNode }) {
  return (
    <div className="border-border overflow-x-auto border">
      <div className="flex h-[560px] min-w-[720px] flex-col">
        <SmartHeader state="default" className="border-0" start={<HeaderStart />} end={<HeaderEnd />} />
        <div className="border-border flex min-h-0 flex-1 border-t">
          {left}
          <div className="min-w-0 flex-1">{children ?? <ContentPlaceholder />}</div>
          {aside}
        </div>
      </div>
    </div>
  );
}

function Row({ icon, label, value, selected }: { icon: ReactNode; label: string; value?: string; selected?: boolean }) {
  return (
    <Button variant="ghost" className={`h-7 w-full justify-start gap-2 px-2 font-normal ${selected ? "bg-active text-ink" : ""}`}>
      {icon}
      <span className="flex-1 text-left">{label}</span>
      {value ? <span className="text-caption text-ink-soft tabular-nums">{value}</span> : null}
    </Button>
  );
}

function Label({ children }: { children: string }) {
  return <span className="text-caption text-ink-soft px-2 pt-2">{children}</span>;
}

export function WatchlistAside({ onClose }: { onClose?: () => void }) {
  return (
    <AsidePanel
      aria-label="Watchlist"
      header={
        <AsideHeader
          title="Watchlist"
          {...(onClose ? { onClose } : {})}
          actions={
            <>
              <IconButton aria-label="Pin" size="sm"><BookmarkIcon /></IconButton>
              <IconButton aria-label="More" size="sm"><MoreIcon /></IconButton>
            </>
          }
        />
      }
    >
      <Tabs defaultValue="lists">
        <TabsList className="border-border h-8 w-full justify-start rounded-none border-b bg-transparent px-3">
          <TabsTrigger value="lists">Lists</TabsTrigger>
          <TabsTrigger value="alerts">Alerts</TabsTrigger>
        </TabsList>
      </Tabs>
      <div className="flex flex-col gap-1 px-2 py-3">
        <Input aria-label="Search instruments" placeholder="Search instruments" className="mb-2" />
        <Label>Pinned</Label>
        <Row icon={<StarIcon />} label="ABCUSD" value="2398.73" selected />
        <Row icon={<StarIcon />} label="DEFUSD" value="51.20" />
        <Row icon={<StarIcon />} label="GHIUSD" value="0.0412" />
        <Label>Recent</Label>
        <Row icon={<HistoryIcon />} label="JKLUSD" value="1.0840" />
        <Row icon={<HistoryIcon />} label="MNOUSD" value="312.50" />
        <Row icon={<HistoryIcon />} label="PQRUSD" value="88.14" />
        <Label>Lists</Label>
        <Row icon={<BookmarkIcon />} label="Majors" />
        <Row icon={<BookmarkIcon />} label="Volatility movers" />
        <Row icon={<PlusIcon />} label="New list" />
      </div>
    </AsidePanel>
  );
}

export function EditAlertAside({ onClose }: { onClose?: () => void }) {
  return (
    <AsidePanel
      aria-label="Edit alert"
      header={<AsideHeader title="Edit alert" onBack={() => {}} {...(onClose ? { onClose } : {})} />}
      footer={
        <>
          <Button variant="ghost">Cancel</Button>
          <Button>Save alert</Button>
        </>
      }
    >
      <div className="flex flex-col gap-4 p-4">
        <SystemBanner icon={<BellIcon />}>Alerts trigger on the last traded price.</SystemBanner>
        <Field>
          <FieldLabel>Instrument</FieldLabel>
          <Input defaultValue="ABCUSD" />
        </Field>
        <Field>
          <FieldLabel>Condition</FieldLabel>
          <Select defaultValue="above">
            <SelectTrigger className="w-full">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="above">Price crosses above</SelectItem>
              <SelectItem value="below">Price crosses below</SelectItem>
            </SelectContent>
          </Select>
        </Field>
        <Field>
          <FieldLabel>Value</FieldLabel>
          <Input defaultValue="2410.00" />
        </Field>
        <label className="text-body text-ink flex items-center gap-2">
          <span className="flex-1">Notify by email</span>
          <Switch defaultChecked aria-label="Notify by email" />
        </label>
        <label className="text-body text-ink flex items-center gap-2">
          <span className="flex-1">Repeat every time</span>
          <Switch aria-label="Repeat every time" />
        </label>
      </div>
    </AsidePanel>
  );
}

export function ChatAside({ onClose }: { onClose?: () => void }) {
  const a = useAssistant();
  return (
    <AsidePanel
      aria-label="Assistant"
      header={
        <AsideHeader
          title="Assistant"
          {...(onClose ? { onClose } : {})}
          actions={<IconButton aria-label="History" size="sm"><HistoryIcon /></IconButton>}
        />
      }
    >
      <ChatPanel
        className="h-full"
        composer={
          <div className="px-4 pb-4">
            <AssistantComposer streaming={a.streaming} onSend={a.send} onStop={a.stop} placeholder="Ask the assistant…" />
          </div>
        }
      >
        <DockThread streaming={a.streaming} shown={a.shown} />
      </ChatPanel>
    </AsidePanel>
  );
}

export function NavigationAside({ onClose }: { onClose?: () => void }) {
  return (
    <AsidePanel
      aria-label="Workspace"
      side="left"
      className="w-[260px]"
      header={<AsideHeader title="Workspace" {...(onClose ? { onClose } : {})} />}
    >
      <SectionNav
        className="w-full p-2"
        value="overview"
        groups={[
          {
            id: "general",
            label: "General",
            items: [
              { id: "overview", label: "Overview", icon: HomeIcon },
              { id: "positions", label: "Positions", icon: LayersIcon },
              { id: "orders", label: "Orders", icon: SwapIcon },
              { id: "reports", label: "Reports", icon: FileTextIcon },
            ],
          },
          {
            id: "tools",
            label: "Tools",
            items: [
              { id: "alerts", label: "Alerts", icon: BellIcon },
              { id: "automations", label: "Automations", icon: SlidersIcon },
              { id: "integrations", label: "Integrations", icon: LinkIcon },
            ],
          },
        ]}
        onValueChange={() => {}}
      />
    </AsidePanel>
  );
}

export function DeleteAlertOverlay({ open, onOpenChange }: { open: boolean; onOpenChange: (open: boolean) => void }) {
  return (
    <AsidePanel
      aria-label="Delete alert"
      mode="overlay"
      open={open}
      onOpenChange={onOpenChange}
      header={<AsideHeader title="Delete alert" onClose={() => onOpenChange(false)} />}
      footer={
        <>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>Cancel</Button>
          <Button variant="danger" onClick={() => onOpenChange(false)}>Delete alert</Button>
        </>
      }
    >
      <div className="flex flex-col gap-3 p-4">
        <p className="text-body text-ink-mid">This alert stops firing immediately. Notifications already sent stay in your history.</p>
        <KeyValue label="Instrument" value="ABCUSD" />
        <KeyValue label="Condition" value="Price crosses above 2410.00" />
        <KeyValue label="Created" value="Oct 18, 11:20" />
      </div>
    </AsidePanel>
  );
}

/** A rail that opens and closes the three right-docked asides; each one slides in. */
export function RailDemo({ initial = "list" }: { initial?: AsideId | null }) {
  const [open, setOpen] = useState<string | null>(initial);
  const close = () => setOpen(null);
  return (
    <AppFrame
      aside={
        <>
          {open === "list" ? <WatchlistAside onClose={close} /> : null}
          {open === "alerts" ? <EditAlertAside onClose={close} /> : null}
          {open === "chat" ? <ChatAside onClose={close} /> : null}
          <AsideRail items={RAIL} endItems={RAIL_END} value={open} onValueChange={setOpen} />
        </>
      }
    />
  );
}

/** A rail with nothing wired to it, for a stage that shows one fixed state. */
export function StaticRail({ value }: { value: AsideId | null }) {
  return <AsideRail items={RAIL} endItems={RAIL_END} value={value} onValueChange={() => {}} />;
}
