import { useState } from "react";
import { Button } from "@evinvest/uikit";
import { Caption, PageRoot } from "../../gallery/ui";
import { AppFrame, ChatAside, DeleteAlertOverlay, EditAlertAside, NavigationAside, StaticRail, WatchlistAside } from "../parts/aside-parts";

function Stage({ caption, children }: { caption: string; children: React.ReactNode }) {
  return (
    <section className="flex min-w-0 flex-col gap-2">
      <Caption>{caption}</Caption>
      {children}
    </section>
  );
}

function OverlayStage() {
  const [open, setOpen] = useState(false);
  return (
    <Stage caption="Overlay · dialog aside over a scrim — same anatomy, floats instead of docking">
      <AppFrame
        aside={<StaticRail value="alerts" />}
        children={
          <div className="bg-background flex h-full items-center justify-center p-4">
            <Button variant="secondary" onClick={() => setOpen(true)}>
              Open the delete-alert dialog aside
            </Button>
          </div>
        }
      />
      <DeleteAlertOverlay open={open} onOpenChange={setOpen} />
    </Stage>
  );
}

export default function AsidesScreen() {
  return (
    <PageRoot
      title="Asides"
      lede="Five stages of the same shell: a menu aside, a dialog aside with a sticky footer, an AI chat aside, an overlay, and a left navigation aside. The page body is a neutral placeholder."
    >
      <Stage caption="Docked right · menu aside (watchlist) with rail">
        <AppFrame
          aside={
            <>
              <WatchlistAside onClose={() => {}} />
              <StaticRail value="list" />
            </>
          }
        />
      </Stage>
      <Stage caption="Docked right · dialog aside (edit form, sticky footer)">
        <AppFrame
          aside={
            <>
              <EditAlertAside onClose={() => {}} />
              <StaticRail value="alerts" />
            </>
          }
        />
      </Stage>
      <Stage caption="Docked right · AI chat aside">
        <AppFrame
          aside={
            <>
              <ChatAside onClose={() => {}} />
              <StaticRail value="chat" />
            </>
          }
        />
      </Stage>
      <OverlayStage />
      <Stage caption="Docked left · navigation aside, right rail with all asides closed">
        <AppFrame left={<NavigationAside onClose={() => {}} />} aside={<StaticRail value={null} />} />
      </Stage>
    </PageRoot>
  );
}
