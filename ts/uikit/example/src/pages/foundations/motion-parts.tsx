import { useEffect, useState, useSyncExternalStore, type ReactNode } from "react";
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  Settled,
  Skeleton,
  ToggleGroup,
  ToggleGroupItem,
  toast,
  type ToastVariant,
} from "@evinvest/uikit";
import { useComputedProp } from "../../gallery/useComputedProp";
import { Specimen } from "../../gallery/ui";

export function Replay({ onClick }: { onClick: () => void }) {
  return (
    <Button size="sm" variant="secondary" onClick={onClick}>
      Replay
    </Button>
  );
}

/** Remounts `children` on demand so an entrance plays again. */
function Replayable({ children, actions }: { children: (key: number) => ReactNode; actions?: ReactNode }) {
  const [round, setRound] = useState(0);
  return (
    <Specimen className="items-start">
      <div className="flex w-full flex-col gap-3">
        <div className="flex flex-wrap items-center gap-2">
          <Replay onClick={() => setRound((r) => r + 1)} />
          {actions}
        </div>
        {children(round)}
      </div>
    </Specimen>
  );
}

const ROWS = ["Open positions", "Net exposure", "Funding", "Realised", "Fees", "Margin"] as const;

export function RiseDemo() {
  return (
    <Replayable>
      {(key) => (
        <div key={key} data-enter="rise" className="border-border bg-secondary text-body text-ink max-w-sm rounded-lg border p-4">
          One element fades in and rises --ev-rise, once.
        </div>
      )}
    </Replayable>
  );
}

// Whole class strings, so Tailwind sees both.
const STEP = { row: "[--enter-step:var(--ev-stagger)]", section: "[--enter-step:var(--ev-stagger-section)]" } as const;

export function StaggerDemo() {
  const [step, setStep] = useState<keyof typeof STEP>("row");
  return (
    <Replayable
      actions={
        <ToggleGroup
          type="single"
          variant="outline"
          size="sm"
          aria-label="Step"
          value={step}
          onValueChange={(v) => (v === "row" || v === "section") && setStep(v)}
        >
          <ToggleGroupItem value="row">Row 35 ms</ToggleGroupItem>
          <ToggleGroupItem value="section">Section 50 ms</ToggleGroupItem>
        </ToggleGroup>
      }
    >
      {(key) => (
        <div key={`${key}-${step}`} data-enter="stagger" className={`flex max-w-sm flex-col gap-1.5 ${STEP[step]}`}>
          {ROWS.map((r) => (
            <div key={r} className="border-border bg-secondary text-body text-ink rounded-(--control-radius) border px-3 py-1.5">
              {r}
            </div>
          ))}
        </div>
      )}
    </Replayable>
  );
}

const LATENCY = ["300", "900", "2000"] as const;

export function SettledDemo() {
  const [latency, setLatency] = useState<(typeof LATENCY)[number]>("900");
  const [loading, setLoading] = useState(true);
  const [round, setRound] = useState(0);
  useEffect(() => {
    setLoading(true);
    const t = setTimeout(() => setLoading(false), Number(latency));
    return () => clearTimeout(t);
  }, [round, latency]);
  return (
    <Specimen className="items-start">
      <div className="flex w-full flex-col gap-3">
        <div className="flex flex-wrap items-center gap-2">
          <Button size="sm" variant="secondary" onClick={() => setRound((r) => r + 1)}>
            Reload
          </Button>
          <ToggleGroup
            type="single"
            variant="outline"
            size="sm"
            aria-label="Latency"
            value={latency}
            onValueChange={(v) => {
              const next = LATENCY.find((l) => l === v);
              if (next) setLatency(next);
            }}
          >
            {LATENCY.map((l) => (
              <ToggleGroupItem key={l} value={l}>
                {l} ms
              </ToggleGroupItem>
            ))}
          </ToggleGroup>
        </div>
        <Settled
          loading={loading}
          className="max-w-sm"
          skeleton={
            <div className="flex flex-col gap-2">
              <Skeleton className="h-8 w-40" />
              <Skeleton className="h-4 w-56" />
              <Skeleton className="h-4 w-48" />
            </div>
          }
        >
          <div className="flex flex-col gap-1">
            <span className="font-display text-figure-l text-ink font-light tabular-nums">2,398.73</span>
            <span className="text-body text-ink-mid">Net exposure is 38% of equity.</span>
            <span className="text-caption text-ink-soft">24h volume · Funding · Updated 13:06</span>
          </div>
        </Settled>
      </div>
    </Specimen>
  );
}

