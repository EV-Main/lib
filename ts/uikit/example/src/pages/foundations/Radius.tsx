import type { ReactNode } from "react";
import { Avatar, AvatarFallback, Badge, Button, Card, CardContent, Input } from "@evinvest/uikit";
import { useComputedProp } from "../../gallery/useComputedProp";
import { Block, Caption, PageRoot } from "../../gallery/ui";

function RadiusTile({ name, utility, children }: { name: string; utility: string; children: ReactNode }) {
  const [ref, radius] = useComputedProp<HTMLDivElement>("border-top-left-radius");
  return (
    <Card className="gap-3 py-4 shadow-card">
      <CardContent className="flex flex-col gap-3 px-4">
        <div ref={ref} className={`border-input bg-muted flex h-20 items-center justify-center border ${utility}`}>
          {children}
        </div>
        <div className="flex flex-col">
          <span className="text-body text-ink font-medium">{name}</span>
          <Caption>
            {utility} · <span className="tabular-nums">{radius || "…"}</span>
          </Caption>
        </div>
      </CardContent>
    </Card>
  );
}

export default function Radius() {
  return (
    <PageRoot
      title="Radius"
      lede="Three shapes: controls, panels, and chips. Dark and Light use 4, 6 and 4 px; Oldschool uses 6, 8 and a pill."
    >
      <Block title="Shapes" note="Each tile reads its computed corner, so the number follows the active theme.">
        <div className="grid grid-cols-[repeat(auto-fill,minmax(14rem,1fr))] gap-3">
          <RadiusTile name="Control" utility="rounded-(--control-radius)">
            <Button size="sm">Button</Button>
          </RadiusTile>
          <RadiusTile name="Panel and menu" utility="rounded-lg">
            <span className="text-body-s text-ink-mid">Card, menu, dialog</span>
          </RadiusTile>
          <RadiusTile name="Chip" utility="rounded-(--chip-radius)">
            <Badge variant="neutral">Badge</Badge>
          </RadiusTile>
          <RadiusTile name="Circle" utility="rounded-full">
            <Avatar shape="circle" size="lg">
              <AvatarFallback>J</AvatarFallback>
            </Avatar>
          </RadiusTile>
        </div>
      </Block>
      <Block title="In use" note="Input and Button share the control radius; Badge and Switch share the chip radius.">
        <div className="flex flex-wrap items-center gap-3">
          <Input aria-label="Sample" placeholder="Input" className="w-48" />
          <Button variant="secondary">Secondary</Button>
          <Badge variant="primary">Accent</Badge>
          <Avatar shape="square" size="md">
            <AvatarFallback>J</AvatarFallback>
          </Avatar>
        </div>
      </Block>
    </PageRoot>
  );
}
