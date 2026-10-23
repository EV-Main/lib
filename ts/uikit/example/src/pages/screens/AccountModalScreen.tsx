import { Button, SectionNav, type SectionNavGroup } from "@evinvest/uikit";
import {
  CreditCardIcon,
  FileTextIcon,
  GlobeIcon,
  HelpIcon,
  KeyboardIcon,
  LinkIcon,
  SlidersIcon,
  SwapIcon,
  UsersIcon,
} from "../../gallery/icons";
import { Block, PageRoot, Specimen } from "../../gallery/ui";
import { AccountSettingsButton, ConfirmVariants, MyInformation, ToastTrio } from "../parts/account-parts";

const SIDEBAR: readonly SectionNavGroup[] = [
  {
    id: "main",
    items: [
      { id: "info", label: "My information", icon: SlidersIcon },
      { id: "billing", label: "Billing", icon: CreditCardIcon },
      { id: "orders", label: "Orders", icon: SwapIcon },
      { id: "integrations", label: "Integrations", icon: LinkIcon },
      { id: "access", label: "Access", icon: UsersIcon },
      { id: "support", label: "Support center", icon: HelpIcon },
      { id: "shortcuts", label: "Keyboard shortcuts", icon: KeyboardIcon },
      { id: "language", label: "Language", icon: GlobeIcon },
      { id: "notes", label: "Release notes", icon: FileTextIcon },
    ],
  },
];

/** The modal laid out in the page, so the frame can be read without opening anything. */
function ModalPreview() {
  return (
    <div className="border-border bg-popover flex flex-col overflow-hidden rounded-lg border md:flex-row">
      <div className="border-border-subtle bg-card flex shrink-0 flex-col gap-2 border-b p-2 md:w-60 md:border-r md:border-b-0">
        <SectionNav className="w-full" groups={SIDEBAR} value="info" onValueChange={() => {}} />
        <div className="border-border-subtle flex items-center border-t pt-2">
          <Button variant="ghost">Sign out</Button>
        </div>
      </div>
      <div className="flex min-w-0 flex-1 flex-col gap-5 p-6 md:p-8">
        <div className="flex flex-col gap-1">
          <h2 className="font-display text-h1 text-ink font-light">Account settings</h2>
          <p className="text-body text-ink-mid">Manage your account information and security settings</p>
        </div>
        <MyInformation />
      </div>
    </div>
  );
}

export default function AccountModalScreen() {
  return (
    <PageRoot
      title="Account modal"
      lede="The account settings modal and its companions: a confirm dialog and three toasts. The first block is the modal's layout in the page; the real dialog opens from the button."
    >
      <Block title="Account settings">
        <div className="flex flex-col gap-3">
          <Specimen>
            <AccountSettingsButton />
          </Specimen>
          <ModalPreview />
        </div>
      </Block>
      <Block title="Confirm dialog" note="Delete workspace? is the danger tone from the frame. Each tone has a slow and a rejecting confirm.">
        <Specimen>
          <ConfirmVariants />
        </Specimen>
      </Block>
      <Block title="Toasts" note="Order filled, order rejected, alert triggered. The stack collapses and spreads on hover.">
        <Specimen>
          <ToastTrio />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
