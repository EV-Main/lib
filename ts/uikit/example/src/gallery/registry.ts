import type { ComponentType } from "react";
import Brands from "../pages/foundations/Brands";
import Colors from "../pages/foundations/Colors";
import Elevation from "../pages/foundations/Elevation";
import Motion from "../pages/foundations/Motion";
import Radius from "../pages/foundations/Radius";
import Spacing from "../pages/foundations/Spacing";
import Type from "../pages/foundations/Type";

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

export const ENTRIES: readonly Entry[] = [
  { tier: "foundations", id: "colors", title: "Colors", Page: Colors },
  { tier: "foundations", id: "type", title: "Type", Page: Type },
  { tier: "foundations", id: "spacing", title: "Spacing", Page: Spacing },
  { tier: "foundations", id: "radius", title: "Radius", Page: Radius },
  { tier: "foundations", id: "elevation", title: "Elevation", Page: Elevation },
  { tier: "foundations", id: "motion", title: "Motion", Page: Motion },
  { tier: "foundations", id: "brands", title: "Brands", Page: Brands },
];

export const entryPath = (e: Entry): string => `/${e.tier}/${e.id}`;

export const findEntry = (route: string): Entry | undefined => ENTRIES.find((e) => entryPath(e) === route);

export const entriesOf = (tier: TierId): Entry[] => ENTRIES.filter((e) => e.tier === tier);
