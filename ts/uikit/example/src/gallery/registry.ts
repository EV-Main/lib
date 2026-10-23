import type { ComponentType } from "react";
import BadgesPage from "../pages/elements/BadgesPage";
import BannersPage from "../pages/elements/BannersPage";
import ButtonPage from "../pages/elements/ButtonPage";
import CardsPage from "../pages/elements/CardsPage";
import DatePage from "../pages/elements/DatePage";
import FeedbackPage from "../pages/elements/FeedbackPage";
import IconButtonPage from "../pages/elements/IconButtonPage";
import InputsPage from "../pages/elements/InputsPage";
import MenusPage from "../pages/elements/MenusPage";
import NavigationPage from "../pages/elements/NavigationPage";
import OverlaysPage from "../pages/elements/OverlaysPage";
import SelectionPage from "../pages/elements/SelectionPage";
import TablesPage from "../pages/elements/TablesPage";
import TabsPage from "../pages/elements/TabsPage";
import TipsPage from "../pages/elements/TipsPage";
import ToastsPage from "../pages/elements/ToastsPage";
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
  { tier: "elements", id: "button", title: "Button", Page: ButtonPage },
  { tier: "elements", id: "icon-button", title: "IconButton", Page: IconButtonPage },
  { tier: "elements", id: "inputs", title: "Inputs", Page: InputsPage },
  { tier: "elements", id: "selection", title: "Selection", Page: SelectionPage },
  { tier: "elements", id: "tabs", title: "Tabs", Page: TabsPage },
  { tier: "elements", id: "badges", title: "Badges and status", Page: BadgesPage },
  { tier: "elements", id: "banners", title: "Banners and alerts", Page: BannersPage },
  { tier: "elements", id: "menus", title: "Menus", Page: MenusPage },
  { tier: "elements", id: "tips", title: "Tips and popovers", Page: TipsPage },
  { tier: "elements", id: "overlays", title: "Dialogs and sheets", Page: OverlaysPage },
  { tier: "elements", id: "tables", title: "Tables", Page: TablesPage },
  { tier: "elements", id: "cards", title: "Cards and lists", Page: CardsPage },
  { tier: "elements", id: "navigation", title: "Navigation", Page: NavigationPage },
  { tier: "elements", id: "feedback", title: "Progress and loading", Page: FeedbackPage },
  { tier: "elements", id: "dates", title: "Dates", Page: DatePage },
  { tier: "elements", id: "toasts", title: "Toasts", Page: ToastsPage },
];

export const entryPath = (e: Entry): string => `/${e.tier}/${e.id}`;

export const findEntry = (route: string): Entry | undefined => ENTRIES.find((e) => entryPath(e) === route);

export const entriesOf = (tier: TierId): Entry[] => ENTRIES.filter((e) => e.tier === tier);
