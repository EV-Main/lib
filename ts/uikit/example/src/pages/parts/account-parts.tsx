import { useState, type ReactNode } from "react";
import {
  AccountSettingsDialog,
  Avatar,
  AvatarFallback,
  Badge,
  Button,
  ConfirmDialog,
  FormRow,
  IconButton,
  Input,
  SettingsRow,
  Switch,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  toast,
  type AccountSettingsSection,
} from "@evinvest/uikit";
import {
  CreditCardIcon,
  FileTextIcon,
  GlobeIcon,
  HelpIcon,
  KeyboardIcon,
  LinkIcon,
  MessageIcon,
  ShieldIcon,
  SlidersIcon,
  SwapIcon,
  UserIcon,
  UsersIcon,
} from "../../gallery/icons";

const FORM_CARD = "border-border bg-card flex flex-col overflow-hidden rounded-lg border";

function ProfileTab() {
  return (
    <div className="flex flex-col gap-5">
      <div className="border-border bg-card flex flex-wrap items-center gap-4 rounded-lg border p-5">
        <Avatar shape="circle" size="lg">
          <AvatarFallback>J</AvatarFallback>
        </Avatar>
        <div className="flex flex-col items-start gap-1.5">
          <span className="text-body text-ink font-medium">Profile picture</span>
          <span className="text-caption text-ink-mid">Recommended size is 400×400px</span>
          <Button size="sm" variant="secondary">
            Upload new
          </Button>
        </div>
      </div>
      <div className="flex flex-col gap-2">
        <span className="text-body-s text-ink-soft">Basic information</span>
        <div className={FORM_CARD}>
          <FormRow label="User name" hint="You in chat (@JD) and the address of what you publish (@JD/…).">
            <span className="text-body text-ink">JD</span>
          </FormRow>
          <FormRow label="First name">
            <span className="text-body text-ink">JD</span>
          </FormRow>
          <FormRow label="Last name">
            <span className="text-body text-ink">—</span>
          </FormRow>
          <FormRow label="Email" divider={false}>
            <span className="text-body text-ink">jane.doe@example.com</span>
          </FormRow>
        </div>
      </div>
      <Button className="self-start" onClick={() => toast.positive("Saved", { description: "Your profile is up to date." })}>
        Save changes
      </Button>
    </div>
  );
}

const SESSIONS = [
  { id: "s1", name: "MacBook · Lisbon", note: "This device", current: true },
  { id: "s2", name: "iPhone · Lisbon", note: "2 hours ago", current: false },
] as const;

function SecurityTab() {
  return (
    <div className="flex flex-col gap-5">
      <div className={FORM_CARD}>
        <SettingsRow label="Password" hint="Last changed 3 months ago.">
          <Button size="sm" variant="secondary">
            Change password
          </Button>
        </SettingsRow>
        <SettingsRow label="Two-factor authentication" hint="Ask for a code from your authenticator app when you sign in." inline>
          <Switch aria-label="Two-factor authentication" defaultChecked />
        </SettingsRow>
        <SettingsRow label="Recovery email" htmlFor="recovery" divider={false}>
          <Input id="recovery" defaultValue="jane.doe@example.com" className="max-w-xs" />
        </SettingsRow>
      </div>
      <div className="flex flex-col gap-2">
        <span className="text-body-s text-ink-soft">Active sessions</span>
        <div className={FORM_CARD}>
          {SESSIONS.map((s, i) => (
            <SettingsRow key={s.id} label={s.name} hint={s.note} divider={i < SESSIONS.length - 1} inline>
              {s.current ? (
                <Badge variant="positive">Current</Badge>
              ) : (
                <Button size="sm" variant="danger">
                  Sign out
                </Button>
              )}
            </SettingsRow>
          ))}
        </div>
      </div>
    </div>
  );
}

/** "My information": the Profile and Security tabs from the Figma frame. */
export function MyInformation() {
  return (
    <Tabs defaultValue="profile" className="gap-5">
      <TabsList className="border-border-subtle h-8 w-full justify-start gap-1 rounded-none border-b bg-transparent p-0">
        <TabsTrigger value="profile">
          <UserIcon /> Profile
        </TabsTrigger>
        <TabsTrigger value="security">
          <ShieldIcon /> Security
        </TabsTrigger>
      </TabsList>
      <TabsContent value="profile">
        <ProfileTab />
      </TabsContent>
      <TabsContent value="security">
        <SecurityTab />
      </TabsContent>
    </Tabs>
  );
}

