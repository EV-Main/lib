import { useEffect, useState, type ReactNode } from "react";
import { AnimatedNumber, Button, Label, Switch, useCountTween } from "@evinvest/uikit";
import { useReducedMotion } from "../../gallery/useReducedMotion";
import { Specimen } from "../../gallery/ui";

const OPEN = 2398.81;
const USD = { style: "currency", currency: "USD" } as const;
const SIGNED_PERCENT = { style: "percent", maximumFractionDigits: 2, signDisplay: "exceptZero" } as const;
const COMPACT = { notation: "compact", maximumFractionDigits: 1 } as const;

function Figure({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="flex min-w-36 flex-col gap-1">
      <span className="text-caption text-ink-soft">{label}</span>
      <span className="font-display text-figure-l font-light text-ink">{children}</span>
    </div>
  );
}

/** A random walk that steps every 1.5 s, up and down, until paused. */
export function TickerDemo() {
  const [paused, setPaused] = useState(false);
  const [price, setPrice] = useState(OPEN);
  const [volume, setVolume] = useState(18_400);
  useEffect(() => {
    if (paused) return;
    const id = setInterval(() => {
      setPrice((p) => Math.round((p + (Math.random() - 0.5) * 60) * 100) / 100);
      setVolume((v) => Math.max(0, Math.round(v + (Math.random() - 0.4) * 9_000)));
    }, 1500);
    return () => clearInterval(id);
  }, [paused]);
  return (
    <Specimen>
      <Figure label="Price, currency">
        <AnimatedNumber value={price} format={USD} locales="en" />
      </Figure>
      <Figure label="Change, percent with sign">
        <AnimatedNumber value={price / OPEN - 1} format={SIGNED_PERCENT} locales="en" />
      </Figure>
      <Figure label="Volume, compact">
        <AnimatedNumber value={volume} format={COMPACT} locales="en" />
      </Figure>
      <div className="flex items-center gap-2">
        <Switch id="ticker-pause" checked={paused} onCheckedChange={setPaused} />
        <Label htmlFor="ticker-pause">Pause the ticker</Label>
      </div>
    </Specimen>
  );
}

function CountText({ value }: { value: number }) {
  const shown = useCountTween(value, { animateOnMount: true, from: 0, duration: 1200 });
  return <span>{new Intl.NumberFormat("en", USD).format(shown)}</span>;
}

/** Both modes and the hook count up from zero each time the block is replayed. */
export function CountOnReveal() {
  const [round, setRound] = useState(0);
  return (
    <Specimen className="items-start">
      <Button size="sm" variant="secondary" onClick={() => setRound((r) => r + 1)}>
        Replay
      </Button>
      <Figure label="mode=&quot;count&quot;">
        <AnimatedNumber key={`c${round}`} mode="count" animateOnMount value={12480.5} format={USD} locales="en" duration={1200} />
      </Figure>
      <Figure label="mode=&quot;roll&quot;">
        <AnimatedNumber key={`r${round}`} animateOnMount value={12480.5} format={USD} locales="en" />
      </Figure>
      <Figure label="useCountTween">
        <CountText key={`h${round}`} value={12480.5} />
      </Figure>
    </Specimen>
  );
}

const TRENDS = [
  { label: "default (follows the change)", trend: undefined },
  { label: "trend={1}", trend: 1 },
  { label: "trend={-1}", trend: -1 },
  { label: "trend={0}", trend: 0 },
] as const;

/** The same step fed to four columns of settings. */
export function TrendDemo() {
  const [value, setValue] = useState(1990);
  return (
    <Specimen className="items-start">
      <div className="flex gap-2">
        <Button size="sm" variant="secondary" onClick={() => setValue((v) => v + 11)}>
          +11
        </Button>
        <Button size="sm" variant="secondary" onClick={() => setValue((v) => v - 11)}>
          −11
        </Button>
        <Button size="sm" variant="secondary" onClick={() => setValue((v) => v + 2000)}>
          +2000
        </Button>
      </div>
      <div className="flex flex-wrap gap-6">
        {TRENDS.map(({ label, trend }) => (
          <Figure key={label} label={label}>
            {trend === undefined ? (
              <AnimatedNumber value={value} locales="en" />
            ) : (
              <AnimatedNumber value={value} trend={trend} locales="en" />
            )}
          </Figure>
        ))}
      </div>
    </Specimen>
  );
}

const STEPS = [9, 10, 1000, 99] as const;

/** Columns come and go as the number of digits changes. */
export function DigitsDemo() {
  const [at, setAt] = useState(0);
  return (
    <Specimen>
      <Button size="sm" variant="secondary" onClick={() => setAt((i) => (i + 1) % STEPS.length)}>
        Next: {STEPS[(at + 1) % STEPS.length]}
      </Button>
      <Figure label="9 → 10 → 1,000 → 99">
        <AnimatedNumber value={STEPS[at] ?? 0} locales="en" />
      </Figure>
    </Specimen>
  );
}

/** What the number does when the user asks for less motion. */
export function NumberReducedNote() {
  const reduced = useReducedMotion();
  return (
    <Specimen>
      <p className="max-w-prose text-body text-ink-mid">
        Under reduced motion a number swaps to its new value at once: no spin, no sliding columns, no tween frames. The text for assistive
        tech is the same in both cases. This browser currently reports <strong className="text-ink">{reduced ? "reduce" : "no preference"}</strong>.
      </p>
    </Specimen>
  );
}
