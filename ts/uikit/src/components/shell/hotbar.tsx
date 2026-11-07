"use client";

import * as React from "react";
import { cn } from "../../lib/cn";
import { toneClasses } from "../../generated/tone";
import { iconShapes } from "../../icons/icon-shapes";
import { Button } from "../atomic/button";
import { Checkbox } from "../atomic/checkbox";
import { Kbd } from "../atomic/kbd";
import { Popover, PopoverContent, PopoverTrigger } from "../atomic/popover";
import { useAppleKeys } from "./header-controls";
import { TriangleAlertIcon } from "./shell-icons";

type Letter = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z";
type Digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";
type HotbarKey = Letter | Digit;

export interface HotbarAction {
  id: string;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  /** `"N"` alone, or `"mod+K"` — ⌘ on Apple, Ctrl elsewhere. Shown in a `Kbd` and bound while the bar is mounted, pinned or not. */
  shortcut?: HotbarKey | `mod+${HotbarKey}`;
  onSelect: () => void;
  /** A toggle that is on — the agent connected, a map layer shown. The app owns the state. */
  pressed?: boolean;
  /** `false` lists it only under Customize until the user pins it. Default pinned. */
  pinned?: boolean;
}

export interface HotbarLabels {
  /** Accessible name of the strip. Default `"Hotbar"`. */
  region?: string;
  /** The customize button. Default `"Customize"`. */
  customize?: string;
  /** Default `"Customize hotbar"`. */
  title?: string;
  /** Default `"Pinned actions show in the bar and in the mobile chip row."`. */
  description?: string;
  /** Default `"Reset to default"`. */
  reset?: string;
  /** Default `"Done"`. */
  done?: string;
}

export interface HotbarProps extends Omit<React.ComponentProps<"section">, "children"> {
  /** Every action the app offers, in bar order. */
  actions: readonly HotbarAction[];
  /** `StatusItem`s at the end of the wide strip. */
  status?: React.ReactNode;
  /** An urgent level: replaces the actions with its message until the app clears it. */
  takeover?: { tone: "warn" | "error"; message: React.ReactNode; action?: React.ReactNode };
  labels?: HotbarLabels;
}

interface Shortcut {
  mod: boolean;
  code: string;
  key: HotbarKey;
}

function parseShortcut(shortcut: NonNullable<HotbarAction["shortcut"]>): Shortcut {
  const mod = shortcut.startsWith("mod+");
  const key = (mod ? shortcut.slice(4) : shortcut) as HotbarKey;
  return { mod, key, code: /\d/.test(key) ? `Digit${key}` : `Key${key}` };
}

const STORAGE_KEY = "ev-hotbar";
/** The user's pins that differ from an action's default, by action id. */
type Overrides = Readonly<Record<string, boolean>>;
const NO_OVERRIDES: Overrides = {};
let snapshot: { raw: string | null; value: Overrides } = { raw: null, value: NO_OVERRIDES };
const listeners = new Set<() => void>();

function readOverrides(): Overrides {
  const raw = localStorage.getItem(STORAGE_KEY);
  if (raw === snapshot.raw) return snapshot.value;
  const value: unknown = raw === null ? NO_OVERRIDES : JSON.parse(raw);
  if (typeof value !== "object" || value === null || Array.isArray(value) || Object.values(value).some(v => typeof v !== "boolean"))
    throw new Error(`localStorage["${STORAGE_KEY}"] is not an {actionId: boolean} map: ${raw}`);
  snapshot = { raw, value: value as Overrides };
  return snapshot.value;
}

function writeOverrides(next: Overrides) {
  if (Object.keys(next).length === 0) localStorage.removeItem(STORAGE_KEY);
  else localStorage.setItem(STORAGE_KEY, JSON.stringify(next));
  listeners.forEach(l => l());
}

function subscribeOverrides(listener: () => void) {
  const onStorage = (e: StorageEvent) => {
    if (e.key === STORAGE_KEY || e.key === null) listener();
  };
  listeners.add(listener);
  window.addEventListener("storage", onStorage);
  return () => {
    listeners.delete(listener);
    window.removeEventListener("storage", onStorage);
  };
}

const EDITABLE = "input, textarea, select, [contenteditable]:not([contenteditable='false'])";

/**
 * The bottom strip of a terminal-style app: pinned actions with bound shortcuts,
 * a customize popover, live status at the end. Position-agnostic — the app docks
 * it. Below a 64rem strip it turns into a sideways-scrolling chip row with no
 * status. Which actions are pinned is the user's, kept in this browser.
 */
