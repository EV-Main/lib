import { Tabs, TabsContent, TabsList, TabsTrigger, ToggleGroup, ToggleGroupItem, WorkspaceTab, WorkspaceTabs } from "@evinvest/uikit";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

function TabsDemo() {
  return (
    <Tabs defaultValue="overview" className="w-full max-w-md">
      <TabsList>
        <TabsTrigger value="overview">Overview</TabsTrigger>
        <TabsTrigger value="returns">Returns</TabsTrigger>
        <TabsTrigger value="risk">Risk</TabsTrigger>
        <TabsTrigger value="off" disabled>
          Locked
        </TabsTrigger>
      </TabsList>
      <TabsContent value="overview" className="text-body text-ink-mid">
        A navy-led institutional base with tiered accents.
      </TabsContent>
      <TabsContent value="returns" className="text-body text-ink-mid">
        Headline IRR and ROI figures sit on the accent.
      </TabsContent>
      <TabsContent value="risk" className="text-body text-ink-mid">
        Drawdowns and exposure, kept legible.
      </TabsContent>
    </Tabs>
  );
}

export default function TabsPage() {
  return (
    <PageRoot title="Tabs" lede="Tabs switch a panel in place. A segmented control switches a view of the same data, and workspace tabs hold open documents.">
      <Block title="Tabs" note="Arrow keys move between tabs; the active tab is the only tab stop.">
        <Specimen>
          <TabsDemo />
        </Specimen>
      </Block>
      <Block title="Segmented" note="The kit has no Segmented export yet; a bare ToggleGroup is the stand-in for the Figma SegmentedItem.">
        <Specimen>
          <ToggleGroup type="single" aria-label="Range" defaultValue="1m">
            {["1d", "1w", "1m", "3m", "1y"].map((r) => (
              <ToggleGroupItem key={r} value={r}>
                {r}
              </ToggleGroupItem>
            ))}
          </ToggleGroup>
        </Specimen>
      </Block>
      <Block title="Workspace tabs">
        <Specimen>
          <WorkspaceTabs aria-label="Workspaces" defaultValue="portfolio">
            <WorkspaceTab value="markets" onClose={() => {}} closeLabel="Close Markets">
              Markets
            </WorkspaceTab>
            <WorkspaceTab value="portfolio" onClose={() => {}} closeLabel="Close Portfolio">
              Portfolio
            </WorkspaceTab>
            <WorkspaceTab value="research">Research</WorkspaceTab>
          </WorkspaceTabs>
        </Specimen>
      </Block>
    </PageRoot>
  );
}
