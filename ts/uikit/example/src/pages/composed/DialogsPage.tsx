import { Block, PageRoot, Specimen } from "../../gallery/ui";
import { AccountSettingsButton, ConfirmVariants, ToastTrio } from "../parts/account-parts";

export default function DialogsPage() {
  return (
    <PageRoot
      title="Account dialogs"
      lede="The account modal is a vertical tab rail and a content pane. A confirm dialog resolves a promise: it stays open and busy while the promise is pending, and shows the message when it rejects."
    >
      <Block title="AccountSettingsDialog" note="Sidebar entries are sections; My information holds the Profile and Security tabs. Escape closes it; the focus returns to the button.">
        <Specimen>
          <AccountSettingsButton />
        </Specimen>
      </Block>
      <Block title="ConfirmDialog" note="Two tones, each with a slow confirm (1.2 s) and a rejecting one (0.8 s, then the error shows under the text).">
        <Specimen>
          <ConfirmVariants />
        </Specimen>
      </Block>
      <Block title="Toasts" note="Positive, error and info, one after the other.">
        <Specimen>
          <ToastTrio />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
