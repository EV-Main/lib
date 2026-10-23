"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  NavCollapse,
  NavGroupHeader,
  useNavGroupOpen,
  type NavCollapseLabels,
  type NavGroupCollapse,
} from "./nav-collapse";
import { SectionLabel } from "./page-frame";

export interface SectionNavItem {
  id: string;
  label: React.ReactNode;
  icon?: React.ComponentType<{ className?: string }>;
  /** A link (`?section=…`, a sub-route) instead of a button calling `onValueChange`. */
  href?: string;
  /** After the label — a count, a status chip. */
  trailing?: React.ReactNode;
  disabled?: boolean;
}

export interface SectionNavGroup extends NavGroupCollapse {
  id: string;
  label?: React.ReactNode;
  /** One line under the label saying what kind of thing lives here. */
  description?: React.ReactNode;
  items: readonly SectionNavItem[];
}

export interface SectionNavLabels extends NavCollapseLabels {
  /** Accessible name of the nav. Default `"Sections"`. */
  nav?: string;
}

export interface SectionNavProps extends Omit<React.ComponentProps<"nav">, "children"> {
  groups: readonly SectionNavGroup[];
  /** The current section's id. */
  value: string;
  /** Called by button items (those without `href`), and by link items on click. */
  onValueChange?: (id: string) => void;
  linkComponent?: React.ElementType;
  labels?: SectionNavLabels;
  /** Makes every labelled group collapsible; a group's own `collapsible` wins. Default `false`. */
  collapsible?: boolean;
}

const ITEM =
  "flex h-8 w-full items-center gap-2.5 rounded-(--control-radius) px-2.5 text-left text-body outline-none transition-colors focus-visible:ring-2 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-40";

/**
 * A screen's own section rail — settings panes, a document's chapters: grouped
 * rows, one current (`aria-current`), each a button or a link. The shell's rail
 * is `ShellNav`; this one lives inside a page, so it carries a tint rather than
 * the rail's solid marker and does not slide.
 */
export function SectionNav({
  groups,
  value,
  onValueChange,
  linkComponent,
  labels = {},
  collapsible = false,
  className,
  ...props
}: SectionNavProps) {
  return (
    <nav
      data-slot="section-nav"
      aria-label={labels.nav ?? "Sections"}
      className={cn("flex w-64 shrink-0 flex-col gap-0.5", className)}
      {...props}
    >
      {groups.map((group) => (
        <SectionNavGroupView
          key={group.id}
          group={group}
          value={value}
          onSelect={onValueChange}
          linkComponent={linkComponent}
          labels={labels}
          collapsible={collapsible}
        />
      ))}
    </nav>
  );
}

function SectionNavGroupView({
  group,
  value,
  onSelect,
  linkComponent,
  labels,
  collapsible,
}: {
  group: SectionNavGroup;
  value: string;
  onSelect: ((id: string) => void) | undefined;
  linkComponent: React.ElementType | undefined;
  labels: SectionNavLabels;
  collapsible: boolean;
}) {
  const labelId = React.useId();
  const regionId = React.useId();
  const [open, setOpen] = useNavGroupOpen(group, group.items.some((i) => i.id === value));
  const folds = (group.collapsible ?? collapsible) && group.label !== undefined;
  const rows = group.items.map((item) => (
    <SectionNavRow
      key={item.id}
      item={item}
      active={item.id === value}
      nested={folds}
      onSelect={onSelect}
      linkComponent={linkComponent}
    />
  ));
  if (folds) {
    return (
      <div className="flex flex-col pt-1.5 first:pt-0">
        <NavGroupHeader
          label={group.label}
          labelId={labelId}
          regionId={regionId}
          open={open}
          onToggle={() => setOpen(!open)}
          href={group.href}
          linkComponent={linkComponent}
          labels={labels}
        />
        <NavCollapse open={open} id={regionId}>
          <div className="flex flex-col gap-0.5 pt-0.5 pl-3">{rows}</div>
        </NavCollapse>
      </div>
    );
  }
  return (
    <div className="flex flex-col gap-0.5 pt-3.5 first:pt-0">
      {(group.label !== undefined || group.description !== undefined) && (
        <div className="flex flex-col">
          {group.label !== undefined && <SectionLabel>{group.label}</SectionLabel>}
          {group.description !== undefined && (
            <p className="px-2 text-caption text-ink-soft">{group.description}</p>
          )}
        </div>
      )}
      {rows}
    </div>
  );
}

function SectionNavRow({
  item,
  active,
  nested,
  onSelect,
  linkComponent,
}: {
  item: SectionNavItem;
  active: boolean;
  nested: boolean;
  onSelect: ((id: string) => void) | undefined;
  linkComponent: React.ElementType | undefined;
}) {
  const Icon = item.icon;
  const className = cn(
    ITEM,
    active ? "bg-active text-ink" : "text-ink-mid hover:bg-hover",
    nested && "h-9",
  );
  const content = (
    <>
      {Icon && <Icon className="size-4 shrink-0 text-ink-soft" />}
      <span className="min-w-0 flex-1 truncate">{item.label}</span>
      {item.trailing}
    </>
  );
  if (item.href !== undefined && !item.disabled) {
    const Link = linkComponent ?? "a";
    return (
      <Link
        href={item.href}
        aria-current={active ? "page" : undefined}
        className={className}
        onClick={() => onSelect?.(item.id)}
      >
        {content}
      </Link>
    );
  }
  return (
    <button
      type="button"
      disabled={item.disabled}
      aria-current={active ? "true" : undefined}
      className={className}
      onClick={() => onSelect?.(item.id)}
    >
      {content}
    </button>
  );
}
