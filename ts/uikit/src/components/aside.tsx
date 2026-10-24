"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import {
  ASIDE_BODY,
  ASIDE_FOOTER,
  ASIDE_HEADER_BACK_ICON,
  ASIDE_HEADER_BASE,
  ASIDE_HEADER_TITLE,
  ASIDE_PANEL_DOCKED,
  ASIDE_PANEL_DRAWER,
  ASIDE_PANEL_SHEET,
  ASIDE_RAIL_BASE,
  ASIDE_RAIL_SPACER,
  asidePanelSideClasses,
  type AsidePanelSide,
} from "../generated/aside";
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
      className={cn(ASIDE_HEADER_BASE, className)}
      {...props}
    >
      {onBack && (
        <IconButton aria-label={labels.back ?? "Back"} size="sm" onClick={onBack}>
          <UiIcon name="chevron-right" className={ASIDE_HEADER_BACK_ICON} />
        </IconButton>
      )}
      <h2 className={ASIDE_HEADER_TITLE}>{title}</h2>
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
      className={cn(ASIDE_RAIL_BASE, className)}
      {...props}
    >
      {items.map(button)}
      <span className={ASIDE_RAIL_SPACER} />
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
  side?: AsidePanelSide;
  open?: boolean;
  /**
   * Escape, the scrim and the drawer drag call it with `false`. A docked panel
   * is only dismissable this way where it falls back to the drawer (below `md`).
   */
  onOpenChange?: (open: boolean) => void;
  /** An `AsideHeader`. */
  header?: React.ReactNode;
  /** Sticky under the body: actions. */
  footer?: React.ReactNode;
  /**
   * Below this width the panel floats on a drawer so the content keeps its room.
   * `false` keeps it docked (or on a sheet) at every width, for a framed preview
   * or an embedded layout where the viewport says nothing about the room.
   */
  collapseBelow?: "md" | false;
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
  collapseBelow = "md",
  className,
  children,
  ...props
}: AsidePanelProps) {
  const viewportWide = useAtLeast("(min-width: 768px)");
  const wide = collapseBelow === false || viewportWide;
  const body = (
    <>
      {header}
      <div data-slot="aside-body" className={ASIDE_BODY}>
        {children}
      </div>
      {footer != null && (
        <div data-slot="aside-footer" className={ASIDE_FOOTER}>
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
        className={cn(ASIDE_PANEL_DOCKED, asidePanelSideClasses[side], className)}
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
        <DrawerContent aria-label={label} className={ASIDE_PANEL_DRAWER}>
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
        className={cn(ASIDE_PANEL_SHEET, className)}
      >
        {body}
      </SheetContent>
    </Sheet>
  );
}
