import { Block, Matrix, PageRoot } from "../../gallery/ui";

/** Whole class strings, so Tailwind sees each one. */
const SPACE = [
  { token: "space/0", px: 0, bar: "w-0" },
  { token: "space/1", px: 2, bar: "w-0.5" },
  { token: "space/2", px: 4, bar: "w-1" },
  { token: "space/3", px: 6, bar: "w-1.5" },
  { token: "space/4", px: 8, bar: "w-2" },
  { token: "space/5", px: 12, bar: "w-3" },
  { token: "space/6", px: 16, bar: "w-4" },
  { token: "space/7", px: 20, bar: "w-5" },
  { token: "space/8", px: 24, bar: "w-6" },
  { token: "space/9", px: 32, bar: "w-8" },
] as const;

const HEIGHTS = [
  { token: "height/xs", px: 20, box: "h-5", used: "Badge, Kbd, StatusItem, Avatar sm" },
  { token: "height/sm", px: 24, box: "h-6", used: "Button sm, IconButton sm, Segmented, SectionLabel" },
  { token: "height/md", px: 28, box: "h-7", used: "Button md, Input, Select, MenuItem, Avatar md" },
  { token: "height/lg", px: 32, box: "h-8", used: "Tab, NavItem, Banner" },
  { token: "touch", px: 44, box: "h-11", used: "Touch targets on coarse pointers" },
] as const;

const ICONS = [
  { token: "icon/sm", px: 12, box: "size-3" },
  { token: "icon/md", px: 14, box: "size-3.5" },
  { token: "icon/lg", px: 16, box: "size-4" },
] as const;

const MENUS = [
  { token: "menu/w-sm", px: 200, box: "w-50" },
  { token: "menu/w-md", px: 240, box: "w-60" },
  { token: "menu/w-lg", px: 300, box: "w-75" },
] as const;

function Bar({ className }: { className: string }) {
  return <div className={`bg-primary h-3 rounded-sm ${className}`} />;
}

export default function Spacing() {
  return (
    <PageRoot title="Spacing" lede="A ten-step scale on a 2 px base, and four control heights. Every border is 1 px and sits inside the box.">
      <Block title="Space scale" note="Drawn at true size with the Tailwind step that produces it.">
        <Matrix
          columns={["Pixels", "Bar"]}
          rows={SPACE.map((s) => ({ label: s.token, cells: [<span className="tabular-nums" key="px">{s.px} px</span>, <Bar key="bar" className={s.bar} />] }))}
        />
      </Block>
      <Block title="Control heights">
        <Matrix
          columns={["Pixels", "Box", "Used by"]}
          rows={HEIGHTS.map((h) => ({
            label: h.token,
            cells: [
              <span className="tabular-nums" key="px">{h.px} px</span>,
              <div key="box" className={`border-input w-24 rounded-(--control-radius) border ${h.box}`} />,
              <span className="text-body-s text-ink-soft" key="used">{h.used}</span>,
            ],
          }))}
        />
      </Block>
      <Block title="Icons and menus">
        <Matrix
          columns={["Pixels", "Box"]}
          rows={[
            ...ICONS.map((i) => ({
              label: i.token,
              cells: [<span className="tabular-nums" key="px">{i.px} px</span>, <div key="box" className={`bg-ink-mid rounded-sm ${i.box}`} />],
            })),
            ...MENUS.map((m) => ({
              label: m.token,
              cells: [<span className="tabular-nums" key="px">{m.px} px</span>, <div key="box" className={`border-border bg-popover h-6 rounded-sm border ${m.box}`} />],
            })),
          ]}
        />
      </Block>
    </PageRoot>
  );
}