export function Hotbar({ actions, status, takeover, labels = {}, className, ...props }: HotbarProps) {
  const parsed = React.useMemo(() => {
    const ids = new Set<string>();
    const codes = new Set<string>();
    return actions.map(action => {
      if (ids.has(action.id)) throw new Error(`Hotbar: two actions have the id "${action.id}"`);
      ids.add(action.id);
      if (action.shortcut === undefined) return { action, shortcut: undefined };
      const shortcut = parseShortcut(action.shortcut);
      const chord = `${shortcut.mod}:${shortcut.code}`;
      if (codes.has(chord)) throw new Error(`Hotbar: two actions are bound to ${action.shortcut}`);
      codes.add(chord);
      return { action, shortcut };
    });
  }, [actions]);

  const latest = React.useRef(parsed);
  React.useEffect(() => {
    latest.current = parsed;
  });
  React.useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.defaultPrevented || e.altKey || e.shiftKey) return;
      const mod = e.metaKey || e.ctrlKey;
      const hit = latest.current.find(({ shortcut }) => shortcut?.code === e.code && shortcut.mod === mod);
      if (hit === undefined) return;
      if (!mod && e.target instanceof Element && e.target.closest(EDITABLE)) return;
      e.preventDefault();
      // A held key would fire the action once per repeat.
      if (!e.repeat) hit.action.onSelect();
    };
    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, []);

  const overrides = React.useSyncExternalStore(subscribeOverrides, readOverrides, () => NO_OVERRIDES);
  const isPinned = (action: HotbarAction) => overrides[action.id] ?? action.pinned ?? true;
  const setPinned = (action: HotbarAction, pinned: boolean) => {
    const next: Record<string, boolean> = { ...overrides };
    delete next[action.id];
    if (pinned !== (action.pinned ?? true)) next[action.id] = pinned;
    writeOverrides(next);
  };

  const apple = useAppleKeys();
  const kbd = (s: Shortcut) => (s.mod ? (apple ? `⌘${s.key}` : `Ctrl ${s.key}`) : s.key);
  const [customizing, setCustomizing] = React.useState(false);

  return (
    <section
      data-slot="hotbar"
      aria-label={labels.region ?? "Hotbar"}
      className={cn(
        "@container border-t border-border-subtle bg-background",
        takeover && [toneClasses[takeover.tone], "border-(color:--tone-edge) bg-[image:linear-gradient(var(--tone-wash),var(--tone-wash))]"],
        className,
      )}
      {...props}
    >
      <div className="flex h-(--control-h-touch) items-center gap-2 overflow-x-auto px-3 @5xl:h-(--control-h) @5xl:gap-1.5 @5xl:overflow-visible">
        {takeover ? (
          <div data-slot="hotbar-takeover" role="alert" className="flex min-w-0 items-center gap-2 text-body-s font-medium text-(color:--tone-ink)">
            <TriangleAlertIcon className="size-4 shrink-0" />
            <span className="truncate">{takeover.message}</span>
            {takeover.action}
          </div>
        ) : (
          <>
            {parsed.filter(({ action }) => isPinned(action)).map(({ action, shortcut }) => (
              <button
                key={action.id}
                type="button"
                data-slot="hotbar-action"
                aria-pressed={action.pressed}
                onClick={action.onSelect}
                className={cn(ITEM, action.pressed && PRESSED)}
              >
                <action.icon className="size-4 shrink-0" />
                {action.label}
                {shortcut && <Kbd className="hidden @5xl:inline-flex">{kbd(shortcut)}</Kbd>}
              </button>
            ))}
            <span aria-hidden className="hidden h-4 w-px shrink-0 bg-border @5xl:block" />
            <Popover open={customizing} onOpenChange={setCustomizing}>
              <PopoverTrigger type="button" data-slot="hotbar-customize" className={cn(ITEM, "@5xl:w-(--control-h-2xs) @5xl:justify-center @5xl:px-0")}>
                <svg className="size-4 shrink-0" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth={2} strokeLinecap="round" strokeLinejoin="round" aria-hidden>
                  {iconShapes("sliders-horizontal")}
                </svg>
                <span className="@5xl:sr-only">{labels.customize ?? "Customize"}</span>
              </PopoverTrigger>
              <PopoverContent side="top" align="start" className="w-80 p-0">
                <div className="flex flex-col gap-0.5 px-3 pt-3 pb-2">
                  <p className="text-body-l font-medium">{labels.title ?? "Customize hotbar"}</p>
                  <p className="text-caption text-ink-soft">
                    {labels.description ?? "Pinned actions show in the bar and in the mobile chip row."}
                  </p>
                </div>
                <ul className="flex flex-col px-1 pb-1">
                  {parsed.map(({ action, shortcut }) => (
                    <li key={action.id}>
                      <label className="flex h-(--control-h-sm) cursor-pointer items-center gap-2.5 rounded-control px-2 text-body text-ink-mid hover:bg-hover">
                        <Checkbox checked={isPinned(action)} onCheckedChange={pinned => setPinned(action, pinned)} />
                        <action.icon className="size-4 shrink-0" />
                        <span className="flex-1 truncate">{action.label}</span>
                        {shortcut && <Kbd>{kbd(shortcut)}</Kbd>}
                      </label>
                    </li>
                  ))}
                </ul>
                <div className="flex items-center justify-between border-t border-border px-3 py-2">
                  <Button variant="ghost" size="sm" onClick={() => writeOverrides(NO_OVERRIDES)}>
                    {labels.reset ?? "Reset to default"}
                  </Button>
                  <Button size="sm" onClick={() => setCustomizing(false)}>
                    {labels.done ?? "Done"}
                  </Button>
                </div>
              </PopoverContent>
            </Popover>
          </>
        )}
        {status != null && (
          <div data-slot="hotbar-status" className="ml-auto hidden shrink-0 items-center @5xl:flex">
            {status}
          </div>
        )}
      </div>
    </section>
  );
}

// A chip below the container breakpoint, a ghost row item from it up.
const ITEM =
  "inline-flex h-(--control-h-xs) shrink-0 items-center gap-1.5 whitespace-nowrap rounded-chip border border-border bg-secondary pr-3 pl-2 text-body-s text-ink-mid transition-colors outline-none hover:text-ink focus-visible:ring-2 focus-visible:ring-ring " +
  "@5xl:h-(--control-h-2xs) @5xl:rounded-control @5xl:border-transparent @5xl:bg-transparent @5xl:px-2 @5xl:hover:bg-hover";
const PRESSED = `${toneClasses.primary} border-(color:--tone-edge) bg-(color:--tone-fill) text-(color:--tone-ink) hover:text-(color:--tone-ink) @5xl:border-(color:--tone-edge) @5xl:bg-(color:--tone-fill)`;
