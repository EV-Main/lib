import { useState } from "react";
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
  PageHeading,
  SettingsLayout,
  SystemBanner,
  Tabs,
  TabsList,
  TabsTrigger,
} from "@evinvest/uikit";
import { BellIcon } from "../../gallery/icons";
import { AccountsSection, DeleteSection, ProfileSection, SettingsNav } from "../parts/settings-parts";

const TABS = ["Home", "Overview", "Holdings", "Reports", "Automations", "Settings"] as const;

export default function SettingsScreen() {
  const [banner, setBanner] = useState(true);
  const [section, setSection] = useState("account");
  return (
    <div data-enter="stagger" className="page-root flex flex-col">
      {banner ? (
        <SystemBanner icon={<BellIcon className="size-4" />} onDismiss={() => setBanner(false)} className="rounded-none">
          New: the risk dashboard is in beta
        </SystemBanner>
      ) : (
        <span />
      )}
      <div className="border-border-subtle flex h-12 items-center border-b px-4">
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbLink href="#/screens/settings">Home</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbLink href="#/screens/settings">Settings</BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator />
            <BreadcrumbItem>
              <BreadcrumbPage>overview</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
      </div>
      <Tabs defaultValue="Settings" className="border-border-subtle overflow-x-auto border-b px-2">
        <TabsList className="h-10 bg-transparent">
          {TABS.map((t) => (
            <TabsTrigger key={t} value={t}>
              {t}
            </TabsTrigger>
          ))}
        </TabsList>
      </Tabs>
      <PageHeading
        className="border-border-subtle border-b px-4 py-7 lg:px-8"
        title="Settings"
        description="Account, billing and developer configuration."
      />
      <SettingsLayout nav={<SettingsNav value={section} onValueChange={setSection} />}>
        <ProfileSection />
        <AccountsSection />
        <DeleteSection />
      </SettingsLayout>
    </div>
  );
}
