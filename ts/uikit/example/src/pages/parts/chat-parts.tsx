import { useState, type ReactNode } from "react";
import {
  Avatar,
  AvatarFallback,
  Badge,
  Button,
  ChatAttachment,
  ChatBubble,
  ChatComposer,
  ChatDock,
  ChatMarker,
  ChatMessage,
  ChatPanel,
  IconButton,
  Input,
  ModelChip,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  type ModelOption,
} from "@evinvest/uikit";
import { HistoryIcon, MoreIcon, SearchIcon, UploadIcon } from "../../gallery/icons";
import { useTokenStream } from "../../gallery/useTokenStream";

const SPEED: readonly ModelOption[] = [
  { id: "fast", label: "Fast" },
  { id: "balanced", label: "Balanced" },
];
const DEPTH: readonly ModelOption[] = [
  { id: "reasoning", label: "Reasoning" },
  { id: "off", label: "No reasoning" },
];

export function ComposerTools() {
  const [speed, setSpeed] = useState("fast");
  const [depth, setDepth] = useState("reasoning");
  return (
    <>
      <ModelChip models={SPEED} value={speed} onValueChange={setSpeed} aria-label="Speed" />
      <ModelChip models={DEPTH} value={depth} onValueChange={setDepth} aria-label="Reasoning" />
    </>
  );
}

const REPLY =
  "Both positions are above the limit because ABCUSD rallied faster than the rest of the book. Trimming ABCUSD by 2.1 units and DEFUSD by 0.6 units brings both under 25%, and leaves net exposure near 36% of equity.";

/** The composer, wired to a simulated token stream: send starts it, Stop ends it. */
export function useAssistant() {
  const stream = useTokenStream(REPLY);
  const [asked, setAsked] = useState<string | null>(null);
  const send = (text: string) => {
    setAsked(text);
    stream.start();
  };
  return { ...stream, asked, send };
}

