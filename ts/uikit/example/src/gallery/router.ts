import { useSyncExternalStore } from "react";

export const DEFAULT_ROUTE = "/foundations/colors";

function subscribe(onChange: () => void): () => void {
  window.addEventListener("hashchange", onChange);
  return () => window.removeEventListener("hashchange", onChange);
}

// `#/elements/button` -> `/elements/button`; anything else is the default page.
function read(): string {
  const raw = window.location.hash.slice(1);
  return raw.startsWith("/") ? raw : DEFAULT_ROUTE;
}

export function useRoute(): string {
  return useSyncExternalStore(subscribe, read, () => DEFAULT_ROUTE);
}

export function navigate(path: string): void {
  window.location.hash = path;
}
