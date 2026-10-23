import { useState } from "react";
import { Button } from "@evinvest/uikit";
import { Block, PageRoot, Specimen } from "../../gallery/ui";
import { AppFrame, DeleteAlertOverlay, NavigationAside, RailDemo } from "../parts/aside-parts";

function Overlay() {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Button variant="secondary" onClick={() => setOpen(true)}>
        Open overlay aside
      </Button>
      <DeleteAlertOverlay open={open} onOpenChange={setOpen} />
    </>
  );
}

export default function AsidesPage() {
  return (
    <PageRoot
      title="Asides"
      lede="Side panels of an app shell. A docked aside sits beside the content; an overlay floats over a scrim with the same anatomy. A rail on the right edge opens and closes them."
    >
      <Block title="Rail and docked asides" note="Press a rail button: the panel slides in from its edge. Press the open one again, or the close button, to shut it. Escape does not close a docked panel.">
        <RailDemo initial="list" />
      </Block>
      <Block title="Docked left" note="A navigation aside is 260 wide and keeps its edge on the content side.">
        <AppFrame left={<NavigationAside />} />
      </Block>
      <Block title="Overlay" note="The same panel as a Sheet on a wide screen and a Drawer on a phone. Escape and the scrim close it.">
        <Specimen>
          <Overlay />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
