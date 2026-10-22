import { useSyncExternalStore } from "react";

export type Theme = "dark" | "light" | "oldschool";
export const THEMES: readonly Theme[] = ["dark", "light", "oldschool"];
export const THEME_LABEL: Record<Theme, string> = { dark: "Dark", light: "Light", oldschool: "Oldschool" };

export interface GalleryState {
  theme: Theme;
  compare: boolean;
}

const STORAGE_KEY = "uikit-gallery";

export const isTheme = (v: unknown): v is Theme => THEMES.some((t) => t === v);

function readStored(): Partial<GalleryState> {
  try {
    const raw = window.localStorage.getItem(STORAGE_KEY);
    if (raw === null) return {};
    const parsed: unknown = JSON.parse(raw);
    if (typeof parsed !== "object" || parsed === null) return {};
    const out: Partial<GalleryState> = {};
    if ("theme" in parsed && isTheme(parsed.theme)) out.theme = parsed.theme;
    if ("compare" in parsed && typeof parsed.compare === "boolean") out.compare = parsed.compare;
    return out;
  } catch {
    // storage blocked or corrupt: the gallery still works, it just forgets
    return {};
  }
}

function initial(): GalleryState {
  const stored = readStored();
  const params = new URLSearchParams(window.location.search);
  const fromUrl = params.get("theme");
  return {
    theme: isTheme(fromUrl) ? fromUrl : (stored.theme ?? "dark"),
    compare: params.has("compare") ? params.get("compare") === "1" : (stored.compare ?? false),
  };
}

/**
 * Dark is the sheet's default and carries no mark. In a compare view the root
 * stays Dark: each of the three scoped wrappers sets its own theme, and an
 * html-level `.light` would leak into the Dark column.
 */
function applyToRoot(state: GalleryState): void {
  const root = document.documentElement;
  const rootTheme = state.compare ? "dark" : state.theme;
  root.classList.toggle("light", rootTheme === "light");
  if (rootTheme === "oldschool") root.dataset.theme = "oldschool";
  else delete root.dataset.theme;
}

function persist(state: GalleryState): void {
  try {
    window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
  } catch {
    // see readStored
  }
  const url = new URL(window.location.href);
  url.searchParams.set("theme", state.theme);
  if (state.compare) url.searchParams.set("compare", "1");
  else url.searchParams.delete("compare");
  window.history.replaceState(null, "", url);
}

let state = initial();
applyToRoot(state);
const listeners = new Set<() => void>();

function set(next: GalleryState): void {
  state = next;
  applyToRoot(state);
  persist(state);
  listeners.forEach((l) => l());
}

export const setTheme = (theme: Theme): void => set({ ...state, theme });
export const setCompare = (compare: boolean): void => set({ ...state, compare });

function subscribe(onChange: () => void): () => void {
  listeners.add(onChange);
  return () => listeners.delete(onChange);
}

export function useGallery(): GalleryState {
  return useSyncExternalStore(subscribe, () => state, () => state);
}