export function SkeletonDemo() {
  return (
    <Replayable>
      {(key) => (
        <div key={key} className="flex max-w-sm flex-col gap-2">
          <Skeleton className="h-8 w-40" />
          <Skeleton className="h-4 w-full" />
          <Skeleton className="h-4 w-2/3" />
        </div>
      )}
    </Replayable>
  );
}

const TONES: readonly ToastVariant[] = ["neutral", "positive", "info", "warn", "error"];

function fire(variant: ToastVariant): void {
  const title = `${variant[0]?.toUpperCase() ?? ""}${variant.slice(1)} toast`;
  if (variant === "neutral") toast(title);
  else toast[variant](title);
}

export function ToastBurst() {
  return (
    <Specimen>
      <Button
        size="sm"
        onClick={() =>
          TONES.forEach((tone, i) => {
            setTimeout(() => fire(tone), i * 140);
          })
        }
      >
        Burst of five
      </Button>
      {TONES.map((tone) => (
        <Button key={tone} size="sm" variant={tone === "error" ? "danger" : "secondary"} onClick={() => fire(tone)}>
          {tone}
        </Button>
      ))}
      <Button size="sm" variant="ghost" onClick={() => toast("Persistent: it will not close by itself", { duration: Infinity })}>
        Persistent
      </Button>
    </Specimen>
  );
}

export function PresenceDemo() {
  return (
    <Specimen>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="secondary">Menu</Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          <DropdownMenuItem>Rename</DropdownMenuItem>
          <DropdownMenuItem>Duplicate</DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem variant="danger">Delete</DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      <Dialog>
        <DialogTrigger asChild>
          <Button variant="secondary">Dialog</Button>
        </DialogTrigger>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Presence</DialogTitle>
            <DialogDescription>The scrim fades with the panel on open and on close.</DialogDescription>
          </DialogHeader>
          <DialogFooter>
            <Button>Done</Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </Specimen>
  );
}

export function EaseDemo() {
  const [moved, setMoved] = useState(false);
  const track = "bg-muted border-border relative h-6 w-full max-w-sm rounded-full border";
  const dot = (ease: string) =>
    `bg-primary absolute top-0.5 size-4 rounded-full transition-[left] duration-(--ev-dur-slow) ${ease} ${moved ? "left-[calc(100%-1.125rem)]" : "left-0.5"}`;
  return (
    <Specimen className="items-start">
      <div className="flex w-full flex-col gap-3">
        <Button size="sm" variant="secondary" className="self-start" onClick={() => setMoved((m) => !m)}>
          Move
        </Button>
        <div className="flex flex-col gap-1">
          <span className="text-caption text-ink-soft">--ev-ease-out (every entrance)</span>
          <div className={track}>
            <div className={dot("ease-(--ev-ease-out)")} />
          </div>
        </div>
        <div className="flex flex-col gap-1">
          <span className="text-caption text-ink-soft">--ev-ease-in-out (state that reverses)</span>
          <div className={track}>
            <div className={dot("ease-(--ev-ease-in-out)")} />
          </div>
        </div>
      </div>
    </Specimen>
  );
}

function TokenValue({ name }: { name: string }) {
  const [ref, value] = useComputedProp<HTMLSpanElement>(name);
  return (
    <div className="flex flex-col">
      <span className="text-caption text-ink-soft">{name}</span>
      <span ref={ref} className="text-body text-ink tabular-nums">
        {value.trim() || "…"}
      </span>
    </div>
  );
}

export function MotionTokens() {
  return (
    <Specimen>
      {["--ev-dur-fast", "--ev-dur-base", "--ev-dur-slow", "--ev-rise", "--ev-stagger", "--ev-stagger-section"].map((n) => (
        <TokenValue key={n} name={n} />
      ))}
    </Specimen>
  );
}

const REDUCED = "(prefers-reduced-motion: reduce)";
const subscribeReduced = (cb: () => void) => {
  const mq = window.matchMedia(REDUCED);
  mq.addEventListener("change", cb);
  return () => mq.removeEventListener("change", cb);
};

export function ReducedMotionNote() {
  const reduced = useSyncExternalStore(subscribeReduced, () => window.matchMedia(REDUCED).matches, () => false);
  return (
    <Specimen>
      <p className="text-body text-ink-mid max-w-prose">
        With reduced motion on, entrances keep the fade and drop the travel, dialogs and sheets fade in place, and the skeleton keeps its pulse because
        it signals work in flight. This browser currently reports <strong className="text-ink">{reduced ? "reduce" : "no preference"}</strong>.
      </p>
    </Specimen>
  );
}
