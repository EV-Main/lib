import { useEffect, useMemo, useState } from "react";
import {
  CommandPalette,
  HeaderBalance,
  HeaderSearch,
  NotificationBell,
  SmartHeader,
  Switch,
  Label,
  useCommandPaletteShortcut,
  type CommandPaletteGroup,
  type HeaderNotification,
} from "@evinvest/uikit";
import { ThemeSwitch } from "./ThemeSwitch";
import { navigate } from "./router";
import { ENTRIES, TIERS, entryPath, type Entry } from "./registry";
import { THEMES, THEME_LABEL, setCompare, setTheme, type Theme } from "./theme";
import { WalletIcon } from "./icons";

const NOTES: HeaderNotification[] = [
  { id: "n1", title: "Order filled", description: "Bought 0.50 ABC at 2398.81", time: "2m ago", unread: true },
  { id: "n2", title: "Alert triggered", description: "ABCUSD crossed 2400.00", time: "14m ago", unread: true },
  { id: "n3", title: "Statement ready", description: "September statement is available", time: "1d ago" },
];

/** A balance that "loads" for 900 ms on mount; each press sets a new amount, so the roll is visible here. */
function LiveBalance() {
  const [amount, setAmount] = useState(1284.5);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    const t = setTimeout(() => setLoading(false), 900);
    return () => clearTimeout(t);
  }, []);
  return (
    <HeaderBalance
      icon={<WalletIcon />}
      amount={amount}
      locales="en"
      loading={loading}
      onClick={() => setAmount((a) => Math.max(0, Math.round((a + (Math.random() - 0.45) * 4000) * 100) / 100))}
      aria-label="Balance, press for a new amount"
    />
  );
}

function Bell() {
  const [read, setRead] = useState<readonly string[]>([]);
  const notifications = NOTES.map((n) => (read.includes(n.id) ? { ...n, unread: false } : n));
  return <NotificationBell notifications={notifications} onSelect={(id) => setRead((r) => [...r, id])} />;
}

function groupFor(tier: (typeof TIERS)[number]): CommandPaletteGroup[] {
  const items = ENTRIES.filter((e: Entry) => e.tier === tier.id).map((e) => ({
    id: entryPath(e),
    label: e.title,
    keywords: [tier.label],
    onSelect: () => navigate(entryPath(e)),
  }));
  return items.length > 0 ? [{ id: tier.id, heading: tier.label, items }] : [];
}

export function GalleryHeader({ theme, compare, title }: { theme: Theme; compare: boolean; title: string }) {
  const [open, setOpen] = useState(false);
  useCommandPaletteShortcut(() => setOpen(true));
  const groups = useMemo<CommandPaletteGroup[]>(
    () => [
      ...TIERS.flatMap(groupFor),
      {
        id: "theme",
        heading: "Theme",
        items: THEMES.map((t) => ({ id: `theme-${t}`, label: `Switch to ${THEME_LABEL[t]}`, onSelect: () => setTheme(t) })),
      },
    ],
    [],
  );
  return (
    <>
      <SmartHeader
        state="default"
        className="border-x-0 border-t-0"
        start={<span className="text-body text-ink-mid truncate">{title}</span>}
        end={
          <>
            <HeaderSearch onClick={() => setOpen(true)} placeholder="Jump to a page…" />
            <LiveBalance />
            <Bell />
            <span className="flex items-center gap-2">
              <Switch id="compare-header" checked={compare} onCheckedChange={setCompare} />
              <Label htmlFor="compare-header" className="text-body-s text-ink-mid">
                Compare
              </Label>
            </span>
            <ThemeSwitch theme={theme} />
          </>
        }
      />
      <CommandPalette open={open} onOpenChange={setOpen} groups={groups} labels={{ placeholder: "Jump to a page or switch the theme…" }} />
    </>
  );
}
