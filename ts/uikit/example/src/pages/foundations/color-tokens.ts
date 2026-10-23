/**
 * What a swatch is held to against its plane:
 * - `text`: WCAG AA for body text (4.5:1)
 * - `ui`: the non-text floor for a control edge or a focus ring (3:1)
 * - `info`: shown for the record, no floor (surface steps, tints, disabled ink)
 */
export type Floor = "text" | "ui" | "info";

export interface ColorToken {
  /** Contract name: the CSS variable is `--{name}`, the utility `bg-{name}`. */
  name: string;
  /** Contract name of the plane it sits on; `background` when omitted. */
  on?: string;
  floor: Floor;
}

export interface ColorGroup {
  id: string;
  title: string;
  note: string;
  tokens: readonly ColorToken[];
}

const info = (name: string, on?: string): ColorToken => (on ? { name, on, floor: "info" } : { name, floor: "info" });
const text = (name: string, on?: string): ColorToken => (on ? { name, on, floor: "text" } : { name, floor: "text" });
const ui = (name: string, on?: string): ColorToken => (on ? { name, on, floor: "ui" } : { name, floor: "ui" });

export const COLOR_GROUPS: readonly ColorGroup[] = [
  {
    id: "surfaces",
    title: "Surfaces",
    note: "Depth is a ladder of five steps. Each step is measured against the page.",
    tokens: [info("background"), info("card"), info("secondary"), info("popover"), info("muted")],
  },
  {
    id: "tints",
    title: "State tints",
    note: "Hover, active and selected are translucent ink laid over a surface, so they are measured over the card.",
    tokens: [info("hover", "card"), info("active", "card"), info("selected", "card"), info("scrim")],
  },
  {
    id: "edges",
    title: "Edges",
    note: "Hairlines are decorative. The control edge and the focus ring carry the 3:1 floor.",
    tokens: [info("border-subtle", "card"), info("border", "card"), info("border-strong", "card"), ui("input", "muted"), ui("ring")],
  },
  {
    id: "text",
    title: "Text",
    note: "Faint ink is for disabled labels only, so it has no floor.",
    tokens: [text("ink"), text("ink-mid"), text("ink-soft"), info("ink-faint")],
  },
  {
    id: "accent",
    title: "Accent and brand",
    note: "One accent. The fill carries white text; the ink variant is the readable accent on a surface.",
    tokens: [info("brand"), info("primary"), text("on-primary", "primary"), text("primary-ink"), info("primary-hover"), info("primary-border", "card")],
  },
  {
    id: "status",
    title: "Status",
    note: "Each tone has a fill, a label colour for that fill, and an ink for text on a surface.",
    tokens: [
      info("positive"),
      text("positive-ink"),
      info("accent-info"),
      text("accent-info-ink"),
      info("accent-warn"),
      text("accent-warn-ink"),
      info("accent-error"),
      text("accent-error-ink"),
    ],
  },
  {
    id: "charts",
    title: "Charts",
    note: "Six series colours, tuned per theme.",
    tokens: [info("chart-1"), info("chart-2"), info("chart-3"), info("chart-4"), info("chart-5"), info("chart-6")],
  },
];
