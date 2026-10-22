import type { ComponentType } from "react";

export type TierId = "foundations" | "elements" | "composed" | "screens";

export interface Tier {
  id: TierId;
  label: string;
}

/** In nav order. */
export const TIERS: readonly Tier[] = [
  { id: "foundations", label: "Foundations" },
  { id: "elements", label: "Elements" },
  { id: "composed", label: "Composed" },
  { id: "screens", label: "Screens" },
];

export interface Entry {
  tier: TierId;
  id: string;
  title: string;
  Page: ComponentType;
}

export const ENTRIES: readonly Entry[] = [];

export const entryPath = (e: Entry): string => `/${e.tier}/${e.id}`;

export const findEntry = (route: string): Entry | undefined => ENTRIES.find((e) => entryPath(e) === route);

export const entriesOf = (tier: TierId): Entry[] => ENTRIES.filter((e) => e.tier === tier);
