import { useState } from "react";
import {
  AccountList,
  AccountListItem,
  Avatar,
  AvatarFallback,
  Badge,
  Button,
  ConfirmDialog,
  Input,
  KeyValue,
  SectionNav,
  SettingsRow,
  SettingsSection,
  Switch,
  toast,
  type SectionNavGroup,
} from "@evinvest/uikit";
import {
  ActivityIcon,
  AlertIcon,
  BellIcon,
  ChartIcon,
  CreditCardIcon,
  EyeIcon,
  FileTextIcon,
  HistoryIcon,
  KeyIcon,
  LayersIcon,
  LinkIcon,
  PlusIcon,
  SlidersIcon,
  UserIcon,
} from "../../gallery/icons";

export const SETTINGS_NAV: readonly SectionNavGroup[] = [
  { id: "general", label: "General", items: [{ id: "account", label: "Account", icon: UserIcon }, { id: "notifications", label: "Notification settings", icon: BellIcon }] },
  {
    id: "usage",
    label: "Usage & billing",
    items: [
      { id: "usage", label: "Usage", icon: ChartIcon },
      { id: "limits", label: "Limits & tiers", icon: SlidersIcon },
      { id: "billing", label: "Billing", icon: CreditCardIcon },
      { id: "invoices", label: "Invoices", icon: FileTextIcon },
    ],
  },
  {
    id: "developer",
    label: "Developer",
    items: [
      { id: "keys", label: "API keys", icon: KeyIcon },
      { id: "integrations", label: "Integrations", icon: LinkIcon },
      { id: "export", label: "Log export", icon: LayersIcon },
      { id: "retention", label: "Data retention", icon: EyeIcon },
    ],
  },
  {
    id: "monitoring",
    label: "Analytics & monitoring",
    items: [
      { id: "activity", label: "Activity", icon: ActivityIcon, trailing: <Badge variant="neutral">Beta</Badge> },
      { id: "errors", label: "Errors", icon: AlertIcon },
      { id: "history", label: "Audit history", icon: HistoryIcon },
    ],
  },
];

export function SettingsNav({ value, onValueChange }: { value: string; onValueChange: (id: string) => void }) {
  return <SectionNav className="w-full" groups={SETTINGS_NAV} value={value} onValueChange={onValueChange} />;
}

export function ProfileSection() {
  return (
    <SettingsSection title="Personal profile">
      <div className="flex flex-col gap-4">
        <Avatar shape="square" size="lg">
          <AvatarFallback>J</AvatarFallback>
        </Avatar>
        <KeyValue label="Display name" value="JD" copyText="JD" />
        <KeyValue label="Full name" value="JD" copyText="JD" />
        <KeyValue label="Email" value="jane.doe@example.com" copyText="jane.doe@example.com" />
        <KeyValue label="Username" value="jane.doe" copyText="jane.doe" />
        <div className="border-border-subtle bg-muted flex flex-wrap items-center gap-3 rounded-(--control-radius) border py-2 pr-2 pl-3">
          <span className="text-body-s text-ink-mid min-w-0 flex-1">To change your full name or email, contact support.</span>
          <Button size="sm">Contact support</Button>
        </div>
      </div>
    </SettingsSection>
  );
}

export function AccountsSection() {
  const [account, setAccount] = useState("personal");
  return (
    <SettingsSection
      title="Accounts"
      description="Switch between your personal, organization, and team accounts."
      action={
        <Button size="sm" variant="secondary">
          <PlusIcon />
          Create team
        </Button>
      }
    >
      <AccountList aria-label="Accounts" value={account} onValueChange={setAccount}>
        <AccountListItem
          value="personal"
          name="JD"
          handle="jane.doe"
          badges={
            <>
              <Badge variant="neutral">Personal</Badge>
              <Badge variant="neutral">Owner</Badge>
            </>
          }
        />
        <AccountListItem
          value="team"
          name="Desk 7"
          handle="desk-7"
          badges={<Badge variant="neutral">Team</Badge>}
          figure="$12,480.00"
        />
      </AccountList>
    </SettingsSection>
  );
}

export function DeleteSection() {
  return (
    <SettingsSection title="Delete account" tone="danger">
      <div className="flex flex-wrap items-center gap-4">
        <p className="text-body text-ink-mid min-w-0 flex-1">
          Request permanent deletion of your account and data. This sends a request to our support team for review.
        </p>
        <ConfirmDialog
          tone="danger"
          trigger={<Button variant="danger">Request deletion</Button>}
          title="Request deletion?"
          description="Support reviews the request and writes back within two business days."
          confirmLabel="Send request"
          onConfirm={() => {
            toast.info("Deletion request sent");
          }}
        />
      </div>
    </SettingsSection>
  );
}

export function NotificationRows() {
  return (
    <SettingsSection title="Notifications" flush>
      <SettingsRow label="Order fills" hint="Email and push when an order fills." inline>
        <Switch aria-label="Order fills" defaultChecked />
      </SettingsRow>
      <SettingsRow label="Price alerts" inline>
        <Switch aria-label="Price alerts" defaultChecked />
      </SettingsRow>
      <SettingsRow label="Digest" htmlFor="digest-email" divider={false}>
        <Input id="digest-email" defaultValue="jane.doe@example.com" className="max-w-xs" />
      </SettingsRow>
    </SettingsSection>
  );
}
