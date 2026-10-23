import { useEffect, type ComponentProps } from "react";
import {
  AppShell,
  Alert,
  AlertDescription,
  AlertTitle,
  BottomTabBar,
  Label,
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  Switch,
  ShellNav,
  Toaster,
  TooltipProvider,
  type NavGroup,
  type NavItem,
} from "@evinvest/uikit";
import { CompareView, SingleView } from "./ThemeScope";
import { GalleryHeader } from "./GalleryHeader";
import { ThemeSwitch } from "./ThemeSwitch";
import { DEFAULT_ROUTE, navigate, useRoute } from "./router";
import { ENTRIES, TIERS, entriesOf, entryPath, findEntry } from "./registry";
import { setCompare, useGallery } from "./theme";
import { useToastPosition } from "./toast-position";

function HashLink({ href, ...props }: ComponentProps<"a">) {
  return <a href={href === undefined ? undefined : `#${href}`} {...props} />;
}

const NAV_GROUPS: NavGroup[] = TIERS.flatMap((tier) => {
  const items: NavItem[] = entriesOf(tier.id).map((e) => ({ id: entryPath(e), href: entryPath(e), label: e.title }));
  return items.length > 0 ? [{ id: tier.id, label: tier.label, items }] : [];
});

const TAB_ITEMS: NavItem[] = TIERS.flatMap((tier) => {
  const first = entriesOf(tier.id)[0];
  return first ? [{ id: tier.id, href: entryPath(first), label: tier.label, also: [`/${tier.id}`] }] : [];
});

function PageSelect({ route }: { route: string }) {
  const current = findEntry(route);
  return (
    <Select value={current ? route : undefined} onValueChange={navigate}>
      <SelectTrigger size="sm" className="w-44" aria-label="Page">
        <span className="truncate">{current?.title ?? "Pick a page"}</span>
      </SelectTrigger>
      <SelectContent>
        {TIERS.map((tier) => (
          <SelectGroup key={tier.id}>
            <SelectLabel>{tier.label}</SelectLabel>
            {entriesOf(tier.id).map((e) => (
              <SelectItem key={entryPath(e)} value={entryPath(e)}>
                {e.title}
              </SelectItem>
            ))}
          </SelectGroup>
        ))}
      </SelectContent>
    </Select>
  );
}

function CompareToggle({ compare }: { compare: boolean }) {
  return (
    <div className="flex items-center gap-2">
      <Switch id="compare-themes" checked={compare} onCheckedChange={setCompare} />
      <Label htmlFor="compare-themes" className="text-body-s text-ink-mid">
        Compare themes
      </Label>
    </div>
  );
}

function NotFound({ route }: { route: string }) {
  return (
    <div className="p-4 lg:p-8">
      <Alert>
        <AlertTitle>No such page</AlertTitle>
        <AlertDescription>
          <code>{route}</code> is not in the registry.{" "}
          <a className="text-primary-ink underline" href={`#${DEFAULT_ROUTE}`}>
            Back to Foundations
          </a>
        </AlertDescription>
      </Alert>
    </div>
  );
}

export function Gallery() {
  const route = useRoute();
  const { theme, compare } = useGallery();
  const entry = findEntry(route);
  const toastPosition = useToastPosition();

  useEffect(() => {
    window.scrollTo(0, 0);
    document.title = `${entry?.title ?? "Gallery"} · @evinvest/uikit`;
  }, [route, entry]);

  return (
    <TooltipProvider>
      <AppShell
        rail={
          <ShellNav
            groups={NAV_GROUPS}
            pathname={route}
            linkComponent={HashLink}
            header={<span className="text-h3 text-ink px-3 font-medium">@evinvest/uikit</span>}
            footer={<span className="text-caption text-ink-soft px-3 pb-3">v1.0 gallery</span>}
          />
        }
        tabBar={<BottomTabBar items={TAB_ITEMS} pathname={route} linkComponent={HashLink} />}
        topBar={
          <GalleryHeader
            theme={theme}
            compare={compare}
            title={entry ? `${TIERS.find((t) => t.id === entry.tier)?.label} / ${entry.title}` : "Gallery"}
          />
        }
      >
        <div className="border-border bg-background sticky top-0 z-30 flex flex-col gap-2 border-b px-4 py-2 lg:hidden">
          <div className="flex items-center justify-between gap-2">
            <PageSelect route={route} />
            <ThemeSwitch theme={theme} compact />
          </div>
          <CompareToggle compare={compare} />
        </div>
        {entry ? (
          compare ? (
            <CompareView key={route} Page={entry.Page} />
          ) : (
            <SingleView key={route} Page={entry.Page} theme={theme} />
          )
        ) : (
          <NotFound route={ENTRIES.length === 0 ? `${route} (registry is empty)` : route} />
        )}
      </AppShell>
      <Toaster position={toastPosition} />
    </TooltipProvider>
  );
}
