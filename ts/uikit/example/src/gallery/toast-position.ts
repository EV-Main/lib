import { useSyncExternalStore } from "react";
import type { ToastPosition } from "@evinvest/uikit";

export const TOAST_POSITIONS: readonly ToastPosition[] = [
  "top-left",
  "top-center",
  "top-right",
  "bottom-left",
  "bottom-center",
  "bottom-right",
];

let position: ToastPosition = "bottom-right";
const listeners = new Set<() => void>();

export function setToastPosition(next: ToastPosition): void {
  position = next;
  listeners.forEach((l) => l());
}

function subscribe(onChange: () => void): () => void {
  listeners.add(onChange);
  return () => listeners.delete(onChange);
}

/** The shell's single Toaster reads this; the Toasts page writes it. */
export function useToastPosition(): ToastPosition {
  return useSyncExternalStore(subscribe, () => position, () => position);
}
