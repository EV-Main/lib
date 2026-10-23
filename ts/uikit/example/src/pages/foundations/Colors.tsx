import { Badge, Card, CardContent, CardDescription, CardHeader, CardTitle } from "@evinvest/uikit";
import { contrast, over, toHex, type Rgba } from "../../gallery/color";
import { useComputedColors } from "../../gallery/useComputedColors";
import { Block, PageRoot } from "../../gallery/ui";
import { COLOR_GROUPS, type ColorToken, type Floor } from "./color-tokens";

const PRINCIPLES = [
  { title: "Flat", body: "No shadows, no gradients, no tinted fills. Depth comes from surface steps and hairlines." },
  { title: "Hairlines", body: "One-pixel borders at 14% ink. Strong and focus steps are used only for state." },
  { title: "Small radius", body: "4px on controls, 6px on panels and menus. Oldschool keeps 6, 8 and pill." },
  { title: "One accent", body: "Teal appears once or twice per screen. Navy is a brand color, never a surface." },
] as const;

const NAMES = COLOR_GROUPS.flatMap((g) => g.tokens.map((t) => t.name));
const FLOOR: Record<Floor, number> = { text: 4.5, ui: 3, info: 0 };

function Verdict({ ratio, floor }: { ratio: number; floor: Floor }) {
  const label = `${ratio.toFixed(2)}:1`;
  if (floor === "info") return <Badge variant="neutral">{label}</Badge>;
  const pass = ratio >= FLOOR[floor];
  return <Badge variant={pass ? "positive" : "error"}>{`${label} ${pass ? "pass" : "fail"} ${FLOOR[floor]}`}</Badge>;
}

function Swatch({ token, colors }: { token: ColorToken; colors: ReadonlyMap<string, Rgba> }) {
  const plane = colors.get(token.on ?? "background");
  const own = colors.get(token.name);
  const flat = own && plane ? over(own, plane) : undefined;
  const ratio = flat && plane ? contrast(flat, plane) : undefined;
  const planeName = token.on ?? "background";
  return (
    <figure className="border-border bg-card flex min-w-0 flex-col overflow-hidden rounded-lg border" data-testid={`swatch-${token.name}`}>
      <div className="flex h-16 items-center justify-center" style={{ background: `var(--${planeName})` }}>
        <div className="border-border size-10 rounded-(--control-radius) border" style={{ background: `var(--${token.name})` }} />
      </div>
      <figcaption className="flex flex-col gap-1.5 p-3">
        <span className="text-body text-ink font-medium">{token.name}</span>
        <span className="text-caption text-ink-soft tabular-nums">{own ? toHex(own) : "…"}</span>
        <span className="text-caption text-ink-soft">on {planeName}</span>
        {ratio === undefined ? null : <Verdict ratio={ratio} floor={token.floor} />}
      </figcaption>
    </figure>
  );
}

export default function Colors() {
  const { probe, colors } = useComputedColors(NAMES);
  return (
    <PageRoot
      title="Colors"
      lede="Tokens, type and shape for dense, technical interfaces. Every value below is a variable: a theme changes the mode, never the component."
    >
      <span ref={probe} hidden />
      <Block title="Principles">
        <div className="grid grid-cols-[repeat(auto-fill,minmax(15rem,1fr))] gap-3">
          {PRINCIPLES.map((p) => (
            <Card key={p.title} className="gap-2 py-4 shadow-card">
              <CardHeader className="px-4">
                <CardTitle className="text-h3 font-medium">{p.title}</CardTitle>
                <CardDescription className="text-body-s">{p.body}</CardDescription>
              </CardHeader>
            </Card>
          ))}
        </div>
      </Block>
      {COLOR_GROUPS.map((g) => (
        <Block key={g.id} title={g.title} note={g.note}>
          <div className="grid grid-cols-[repeat(auto-fill,minmax(9.5rem,1fr))] gap-3">
            {g.tokens.map((t) => (
              <Swatch key={t.name} token={t} colors={colors} />
            ))}
          </div>
        </Block>
      ))}
      <Block title="Reading the chips" note="Values are read from the computed style of the active theme, so switching the theme re-measures every swatch. Alpha tokens are composited over their plane first.">
        <Card className="gap-0 py-3 shadow-card">
          <CardContent className="text-body-s text-ink-mid px-4">
            Text tokens must reach 4.5:1 on the page. The control edge and the focus ring must reach 3:1.
          </CardContent>
        </Card>
      </Block>
    </PageRoot>
  );
}
