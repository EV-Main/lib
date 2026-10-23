"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { Avatar, AvatarFallback } from "./avatar";
import { Badge } from "./badge";
import {
  DropdownMenuItem,
  DropdownMenuRadioGroup,
  DropdownMenuRadioItem,
  DropdownMenuSeparator,
  DropdownMenuSub,
  DropdownMenuSubContent,
  DropdownMenuSubTrigger,
} from "./dropdown-menu";
import { Switch } from "./switch";
import { UiIcon } from "./ui-icons";

export interface AccountMenuTab {
  id: string;
  label: React.ReactNode;
  /** Shown instead of the menu body when this tab is current; the first tab is always the menu itself. */
  content?: React.ReactNode;
}

export interface AccountMenuProfile {
  /** Under the name on the profile card, e.g. "Free". */
  plan?: string;
  /** Makes the card a menu item that runs this and closes the menu, e.g. to open the account settings. */
  onOpenProfile?: () => void;
  tabs?: readonly AccountMenuTab[];
  /** Names the tab row for assistive tech. Default `"Sections"`. */
  tabsLabel?: string;
  /** Current tab id (controlled). */
  tab?: string;
  onTabChange?: (id: string) => void;
  /** A row with a short value badge on the right ("EN"); picking it runs `onSelect`. */
  language?: { label: string; value: string; onSelect?: () => void };
  /** A row with a switch; the menu closes after a toggle. */
  darkTheme?: { label: string; checked: boolean; onCheckedChange: (checked: boolean) => void };
  /** A submenu of radio options; the current one's label shows on the row. */
  colorTheme?: {
    label: string;
    value: string;
    options: readonly { id: string; label: string }[];
    onValueChange: (id: string) => void;
  };
  /** Glyphs for the rows, so the kit carries no icon set. */
  icons?: { language?: React.ReactNode; darkTheme?: React.ReactNode; colorTheme?: React.ReactNode };
}

/**
 * The tabs are `menuitemradio` items in a group, not a tablist: a menu may only
 * hold items, and this way the menu's own arrow-key roving reaches them.
 * Selecting one does not close the menu.
 */
export function ProfileTabs({
  tabs,
  value,
  onChange,
  label = "Sections",
}: {
  tabs: readonly AccountMenuTab[];
  value: string;
  onChange: (id: string) => void;
  label?: string | undefined;
}) {
  const move = (e: React.KeyboardEvent<HTMLButtonElement>, index: number) => {
    const step = e.key === "ArrowRight" ? 1 : e.key === "ArrowLeft" ? -1 : 0;
    if (step === 0) return;
    // Up and Down belong to the menu; Left and Right stay in the row.
    e.stopPropagation();
    e.preventDefault();
    const target = (index + step + tabs.length) % tabs.length;
    const next = tabs[target];
    if (!next) return;
    onChange(next.id);
    e.currentTarget.parentElement?.querySelectorAll<HTMLElement>('[role="menuitemradio"]')[target]?.focus();
  };
  return (
    <div role="group" aria-label={label} data-slot="account-menu-tabs" className="flex border-b border-border-subtle">
      {tabs.map((t, i) => {
        const selected = t.id === value;
        return (
          <button
            key={t.id}
            type="button"
            role="menuitemradio"
            aria-checked={selected}
            tabIndex={-1}
            onClick={() => onChange(t.id)}
            onKeyDown={(e) => move(e, i)}
            className={cn(
              "-mb-px h-8 flex-1 border-b px-3 text-body font-medium outline-none focus-visible:ring-2 focus-visible:ring-ring",
              selected ? "border-primary text-ink" : "border-transparent text-ink-mid hover:text-ink",
            )}
          >
            {t.label}
          </button>
        );
      })}
    </div>
  );
}

export function ProfileCard({
  name,
  plan,
  initials,
  onOpen,
}: {
  name: string;
  plan?: string | undefined;
  initials: string;
  onOpen?: (() => void) | undefined;
}) {
  const content = (
    <>
      <Avatar className="size-14 shrink-0 rounded-full">
        <AvatarFallback className="font-display text-h2 font-light text-ink">{initials.slice(0, 1)}</AvatarFallback>
      </Avatar>
      <span className="flex min-w-0 flex-1 flex-col gap-0.5">
        <span className="truncate text-body font-medium text-ink">{name}</span>
        {plan && <span className="text-caption text-ink-soft">{plan}</span>}
      </span>
      {onOpen && <UiIcon name="chevron-right" className="size-4 text-ink-soft" />}
    </>
  );
  if (!onOpen) {
    return (
      <div data-slot="account-menu-card" className="flex w-full items-center gap-2.5 p-2">
        {content}
      </div>
    );
  }
  // A real menu item: it joins the arrow-key order and closes the menu, so a dialog opened from it is not left under the menu.
  return (
    <DropdownMenuItem data-slot="account-menu-card" className="h-auto gap-2.5 p-2" onClick={onOpen}>
      {content}
    </DropdownMenuItem>
  );
}

export function ProfileSettingRows({ profile }: { profile: AccountMenuProfile }) {
  const { language, darkTheme, colorTheme, icons } = profile;
  if (!language && !darkTheme && !colorTheme) return null;
  const currentColor = colorTheme?.options.find((o) => o.id === colorTheme.value)?.label;
  return (
    <>
      <DropdownMenuSeparator />
      {language && (
        <DropdownMenuItem onClick={() => language.onSelect?.()}>
          {icons?.language}
          <span className="flex-1">{language.label}</span>
          <Badge>{language.value}</Badge>
        </DropdownMenuItem>
      )}
      {darkTheme && (
        <DropdownMenuItem
          role="menuitemcheckbox"
          aria-checked={darkTheme.checked}
          onClick={() => darkTheme.onCheckedChange(!darkTheme.checked)}
        >
          {icons?.darkTheme}
          <span className="flex-1">{darkTheme.label}</span>
          {/* Presentational: the row is the control, so the switch stays out of the tab order. */}
          <Switch checked={darkTheme.checked} tabIndex={-1} aria-hidden className="pointer-events-none" />
        </DropdownMenuItem>
      )}
      {colorTheme && (
        <DropdownMenuSub>
          <DropdownMenuSubTrigger>
            {icons?.colorTheme}
            <span className="flex-1">{colorTheme.label}</span>
            {currentColor && <span className="text-body-s text-ink-soft">{currentColor}</span>}
          </DropdownMenuSubTrigger>
          <DropdownMenuSubContent>
            <DropdownMenuRadioGroup value={colorTheme.value} onValueChange={colorTheme.onValueChange}>
              {colorTheme.options.map((o) => (
                <DropdownMenuRadioItem key={o.id} value={o.id}>
                  {o.label}
                </DropdownMenuRadioItem>
              ))}
            </DropdownMenuRadioGroup>
          </DropdownMenuSubContent>
        </DropdownMenuSub>
      )}
    </>
  );
}
