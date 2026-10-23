import { useState } from "react";
import { Button } from "@evinvest/uikit";
import { Caption, PageRoot } from "../../gallery/ui";
import { AppFrame, ContentPlaceholder, DeleteAlertOverlay, NavigationAside, RailDemo, StaticRail } from "../parts/aside-parts";

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

function LeftStage() {
  const [open, setOpen] = useState(true);
  return (
    <Stage caption="Docked left · navigation aside, right rail with all asides closed">
      <AppFrame
        left={open ? <NavigationAside onClose={() => setOpen(false)} /> : null}
        aside={<StaticRail value={null} />}
        children={
          <div className="bg-background flex h-full items-center justify-center p-4">
            {open ? (
              <ContentPlaceholder />
            ) : (
              <Button variant="secondary" onClick={() => setOpen(true)}>
                Reopen the navigation aside
              </Button>
            )}
          </div>
        }
      />
    </Stage>
  );
}

export default function AsidesScreen() {
  return (
    <PageRoot
      title="Asides"
      lede="Five stages of the same shell. The first three share a live rail: press a button to slide an aside in, and its close button to shut it. The overlay and the left navigation open and close from buttons. The page body is a neutral placeholder."
    >
      <Stage caption="Docked right · menu aside (watchlist) with rail">
        <RailDemo initial="list" />
      </Stage>
      <Stage caption="Docked right · dialog aside (edit form, sticky footer)">
        <RailDemo initial="alerts" />
      </Stage>
      <Stage caption="Docked right · AI chat aside">
        <RailDemo initial="chat" />
      </Stage>
      <OverlayStage />
      <LeftStage />
    </PageRoot>
  );
}
