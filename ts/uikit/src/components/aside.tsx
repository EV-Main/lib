"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { Drawer, DrawerContent } from "./drawer";
import { IconButton } from "./icon-button";
import { Sheet, SheetContent } from "./sheet";
import { UiIcon } from "./ui-icons";

export interface AsideHeaderLabels {
  back?: string;
  close?: string;
}

export interface AsideHeaderProps extends Omit<React.ComponentProps<"div">, "title"> {
  title: React.ReactNode;
  /** Shows the back button first. */
  onBack?: () => void;
  /** Shows the close button last. */
  onClose?: () => void;
  /** `IconButton`s between the title and the close button: pin, more. */
  actions?: React.ReactNode;
  labels?: AsideHeaderLabels;
}

/** The 44 px title row of an aside. */
export function AsideHeader({ title, onBack, onClose, actions, labels = {}, className, ...props }: AsideHeaderProps) {
  return (
    <div
      data-slot="aside-header"
      className={cn("flex h-11 shrink-0 items-center gap-1.5 border-b border-border bg-card pl-3 pr-2", className)}
      {...props}
    >
      {onBack && (
        <IconButton aria-label={labels.back ?? "Back"} size="sm" onClick={onBack}>
          <UiIcon name="chevron-right" className="size-4 rotate-180" />
        </IconButton>
      )}
      <h2 className="min-w-0 flex-1 truncate text-body font-medium text-ink">{title}</h2>
      {actions}
      {onClose && (
        <IconButton aria-label={labels.close ?? "Close"} size="sm" onClick={onClose}>
          <UiIcon name="x" />
        </IconButton>
      )}
    </div>
  );
}

export interface AsideRailItem {
  id: string;
  /** Names the button and fills its tooltip. */
  label: string;
  icon: React.ReactNode;
}

export interface AsideRailProps extends Omit<React.ComponentProps<"div">, "onChange"> {
  items: readonly AsideRailItem[];
  /** Pinned to the bottom, e.g. settings. */
  endItems?: readonly AsideRailItem[];
  /** The open aside's id; `null` when all are closed. */
  value: string | null;
  /** Pressing the open item's button asks for `null`. */
  onValueChange: (id: string | null) => void;
  /** Accessible name of the rail. Default `"Panels"`. */
  "aria-label"?: string;
}

/** The 44 px strip of icon buttons that opens and closes the asides. */
export function AsideRail({
  items,
  endItems = [],
  value,
  onValueChange,
  className,
  "aria-label": ariaLabel = "Panels",
  ...props
}: AsideRailProps) {
  const button = (item: AsideRailItem) => (
    <IconButton
      key={item.id}
      aria-label={item.label}
      tooltip={item.label}
      pressed={value === item.id}
      onClick={() => onValueChange(value === item.id ? null : item.id)}
    >
      {item.icon}
    </IconButton>
  );
  return (
    <div
      role="group"
      aria-label={ariaLabel}
      data-slot="aside-rail"
      className={cn("flex w-11 shrink-0 flex-col items-center gap-1 border-l border-border bg-card px-1.5 py-2", className)}
      {...props}
    >
      {items.map(button)}
      <span className="flex-1" />
      {endItems.map(button)}
    </div>
  );
}

function useAtLeast(query: string): boolean {
  return React.useSyncExternalStore(
    (notify) => {
      if (typeof window.matchMedia !== "function") return () => {};
      const mq = window.matchMedia(query);
      mq.addEventListener("change", notify);
      return () => mq.removeEventListener("change", notify);
    },
    () => typeof window.matchMedia !== "function" || window.matchMedia(query).matches,
    // Server and first paint assume a wide screen; a phone swaps to the drawer on hydrate.
    () => true,
  );
}

export type AsidePanelMode = "docked" | "overlay";

export interface AsidePanelProps extends Omit<React.ComponentProps<"aside">, "title"> {
  /** Names the panel. */
  "aria-label": string;
  mode?: AsidePanelMode;
  /** Which edge of the screen it hugs. Default `right`. */
  side?: "left" | "right";
  open?: boolean;
  /** Overlay only: Escape, the scrim and the drawer drag call it with `false`. */
  onOpenChange?: (open: boolean) => void;
  /** An `AsideHeader`. */
  header?: React.ReactNode;
  /** Sticky under the body: actions. */
  footer?: React.ReactNode;
  /** Docked 380 (260 for a left navigation), overlay 420: a width class overrides it. */
  className?: string;
}

/**
 * A side panel. `docked` sits in the layout and takes room from the content;
 * `overlay` floats over a scrim on a Sheet. Below `md` both use a bottom Drawer.
 * The edge facing the content is the only border.
 */
export function AsidePanel({
  mode = "docked",
  side = "right",
  open = true,
  onOpenChange,
  header,
  footer,
  className,
  children,
  ...props
}: AsidePanelProps) {
  const wide = useAtLeast("(min-width: 768px)");
  const body = (
    <>
      {header}
      <div data-slot="aside-body" className="min-h-0 flex-1 overflow-y-auto">
        {children}
      </div>
      {footer != null && (
        <div data-slot="aside-footer" className="flex shrink-0 items-center justify-end gap-2 border-t border-border bg-card px-4 py-3">
          {footer}
        </div>
      )}
    </>
  );

  if (mode === "docked" && wide) {
    if (!open) return null;
    return (
      <aside
        data-slot="aside-panel"
        data-mode="docked"
        data-side={side}
        className={cn(
          "flex h-full w-95 shrink-0 flex-col bg-card",
          side === "right" ? "border-l border-border" : "border-r border-border",
          className,
        )}
        {...props}
      >
        {body}
      </aside>
    );
  }

  // Below md there is no room beside the content, so a docked panel floats like an overlay.
  const label = props["aria-label"];
  if (!wide) {
    return (
      <Drawer open={open} {...(onOpenChange ? { onOpenChange } : {})} direction="bottom">
        <DrawerContent aria-label={label} className="flex max-h-[85dvh] flex-col bg-card">
          {body}
        </DrawerContent>
      </Drawer>
    );
  }
  return (
    <Sheet open={open} {...(onOpenChange ? { onOpenChange } : {})}>
      <SheetContent
        side={side}
        aria-label={label}
        className={cn(
          "w-full gap-0 border-border-strong bg-card p-0 sm:max-w-105 [&>[data-slot=sheet-close]]:hidden",
          className,
        )}
      >
        {body}
      </SheetContent>
    </Sheet>
  );
}