export function ExposureTable() {
  const rows = [
    { name: "ABCUSD", size: "12.40", exposure: "31.2 %", over: true },
    { name: "DEFUSD", size: "4.15", exposure: "27.6 %", over: true },
    { name: "GHIUSD", size: "9.00", exposure: "9.8 %", over: false },
  ];
  return (
    <div className="border-border bg-background max-w-[680px] overflow-x-auto rounded-(--control-radius) border">
      <Table density="compact">
        <TableHeader>
          <TableRow>
            <TableHead className="text-ink-soft">Instrument</TableHead>
            <TableHead className="text-ink-soft" align="end">Size</TableHead>
            <TableHead className="text-ink-soft" align="end">Exposure</TableHead>
            <TableHead className="text-ink-soft" align="end">Limit</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {rows.map((r) => (
            <TableRow key={r.name} className="hover:bg-transparent">
              <TableCell className="text-body-s">{r.name}</TableCell>
              <TableCell align="end" className="text-body-s">{r.size}</TableCell>
              <TableCell align="end" className={`text-body-s ${r.over ? "text-accent-error-ink" : ""}`}>{r.exposure}</TableCell>
              <TableCell align="end" className="text-body-s">25%</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}

function Suggestions({ labels, column = false }: { labels: readonly string[]; column?: boolean }) {
  return (
    <div className={column ? "flex flex-col gap-2" : "flex flex-wrap gap-2"}>
      {labels.map((l) => (
        <Button key={l} size={column ? "md" : "sm"} variant="secondary" className={column ? "w-full" : ""}>
          {l}
        </Button>
      ))}
    </div>
  );
}

/** The conversation from the full-chat frame, plus the live turn. */
export function FullThread({ asked, shown, streaming }: { asked: string | null; shown: string; streaming: boolean }) {
  return (
    <div className="mx-auto flex w-full max-w-[720px] flex-col gap-6">
      <ChatMarker type="separator" />
      <div className="flex flex-col items-end gap-2">
        <ChatMessage role="user" name="You" time="13:04">
          <ChatBubble variant="user">Summarize my current exposure and flag anything above the 25% single-name limit.</ChatBubble>
        </ChatMessage>
        <ChatAttachment type="file" name="q3-statement.pdf" meta="PDF · 1.2 MB" />
      </div>
      <ChatMessage role="assistant" name="Assistant" time="13:04">
        <ChatMarker type="status" state="done" />
        <ChatMarker type="tool" state="done" title="get_positions" detail="account=main · scope=all" meta="0.4s" />
        <ChatMarker type="tool" state="done" title="get_limits" detail="policy=single-name" meta="0.2s" />
        <ChatBubble variant="assistant">
          Net exposure is 38% of equity. Two positions exceed the 25% single-name limit — ABCUSD and DEFUSD:
        </ChatBubble>
        <ExposureTable />
        <ChatBubble variant="assistant">
          I can draft a rebalancing plan that brings both under the limit, or set alerts at 24% so you are warned earlier.
        </ChatBubble>
        <Suggestions labels={["Draft rebalancing plan", "Set alerts at 24%", "Export as PDF"]} />
      </ChatMessage>
      <div className="flex justify-end">
        <ChatMessage role="user" name="You" time="13:06">
          <ChatBubble variant="user">Set alerts at 24%.</ChatBubble>
        </ChatMessage>
      </div>
      <ChatMessage role="assistant" name="Assistant" time="13:06">
        <ChatMarker type="tool" state={streaming ? "running" : "done"} title="create_alert" detail="instrument=ABCUSD,DEFUSD · threshold=24%" meta={streaming ? "Running…" : "0.9s"} />
        {streaming ? <ChatMarker type="status" state="running" /> : null}
        {shown ? <ChatBubble variant="assistant" streaming={streaming}>{shown}</ChatBubble> : null}
      </ChatMessage>
      {asked ? (
        <div className="flex justify-end">
          <ChatMessage role="user" name="You" time="13:07">
            <ChatBubble variant="user">{asked}</ChatBubble>
          </ChatMessage>
        </div>
      ) : null}
    </div>
  );
}

export function AssistantComposer({
  streaming,
  onSend,
  onStop,
  placeholder = "Ask about positions, risk or orders…",
  hint,
  attachments,
}: {
  streaming: boolean;
  onSend: (t: string) => void;
  onStop: () => void;
  placeholder?: string;
  hint?: ReactNode;
  attachments?: ReactNode;
}) {
  return (
    <ChatComposer
      streaming={streaming}
      onSend={onSend}
      onStop={onStop}
      onAttach={() => {}}
      placeholder={streaming ? "Assistant is responding — you can queue a follow-up" : placeholder}
      tools={<ComposerTools />}
      hint={hint}
      attachments={attachments}
    />
  );
}

const THREADS = {
  Today: ["Q3 exposure review", "Margin call rules"],
  "Previous 7 days": ["Reconcile fills", "Funding rate summary", "Draft risk memo"],
} as const;

export function ThreadList({ className }: { className?: string }) {
  const [current, setCurrent] = useState("Q3 exposure review");
  return (
    <div className={`bg-background flex flex-col gap-3 p-3 ${className ?? ""}`}>
      <div className="flex h-6 items-center justify-between gap-2">
        <span className="text-body text-ink font-medium">Chats</span>
        <Button size="sm" variant="secondary">New chat</Button>
      </div>
      <div className="relative">
        <SearchIcon className="text-ink-soft pointer-events-none absolute top-1/2 left-2 size-4 -translate-y-1/2" />
        <Input aria-label="Search chats" placeholder="Search chats" className="pl-8" />
      </div>
      {Object.entries(THREADS).map(([group, names]) => (
        <div key={group} className="flex flex-col gap-0.5">
          <span className="text-caption text-ink-soft px-2 pt-2 pb-1">{group}</span>
          {names.map((n) => (
            <button
              key={n}
              type="button"
              aria-current={n === current ? "true" : undefined}
              onClick={() => setCurrent(n)}
              className={`text-body-s focus-visible:ring-ring h-7 rounded-(--control-radius) px-2 text-left outline-none focus-visible:ring-2 ${n === current ? "bg-active text-ink font-medium" : "text-ink-mid hover:bg-hover"}`}
            >
              {n}
            </button>
          ))}
        </div>
      ))}
      <span className="flex-1" />
      <div className="flex items-center gap-2 pt-2">
        <Avatar shape="square" size="sm">
          <AvatarFallback>J</AvatarFallback>
        </Avatar>
        <span className="text-body-s text-ink-mid">jane.doe</span>
        <span className="flex-1" />
        <Badge variant="neutral">Pro</Badge>
      </div>
    </div>
  );
}

/** Header, log and composer of the full chat screen. */
export function FullChat() {
  const a = useAssistant();
  return (
    <div className="flex min-h-0 min-w-0 flex-1 flex-col">
      <div className="border-border flex h-11 shrink-0 items-center gap-2.5 border-b pr-4 pl-5">
        <span className="text-body text-ink font-medium">Q3 exposure review</span>
        <Badge variant="neutral">Agent</Badge>
        <span className="flex-1" />
        <IconButton aria-label="Export" size="sm"><UploadIcon /></IconButton>
        <IconButton aria-label="More" size="sm"><MoreIcon /></IconButton>
      </div>
      <ChatPanel
        className="min-h-0 flex-1"
        composer={
          <div className="mx-auto w-full max-w-[720px] px-4 pb-5">
            <AssistantComposer
              streaming={a.streaming}
              onSend={a.send}
              onStop={a.stop}
              hint="Enter to send · Shift+Enter for a new line"
              attachments={<ChatAttachment type="image" state="uploading" name="exposure-chart.png" progress={62} />}
            />
          </div>
        }
      >
        <FullThread asked={a.asked} shown={a.shown} streaming={a.streaming} />
      </ChatPanel>
    </div>
  );
}

export function DockThread({ streaming, shown }: { streaming: boolean; shown: string }) {
  return (
    <>
      <div className="flex justify-end">
        <ChatMessage role="user" compact>
          <ChatBubble variant="user">Why did ABCUSD move −2.97% in the last hour?</ChatBubble>
        </ChatMessage>
      </div>
      <ChatMessage role="assistant" compact>
        <ChatMarker type="tool" state="done" title="get_candles" detail="ABCUSD · 1m · 60" meta="0.4s" />
        <ChatBubble variant="assistant">
          Price fell with a volume spike at 12:41 and funding turned positive. Most of the move happened in two candles.
        </ChatBubble>
        {shown ? <ChatBubble variant="assistant" streaming={streaming}>{shown}</ChatBubble> : null}
        {streaming ? <ChatMarker type="status" state="running" /> : null}
      </ChatMessage>
    </>
  );
}

const SUMMARY = ["Summarize my exposure", "Explain today’s P&L", "Find unfilled orders"] as const;

export function DockPanel({ empty = false }: { empty?: boolean }) {
  const a = useAssistant();
  return (
    <ChatDock
      className="h-[560px] w-full max-w-[380px]"
      title="Assistant"
      defaultExpanded
      actions={<IconButton aria-label="History" size="sm"><HistoryIcon /></IconButton>}
      empty={
        empty && !a.asked ? (
          <div className="flex flex-col gap-4 px-4">
            <h3 className="font-display text-h2 text-ink font-light">How can I help?</h3>
            <p className="text-body-s text-ink-soft">Ask about positions, risk, orders or documents. Answers can run tools on your account.</p>
            <Suggestions labels={SUMMARY} column />
          </div>
        ) : undefined
      }
      composer={
        <div className="px-4 pb-4">
          <AssistantComposer streaming={a.streaming} onSend={a.send} onStop={a.stop} placeholder="Ask the assistant…" />
        </div>
      }
    >
      {empty && !a.asked ? null : <DockThread streaming={a.streaming} shown={a.shown} />}
    </ChatDock>
  );
}
