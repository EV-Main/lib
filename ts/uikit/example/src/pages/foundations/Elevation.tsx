import { useComputedProp } from "../../gallery/useComputedProp";
import { Block, Caption, PageRoot } from "../../gallery/ui";

const LADDER = [
  { name: "background", cls: "bg-background" },
  { name: "card", cls: "bg-card" },
  { name: "secondary", cls: "bg-secondary" },
  { name: "popover", cls: "bg-popover" },
  { name: "muted", cls: "bg-muted" },
] as const;

const DEPTHS = [
  { name: "card", cls: "shadow-card" },
  { name: "elevated", cls: "shadow-elevated" },
  { name: "overlay", cls: "shadow-overlay" },
] as const;

function DepthTile({ name, cls }: { name: string; cls: string }) {
  const [ref, shadow] = useComputedProp<HTMLDivElement>("box-shadow");
  return (
    <div className="flex flex-col gap-2">
      <div ref={ref} className={`bg-card border-border flex h-24 items-center justify-center rounded-lg border ${cls}`}>
        <span className="text-body text-ink">{name}</span>
      </div>
      <Caption>
        {cls} · <span className="tabular-nums break-all">{shadow || "…"}</span>
      </Caption>
    </div>
  );
}

function DepthRow() {
  return (
    <div className="grid grid-cols-[repeat(auto-fill,minmax(12rem,1fr))] gap-4">
      {DEPTHS.map((d) => (
        <DepthTile key={d.name} {...d} />
      ))}
    </div>
  );
}

export default function Elevation() {
  return (
    <PageRoot
      title="Elevation"
      lede="Dark and Light are flat: depth comes from surface steps and hairlines. Oldschool adds real shadows under the same utilities."
    >
      <Block title="Surface ladder" note="Five steps. A panel sits one step above what it covers, with a hairline edge.">
        <div className="border-border bg-background grid grid-cols-[repeat(auto-fill,minmax(9rem,1fr))] gap-3 rounded-lg border p-4">
          {LADDER.map((s) => (
            <div key={s.name} className={`border-border text-body-s text-ink-mid flex h-20 items-end rounded-lg border p-2 ${s.cls}`}>
              {s.name}
            </div>
          ))}
        </div>
      </Block>
      <Block title="Depth in the current theme" note="The shadow utilities are zero in Dark and Light.">
        <DepthRow />
      </Block>
      <Block title="Depth in Oldschool" note="The same tiles inside an Oldschool scope, whatever the page theme is.">
        <div data-theme="oldschool" className="bg-background rounded-lg p-6">
          <DepthRow />
        </div>
      </Block>
    </PageRoot>
  );
}
