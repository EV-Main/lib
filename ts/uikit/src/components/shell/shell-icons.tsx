import * as React from "react";
import { iconShapes } from "../../icons/icon-shapes";

function Glyph({ className, children }: { className?: string | undefined; children: React.ReactNode }) {
  return (
    <svg
      className={className}
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth={2}
      strokeLinecap="round"
      strokeLinejoin="round"
      aria-hidden
    >
      {children}
    </svg>
  );
}

export function ChevronLeftIcon({ className }: { className?: string | undefined }) {
  return (
    <Glyph className={className}>
      {iconShapes("chevron-left")}
    </Glyph>
  );
}

export function TriangleAlertIcon({ className }: { className?: string | undefined }) {
  return (
    <Glyph className={className}>
      {iconShapes("triangle-alert")}
    </Glyph>
  );
}

export function InfoIcon({ className }: { className?: string | undefined }) {
  return (
    <Glyph className={className}>
      {iconShapes("info")}
    </Glyph>
  );
}

export function RefreshIcon({ className }: { className?: string | undefined }) {
  return (
    <Glyph className={className}>
      {iconShapes("refresh")}
    </Glyph>
  );
}

export function CloseIcon({ className }: { className?: string | undefined }) {
  return (
    <Glyph className={className}>
      {iconShapes("close")}
    </Glyph>
  );
}
