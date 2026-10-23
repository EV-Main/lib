import { useState } from "react";
import {
  AccountList,
  AccountListItem,
  AccountMenu,
  Badge,
  IconButton,
  type NavGroup,
} from "@evinvest/uikit";
import { GlobeIcon, MoonIcon, MoreIcon, SlidersIcon } from "../../gallery/icons";
import { setTheme, useGallery } from "../../gallery/theme";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

function Accounts() {
  const [value, setValue] = useState("personal");
  return (
    <AccountList aria-label="Accounts" value={value} onValueChange={setValue} className="max-w-xl">
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
        actions={
          <IconButton aria-label="Desk 7 options" size="sm">
            <MoreIcon />
          </IconButton>
        }
      />
      <AccountListItem value="org" name="Acme Fund" handle="acme" badges={<Badge variant="primary">Organization</Badge>} />
      <AccountListItem value="archived" name="Old desk" handle="old-desk" disabled badges={<Badge variant="neutral">Archived</Badge>} />
    </AccountList>
  );
}

const SERVICE: NavGroup[] = [
  { id: "service", items: [{ id: "settings", href: "#/screens/settings", label: "Account settings", icon: SlidersIcon }] },
];

function Menu() {
  const { theme } = useGallery();
  const [tab, setTab] = useState("profile");
  return (
    <AccountMenu
      account={{ name: "JD", email: "jane.doe@example.com" }}
      switchHref="#/composed/account-list"
      onSignOut={() => {}}
      groups={SERVICE}
      profile={{
        plan: "Free",
        tab,
        onTabChange: setTab,
        tabs: [
          { id: "profile", label: "Profile" },
          { id: "notifications", label: "Notifications", content: <p className="text-body text-ink-mid p-3">Nothing new.</p> },
        ],
        language: { label: "Language", value: "EN" },
        darkTheme: { label: "Dark theme", checked: theme === "dark", onCheckedChange: (on) => setTheme(on ? "dark" : "light") },
        colorTheme: {
          label: "Color theme",
          value: theme,
          options: [
            { id: "dark", label: "Dark" },
            { id: "light", label: "Light" },
            { id: "oldschool", label: "Oldschool" },
          ],
          onValueChange: (v) => (v === "dark" || v === "light" || v === "oldschool") && setTheme(v),
        },
        icons: { language: <GlobeIcon />, darkTheme: <MoonIcon /> },
      }}
    />
  );
}

export default function AccountListPage() {
  return (
    <PageRoot
      title="Accounts"
      lede="An account list is a radio group of rows. The account menu opens the same identity as a wide profile form."
    >
      <Block title="AccountList" note="One selected row; the arrow keys move the selection. Selected, with a figure, with a menu button, and disabled.">
        <Specimen>
          <Accounts />
        </Specimen>
      </Block>
      <Block title="AccountMenu with a profile" note="Tabs, a profile card, language, a dark-theme switch and a colour-theme select. The switch and the select drive the gallery theme.">
        <Specimen>
          <Menu />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
