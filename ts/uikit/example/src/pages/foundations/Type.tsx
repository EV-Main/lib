import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@evinvest/uikit";
import { Block, PageRoot } from "../../gallery/ui";

interface Style {
  name: string;
  font: string;
  size: string;
  /** The utility classes that produce the style. */
  classes: string;
  sample: string;
}

const BODY = "Net exposure is 38% of equity.";
const CAPTION = "24h volume · Funding · Updated 13:06";
const NUMS = "2402.25  2399.77  2402.24";

const SERIF = "Source Serif 4 Light";
const SANS = "Inter Regular";
const SANS_MEDIUM = "Inter Medium";

const DISPLAY: readonly Style[] = [
  { name: "Heading / H1", font: SERIF, size: "32/38", classes: "font-display text-h1 font-light", sample: "Priced by probability" },
  { name: "Heading / H2", font: SERIF, size: "24/30", classes: "font-display text-h2 font-light", sample: "Open positions" },
  { name: "Figure / L", font: SERIF, size: "28/32", classes: "font-display text-figure-l font-light tabular-nums", sample: "2,398.73" },
  { name: "Figure / M", font: SERIF, size: "20/24", classes: "font-display text-figure-m font-light tabular-nums", sample: "12.40 ABC" },
];

const TEXT: readonly Style[] = [
  { name: "Heading / H3", font: SANS_MEDIUM, size: "14/20", classes: "text-h3 font-medium", sample: "Account settings" },
  { name: "Body / L", font: SANS, size: "14/20", classes: "text-body-l", sample: BODY },
  { name: "Body / M", font: SANS, size: "13/18", classes: "text-body", sample: BODY },
  { name: "Body / M-Medium", font: SANS_MEDIUM, size: "13/18", classes: "text-body font-medium", sample: BODY },
  { name: "Body / S", font: SANS, size: "12/16", classes: "text-body-s", sample: BODY },
  { name: "Body / S-Medium", font: SANS_MEDIUM, size: "12/16", classes: "text-body-s font-medium", sample: BODY },
  { name: "Caption", font: SANS, size: "11/14", classes: "text-caption", sample: CAPTION },
  { name: "Caption-Medium", font: SANS_MEDIUM, size: "11/14", classes: "text-caption font-medium", sample: CAPTION },
  { name: "Label / Section", font: SANS, size: "12/16", classes: "text-body-s", sample: "General" },
];

const NUMERIC: readonly Style[] = [
  { name: "Num / S", font: SANS, size: "11/14", classes: "text-caption tabular-nums", sample: NUMS },
  { name: "Num / M", font: SANS, size: "12/16", classes: "text-body-s tabular-nums", sample: NUMS },
  { name: "Num / M-Medium", font: SANS_MEDIUM, size: "12/16", classes: "text-body-s font-medium tabular-nums", sample: NUMS },
  { name: "Num / L", font: SANS_MEDIUM, size: "14/20", classes: "text-body-l font-medium tabular-nums", sample: NUMS },
];

function StyleTable({ styles }: { styles: readonly Style[] }) {
  return (
    <div className="border-border bg-card shadow-card overflow-x-auto rounded-lg border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="text-ink-soft">Style</TableHead>
            <TableHead className="text-ink-soft">Font and size</TableHead>
            <TableHead className="text-ink-soft">Utilities</TableHead>
            <TableHead className="text-ink-soft">Sample</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {styles.map((s) => (
            <TableRow key={s.name} className="hover:bg-transparent">
              <TableCell className="text-body-s text-ink-mid">{s.name}</TableCell>
              <TableCell className="text-body-s text-ink-soft">
                {s.font} · {s.size}
              </TableCell>
              <TableCell className="text-caption text-ink-soft font-mono">{s.classes}</TableCell>
              <TableCell className={`text-ink py-3 ${s.classes}`}>
                <span className="whitespace-pre">{s.sample}</span>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}

export default function Type() {
  return (
    <PageRoot
      title="Type"
      lede="Two families: Source Serif 4 Light for headings and figures, Inter for everything else. Letter spacing is 0 and nothing is upper-cased."
    >
      <Block title="Display" note="Headings and figures use the serif. Figures are tabular so columns of numbers align.">
        <StyleTable styles={DISPLAY} />
      </Block>
      <Block title="Text">
        <StyleTable styles={TEXT} />
      </Block>
      <Block title="Numbers" note="Every number in a table or a ladder uses tabular figures.">
        <StyleTable styles={NUMERIC} />
      </Block>
    </PageRoot>
  );
}
