import { createContext, useContext, useState, type ComponentType } from "react";
import { PortalProvider } from "@evinvest/uikit";
import { THEMES, THEME_LABEL, type Theme } from "./theme";

const ScopeTheme = createContext<Theme>("dark");

/** The theme of the nearest scope; pages that read computed tokens re-read when it changes. */
export const useScopeTheme = (): Theme => useContext(ScopeTheme);

const SCOPE_ATTRS = {
  dark: {},
  light: {},
  oldschool: { "data-theme": "oldschool" },
} as const;

/**
 * One themed wrapper with its own overlay root. Tokens are inherited custom
 * properties, so an overlay portaled to `document.body` would paint in the
 * root theme; the PortalProvider keeps Dialog, Select, Popover and the rest
 * inside the wrapper that opened them.
 */
function Scope({ theme, Page }: { theme: Theme; Page: ComponentType }) {
  const [overlays, setOverlays] = useState<HTMLDivElement | null>(null);
  return (
    <ScopeTheme.Provider value={theme}>
      <div
        {...SCOPE_ATTRS[theme]}
        data-testid={`scope-${theme}`}
        className={`${theme === "light" ? "light " : ""}bg-background text-ink border-border min-w-0 rounded-lg border font-sans`}
      >
        <PortalProvider container={overlays}>
          <div className="border-border text-body-s text-ink-soft border-b px-4 py-2 font-medium">{THEME_LABEL[theme]}</div>
          <div className="overflow-x-auto">
            <Page />
          </div>
          <div ref={setOverlays} />
        </PortalProvider>
      </div>
    </ScopeTheme.Provider>
  );
}

export function CompareView({ Page }: { Page: ComponentType }) {
  return (
    <div className="grid grid-cols-1 gap-4 p-4 min-[1280px]:grid-cols-3">
      {THEMES.map((theme) => (
        <Scope key={theme} theme={theme} Page={Page} />
      ))}
    </div>
  );
}

export function SingleView({ Page, theme }: { Page: ComponentType; theme: Theme }) {
  return (
    <ScopeTheme.Provider value={theme}>
      <Page />
    </ScopeTheme.Provider>
  );
}
