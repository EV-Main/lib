"use client";

import * as React from "react";
import { cn } from "../lib/cn";
import { useControllableState } from "../primitives/use-controllable-state";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
  CommandSeparator,
  CommandShortcut,
} from "./command";
import { Dialog, DialogContent, DialogTitle } from "./dialog";
import { Kbd } from "./kbd";

export interface CommandPaletteItem {
  id: string;
  label: string;
  icon?: React.ReactNode;
  /** Shown at the row end, e.g. "⌘N". */
  shortcut?: string;
  /** Extra words the search matches besides the label. */
  keywords?: readonly string[];
  disabled?: boolean;
  onSelect: () => void;
}

export interface CommandPaletteGroup {
  id: string;
  heading?: string;
  items: readonly CommandPaletteItem[];
}

export interface CommandPaletteLabels {
  /** Names the dialog for assistive tech. Default `"Command palette"`. */
  title?: string;
  placeholder?: string;
  empty?: string;
  navigate?: string;
  select?: string;
  close?: string;
}

export interface CommandPaletteProps {
  open?: boolean;
  defaultOpen?: boolean;
  onOpenChange?: (open: boolean) => void;
  groups: readonly CommandPaletteGroup[];
  labels?: CommandPaletteLabels;
  className?: string;
}

/**
 * Search-and-run over a list of actions. Choosing a row runs it and closes the
 * palette. Pair it with `useCommandPaletteShortcut` for ⌘K / Ctrl+K.
 */
export function CommandPalette({
  open,
  defaultOpen = false,
  onOpenChange,
  groups,
  labels = {},
  className,
}: CommandPaletteProps) {
  const [isOpen, setOpen] = useControllableState<boolean>({
    ...(open !== undefined ? { value: open } : {}),
    defaultValue: defaultOpen,
    ...(onOpenChange ? { onChange: onOpenChange } : {}),
  });
  return (
    <Dialog open={isOpen} onOpenChange={setOpen}>
      <DialogContent
        showCloseButton={false}
        className={cn("top-[15%] max-w-[calc(100%-2rem)] translate-y-0 gap-0 overflow-hidden bg-popover p-0 sm:max-w-[420px]", className)}
      >
        <DialogTitle className="sr-only">{labels.title ?? "Command palette"}</DialogTitle>
        <Command className="bg-transparent">
          <CommandInput placeholder={labels.placeholder ?? "Type a command or search…"} />
          <CommandList className="max-h-72">
            <CommandEmpty>{labels.empty ?? "No results."}</CommandEmpty>
            {groups.map((group, i) => (
              <React.Fragment key={group.id}>
                {i > 0 && <CommandSeparator />}
                <CommandGroup {...(group.heading !== undefined ? { heading: group.heading } : {})}>
                  {group.items.map((item) => (
                    <CommandItem
                      key={item.id}
                      value={[item.label, ...(item.keywords ?? [])].join(" ")}
                      disabled={item.disabled ?? false}
                      onSelect={() => {
                        setOpen(false);
                        item.onSelect();
                      }}
                    >
                      {item.icon}
                      <span className="min-w-0 flex-1 truncate">{item.label}</span>
                      {item.shortcut !== undefined && <CommandShortcut>{item.shortcut}</CommandShortcut>}
                    </CommandItem>
                  ))}
                </CommandGroup>
              </React.Fragment>
            ))}
          </CommandList>
        </Command>
        <div
          data-slot="command-palette-hints"
          className="flex items-center gap-3 border-t border-border-subtle px-2 py-1 text-caption text-ink-soft"
        >
          <span className="inline-flex items-center gap-1.5">
            <Kbd>↑↓</Kbd>
            {labels.navigate ?? "navigate"}
          </span>
          <span className="inline-flex items-center gap-1.5">
            <Kbd>↵</Kbd>
            {labels.select ?? "select"}
          </span>
          <span className="inline-flex items-center gap-1.5">
            <Kbd>esc</Kbd>
            {labels.close ?? "close"}
          </span>
        </div>
      </DialogContent>
    </Dialog>
  );
}

/**
 * Calls `onTrigger` on ⌘K (Mac) or Ctrl+K, and keeps the browser from taking
 * the key (Chrome focuses the address bar on Ctrl+K in some layouts).
 */
export function useCommandPaletteShortcut(onTrigger: () => void, enabled = true): void {
  // Latest callback without re-binding the listener on every render.
  const ref = React.useRef(onTrigger);
  React.useEffect(() => {
    ref.current = onTrigger;
  });
  React.useEffect(() => {
    if (!enabled) return;
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && !e.altKey && !e.shiftKey && e.key.toLowerCase() === "k") {
        e.preventDefault();
        ref.current();
      }
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [enabled]);
}
