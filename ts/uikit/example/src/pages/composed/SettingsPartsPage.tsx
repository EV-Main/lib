import { useState } from "react";
import { Badge, Button, FormRow, Input, KeyValue, SettingsRow, SettingsSection, Switch } from "@evinvest/uikit";
import { Block, PageRoot } from "../../gallery/ui";
import { AccountsSection, DeleteSection, NotificationRows, ProfileSection, SettingsNav } from "../parts/settings-parts";

function Layout() {
  const [section, setSection] = useState("account");
  return (
    <div className="border-border overflow-hidden rounded-lg border">
      <div className="flex flex-col gap-6 p-4 lg:flex-row lg:gap-8 lg:p-6">
        <div className="shrink-0 lg:w-64">
          <SettingsNav value={section} onValueChange={setSection} />
        </div>
        <div className="flex min-w-0 flex-1 flex-col gap-6">
          <ProfileSection />
          <AccountsSection />
          <DeleteSection />
        </div>
      </div>
    </div>
  );
}

export default function SettingsPartsPage() {
  return (
    <PageRoot
      title="Settings parts"
      lede="A settings screen is a section rail beside stacked cards. Cards hold key-value pairs or rows of label and control; the destructive zone has its own edge."
    >
      <Block title="Layout" note="SettingsLayout, SectionNav, and three SettingsSections. The full Figma screen is under Screens.">
        <Layout />
      </Block>
      <Block title="SettingsSection" note="Default, with an action, flush rows, and the danger tone.">
        <div className="grid max-w-5xl gap-4 md:grid-cols-2">
          <SettingsSection title="Default" description="The body is padded.">
            <p className="text-body text-ink-mid">Cards are one surface step above the page, with a hairline.</p>
          </SettingsSection>
          <SettingsSection title="With an action" action={<Button size="sm" variant="secondary">Create team</Button>}>
            <p className="text-body text-ink-mid">The action sits at the top right.</p>
          </SettingsSection>
          <NotificationRows />
          <SettingsSection title="Danger" tone="danger">
            <p className="text-body text-ink-mid">The edge uses the error colour.</p>
          </SettingsSection>
        </div>
      </Block>
      <Block title="Rows" note="FormRow puts the label left and the control right; SettingsRow adds an inline mode that keeps a switch or a button on the line.">
        <div className="border-border bg-card max-w-2xl overflow-hidden rounded-lg border">
          <FormRow label="User name" hint="You in chat (@JD) and the address of what you publish (@JD/…).">
            <span className="text-body text-ink">JD</span>
          </FormRow>
          <FormRow label="Email" htmlFor="row-email">
            <Input id="row-email" defaultValue="jane.doe@example.com" />
          </FormRow>
          <SettingsRow label="Beta features" inline>
            <Switch aria-label="Beta features" />
          </SettingsRow>
          <SettingsRow label="Plan" inline divider={false}>
            <Badge variant="primary">Pro</Badge>
          </SettingsRow>
        </div>
      </Block>
      <Block title="KeyValue" note="A pair with an optional copy button.">
        <div className="flex max-w-md flex-col gap-3">
          <KeyValue label="Display name" value="JD" copyText="JD" />
          <KeyValue label="Instrument" value="ABCUSD" />
        </div>
      </Block>
    </PageRoot>
  );
}
