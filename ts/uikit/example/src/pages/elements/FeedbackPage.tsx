import { useEffect, useState } from "react";
import { Button, Progress, Settled, Skeleton, Spinner } from "@evinvest/uikit";
import { Block, Matrix, PageRoot, Specimen } from "../../gallery/ui";
import { CountOnReveal, TickerDemo } from "../foundations/number-parts";

function ProgressDemo() {
  const [value, setValue] = useState(66);
  return (
    <Specimen>
      <div className="flex w-64 flex-col gap-2">
        <Progress value={value} aria-label="Upload" />
        <div className="flex gap-2">
          <Button size="sm" variant="secondary" onClick={() => setValue((p) => Math.max(0, p - 10))}>
            −10
          </Button>
          <Button size="sm" variant="secondary" onClick={() => setValue((p) => Math.min(100, p + 10))}>
            +10
          </Button>
        </div>
      </div>
      <div className="w-64">
        <Progress value={0} aria-label="Empty" />
      </div>
      <div className="w-64">
        <Progress value={100} aria-label="Complete" />
      </div>
    </Specimen>
  );
}

function SettledDemo() {
  const [loading, setLoading] = useState(true);
  const [round, setRound] = useState(0);
  useEffect(() => {
    setLoading(true);
    const t = setTimeout(() => setLoading(false), 800);
    return () => clearTimeout(t);
  }, [round]);
  return (
    <Specimen>
      <Button size="sm" variant="secondary" onClick={() => setRound((r) => r + 1)}>
        Reload
      </Button>
      <Settled loading={loading} skeleton={<Skeleton className="h-8 w-40" />}>
        <span className="font-display text-figure-l text-ink font-light tabular-nums">12,480.00</span>
      </Settled>
    </Specimen>
  );
}

export default function FeedbackPage() {
  return (
    <PageRoot title="Progress and loading" lede="Progress shows a known amount. A skeleton and a spinner show that work is in flight; Settled hands one over to the other.">
      <Block title="Progress">
        <ProgressDemo />
      </Block>
      <Block title="Skeleton and Spinner" note="A skeleton fades in before it pulses. The spinner keeps turning under reduced motion.">
        <Matrix
          columns={["Sample"]}
          rows={[
            { label: "Skeleton", cells: [<div key="s" className="flex w-56 flex-col gap-2"><Skeleton className="h-6 w-32" /><Skeleton className="h-4 w-full" /></div>] },
            { label: "Spinner", cells: [<Spinner key="p" />] },
          ]}
        />
      </Block>
      <Block title="Settled" note="Skeleton to content in one handover: the content rises into place.">
        <SettledDemo />
      </Block>
      <Block title="Animated number" note="A figure that moves to its new value instead of swapping. Currency, signed percent and compact all format through Intl.">
        <TickerDemo />
      </Block>
      <Block title="Count up on reveal">
        <CountOnReveal />
      </Block>
    </PageRoot>
  );
}
