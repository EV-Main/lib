import { Button, ConfirmDialog, toast } from "@evinvest/uikit";
import { Block, PageRoot, Specimen } from "../../gallery/ui";
import { DrawerLauncher, ModalTriggers, NestedLayers } from "./overlay-parts";

function Confirms() {
  const slow = () => new Promise<void>((resolve) => setTimeout(resolve, 1200));
  const failing = () => new Promise<void>((_, reject) => setTimeout(() => reject(new Error("The server refused the request.")), 800));
  return (
    <>
      <ConfirmDialog
        trigger={<Button variant="secondary">Confirm (async)</Button>}
        title="Publish the report?"
        description="Investors see it as soon as it is published."
        confirmLabel="Publish"
        onConfirm={async () => {
          await slow();
          toast.positive("Published");
        }}
      />
      <ConfirmDialog
        trigger={<Button variant="danger">Delete (fails)</Button>}
        tone="danger"
        title="Delete the position?"
        description="A rejection keeps the dialog open and shows why."
        confirmLabel="Delete"
        onConfirm={failing}
      />
    </>
  );
}

export default function OverlaysPage() {
  return (
    <PageRoot
      title="Dialogs and sheets"
      lede="Modal layers share one scrim and one rule: Escape and an outside click close the top layer only."
    >
      <Block title="Dialog, AlertDialog, Sheet" note="Open and close each with the mouse: the scrim fades with the panel and never flashes back to full dark.">
        <Specimen>
          <ModalTriggers />
        </Specimen>
      </Block>
      <Block title="ConfirmDialog" note="Resolve to close. While a promise is pending both buttons are disabled.">
        <Specimen>
          <Confirms />
        </Specimen>
      </Block>
      <Block title="Drawer" note="Swipe towards the edge to dismiss. The body scrolls; the panel drags from its top.">
        <Specimen>
          <DrawerLauncher />
        </Specimen>
      </Block>
      <Block title="Nested layers" note="Each Escape and each click away closes one layer, the top one. A modal's backdrop closes only that modal.">
        <Specimen>
          <NestedLayers />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
