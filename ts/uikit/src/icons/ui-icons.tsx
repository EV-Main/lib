import * as React from "react";
import type { IconName } from "../generated/icons";
import { iconShapes } from "./icon-shapes";

// Internal: consumers bring their own icons through slots.
export function UiIcon({ name, className }: { name: IconName; className?: string | undefined }) {
  return (
    <svg
      className={className ?? "size-4"}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={1.75}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden
    >
      {iconShapes(name)}
    </svg>
  );
}