function Empty({ name }: { name: string }) {
  return <p className="text-body text-ink-mid">{name} has no design in the Figma frame; this pane is a placeholder.</p>;
}

const ENTRIES: readonly [string, string, ReactNode][] = [
  ["billing", "Billing", <CreditCardIcon key="i" />],
  ["orders", "Orders", <SwapIcon key="i" />],
  ["integrations", "Integrations", <LinkIcon key="i" />],
  ["access", "Access", <UsersIcon key="i" />],
  ["support", "Support center", <HelpIcon key="i" />],
  ["shortcuts", "Keyboard shortcuts", <KeyboardIcon key="i" />],
  ["language", "Language", <GlobeIcon key="i" />],
  ["notes", "Release notes", <FileTextIcon key="i" />],
];

export const ACCOUNT_SECTIONS: readonly AccountSettingsSection[] = [
  { id: "info", label: "My information", icon: <SlidersIcon />, content: <MyInformation /> },
  ...ENTRIES.map(([id, label, icon]) => ({ id, label, icon, content: <Empty name={label} /> })),
];

export function AccountSettingsButton({ label = "Open account settings" }: { label?: string }) {
  const [open, setOpen] = useState(false);
  return (
    <>
      <Button variant="secondary" onClick={() => setOpen(true)}>
        {label}
      </Button>
      <AccountSettingsDialog
        open={open}
        onOpenChange={setOpen}
        title="Account settings"
        description="Manage your account information and security settings"
        sections={ACCOUNT_SECTIONS}
        sidebarFooter={
          <>
            <Button variant="ghost">Sign out</Button>
            <span className="flex-1" />
            <IconButton aria-label="Feedback" size="sm">
              <MessageIcon />
            </IconButton>
            <IconButton aria-label="Language" size="sm">
              <GlobeIcon />
            </IconButton>
          </>
        }
      />
    </>
  );
}

const slow = () => new Promise<void>((resolve) => setTimeout(resolve, 1200));
const failing = () => new Promise<void>((_, reject) => setTimeout(() => reject(new Error("The server refused the request.")), 800));

export function ConfirmVariants() {
  return (
    <>
      <ConfirmDialog
        trigger={<Button variant="secondary">Default · slow confirm</Button>}
        title="Publish the report?"
        description="Investors see it as soon as it is published."
        confirmLabel="Publish"
        onConfirm={async () => {
          await slow();
          toast.positive("Published");
        }}
      />
      <ConfirmDialog
        trigger={<Button variant="danger">Danger · slow confirm</Button>}
        tone="danger"
        title="Delete workspace?"
        description="This permanently removes the workspace, its layouts and saved drawings. This action cannot be undone."
        confirmLabel="Delete workspace"
        onConfirm={async () => {
          await slow();
          toast("Workspace deleted");
        }}
      />
      <ConfirmDialog
        trigger={<Button variant="secondary">Default · rejects</Button>}
        title="Publish the report?"
        description="A rejection keeps the dialog open and shows why."
        confirmLabel="Publish"
        onConfirm={failing}
      />
      <ConfirmDialog
        trigger={<Button variant="danger">Danger · rejects</Button>}
        tone="danger"
        title="Delete workspace?"
        description="A rejection keeps the dialog open and shows why."
        confirmLabel="Delete workspace"
        onConfirm={failing}
      />
    </>
  );
}

export function ToastTrio() {
  return (
    <Button
      variant="secondary"
      onClick={() => {
        toast.positive("Order filled", { description: "Bought 0.50 ABC at 2398.81" });
        setTimeout(() => toast.error("Order rejected", { description: "Insufficient margin for this size." }), 400);
        setTimeout(() => toast.info("Alert triggered", { description: "ABCUSD crossed 2400.00" }), 800);
      }}
    >
      Fire three toasts
    </Button>
  );
}
