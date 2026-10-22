"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { Avatar, AvatarFallback } from "./avatar";
import { Badge } from "./badge";
import { DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator } from "./dropdown-menu";
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
  /** Makes the card a button, e.g. to open the account settings. */
  onOpenProfile?: () => void;
  tabs?: readonly AccountMenuTab[];
  /** Current tab id (controlled). */
  tab?: string;
  onTabChange?: (id: string) => void;
  /** A row with a short value badge on the right ("EN"); picking it runs `onSelect`. */
  language?: { label: string; value: string; onSelect?: () => void };
  /** A row with a switch; the menu closes after a toggle. */
  darkTheme?: { label: string; checked: boolean; onCheckedChange: (checked: boolean) => void };
  /** A labelled slot for a colour-theme `Select` and its companions. */
  colorTheme?: { label: string; control: React.ReactNode };
  /** Glyphs for the rows, so the kit carries no icon set. */
  icons?: { language?: React.ReactNode; darkTheme?: React.ReactNode };
}

export function ProfileTabs({
  tabs,
  value,
  onChange,
}: {
  tabs: readonly AccountMenuTab[];
  value: string;
  onChange: (id: string) => void;
}) {
  // Arrow keys stay inside the tab row: the surrounding menu would read them as item navigation.
  const move = (e: React.KeyboardEvent<HTMLDivElement>) => {
    const step = e.key === "ArrowRight" ? 1 : e.key === "ArrowLeft" ? -1 : 0;
    if (step === 0) return;
    e.stopPropagation();
    e.preventDefault();
    const at = tabs.findIndex((t) => t.id === value);
    const next = tabs[(at + step + tabs.length) % tabs.length];
    if (next) onChange(next.id);
  };
  return (
    <div
      role="tablist"
      aria-orientation="horizontal"
      data-slot="account-menu-tabs"
      className="flex border-b border-border-subtle"
      onKeyDown={move}
    >
      {tabs.map((t) => {
        const selected = t.id === value;
        return (
          <button
            key={t.id}
            type="button"
            role="tab"
            aria-selected={selected}
            tabIndex={selected ? 0 : -1}
            onClick={() => onChange(t.id)}
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
  const Root = onOpen ? "button" : "div";
  return (
    <Root
      {...(onOpen ? { type: "button" as const, onClick: onOpen } : {})}
      data-slot="account-menu-card"
      className={cn(
        "flex w-full items-center gap-2.5 rounded-(--control-radius) p-2 text-left outline-none",
        onOpen && "hover:bg-hover focus-visible:ring-2 focus-visible:ring-ring",
      )}
    >
      <Avatar className="size-14 shrink-0 rounded-full">
        <AvatarFallback className="font-display text-h2 font-light text-ink">{initials.slice(0, 1)}</AvatarFallback>
      </Avatar>
      <span className="flex min-w-0 flex-1 flex-col gap-0.5">
        <span className="truncate text-body font-medium text-ink">{name}</span>
        {plan && <span className="text-caption text-ink-soft">{plan}</span>}
      </span>
      {onOpen && <UiIcon name="chevron-right" className="size-4 text-ink-soft" />}
    </Root>
  );
}

export function ProfileSettingRows({ profile }: { profile: AccountMenuProfile }) {
  const { language, darkTheme, colorTheme, icons } = profile;
  if (!language && !darkTheme && !colorTheme) return null;
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
        <>
          <DropdownMenuSeparator />
          <DropdownMenuLabel>{colorTheme.label}</DropdownMenuLabel>
          <div className="flex items-center gap-1.5 px-2 pb-1">{colorTheme.control}</div>
        </>
      )}
    </>
  );
}
