"use client";

import * as React from "react";
import { cn } from "../../lib/cn";
import { accentTextClasses, type Accent } from "../../generated/accent";
import { buttonVariants, type ButtonVariant } from "../atomic/button";
import { Logo } from "./logo";
import { STATUS_SCREEN_ACTIONS, STATUS_SCREEN_ACTION_ICON, STATUS_SCREEN_BODY, STATUS_SCREEN_CODE, STATUS_SCREEN_CTA, STATUS_SCREEN_EYEBROW, STATUS_SCREEN_GLOW, STATUS_SCREEN_LOGO, STATUS_SCREEN_ROOT, STATUS_SCREEN_STACK, STATUS_SCREEN_TITLE, STATUS_SCREEN_TITLE_ACCENT } from "../../generated/status-screen";

/**
 * The shared 404 / 403 / 500 status surface.
 *
 * `StatusScreen` is the generic shell; `NotFound` / `Forbidden` / `ServerError`
 * are the ready-made pages with their copy baked in — a host renders those with
 * its own hrefs. The mark is `Logo`'s, i.e. the consumer's. Links default to a
 * plain `<a>` (a full document load, which is what you want off an error page);
 * pass `linkComponent` (e.g. `next/link`) for soft navigation from a client
 * boundary.
 */
export type { Accent };

/**
 * A status CTA is a Button at the page's accent: the canonical button string for
 * that variant and rung, then the mono/uppercase treatment the error pages wear.
 *
 * Exported because a host that renders its own action into `StatusScreen`'s
 * slot — a retry button wired to Next's `reset`, say — has to be able to match
 * the CTAs beside it.
 */
export function statusCtaClass(accent: Accent, variant: ButtonVariant) {
  return cn(
    buttonVariants({ variant, size: "lg", accent }),
    STATUS_SCREEN_CTA,
  );
}

// lucide `arrow-left`, inlined so the kit keeps its zero-icon-dep footprint.
function ArrowLeftIcon({ className }: { className?: string }) {
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
      <path d="m12 19-7-7 7-7" />
      <path d="M19 12H5" />
    </svg>
  );
}

export interface StatusLinkData {
  label: string;
  href: string;
  variant?: ButtonVariant;
  leadingArrow?: boolean;
}

export interface StatusScreenProps {
  accent: Accent;
  eyebrow: string;
  code: string;
  headlineLead: string;
  headlineAccent: string;
  headlineTail?: string;
  subtext: string;
  links?: StatusLinkData[];
  /** Leading action slot, rendered before `links` (e.g. the 500 client retry). */
  children?: React.ReactNode;
  /** CTA element — `next/link` for soft nav; defaults to `<a>` (full load). */
  linkComponent?: React.ElementType;
}

/**
 * Shared skeleton for the 404 / 403 / 500 status pages: a centred hero with the
 * logo mark, a mono eyebrow, a giant Playfair code, a headline whose final clause
 * is an italic accent, supporting copy, and the CTAs. The accent threads through
 * all four marks. Actions render inline (from `links` data, plus an optional
 * `children` slot) — never passed as a JSX prop through a server boundary.
 */
export function StatusScreen({
  accent,
  eyebrow,
  code,
  headlineLead,
  headlineAccent,
  headlineTail = ".",
  subtext,
  links,
  children,
  linkComponent,
}: StatusScreenProps) {
  const L = linkComponent ?? "a";
  const accentText = accentTextClasses[accent];
  return (
    <section className={STATUS_SCREEN_ROOT}>
      <div
        aria-hidden
        className={cn(
          STATUS_SCREEN_GLOW,
          accentText,
        )}
      />
      <div className={STATUS_SCREEN_STACK}>
        <Logo className={cn(STATUS_SCREEN_LOGO, accentText)} />
        <p className={cn(STATUS_SCREEN_EYEBROW, accentText)}>{eyebrow}</p>
        <p className={cn(STATUS_SCREEN_CODE, accentText)}>{code}</p>
        <h1 className={STATUS_SCREEN_TITLE}>
          {headlineLead}
          <span className={cn(STATUS_SCREEN_TITLE_ACCENT, accentText)}>{headlineAccent}</span>
          {headlineTail}
        </h1>
        <p className={STATUS_SCREEN_BODY}>{subtext}</p>
        <div className={STATUS_SCREEN_ACTIONS}>
          {children}
          {links?.map((link) => (
            <L key={link.label} href={link.href} className={statusCtaClass(accent, link.variant ?? "primary")}>
              {link.leadingArrow ? <ArrowLeftIcon className={STATUS_SCREEN_ACTION_ICON} /> : null}
              {link.label}
            </L>
          ))}
        </div>
      </div>
    </section>
  );
}

export interface StatusPageProps {
  /** CTA element — `next/link` for soft nav; defaults to `<a>` (full load). */
  linkComponent?: React.ElementType;
  /** "Back to home" target — `/` for the landing, `/cabinet` for the cabinet. */
  homeHref?: string;
  /** Secondary CTA target (contact / request access). */
  contactHref?: string;
}

/** 404 — page not found. */
export function NotFound({ linkComponent = "a", homeHref = "/", contactHref = "/contact" }: StatusPageProps) {
  return (
    <StatusScreen
      accent="debug"
      eyebrow="Page not found"
      code="404"
      headlineLead="You've reached "
      headlineAccent="open water"
      subtext="The page you're looking for has drifted off our coastline — moved, renamed, or never charted. Let's get you back to shore."
      linkComponent={linkComponent}
      links={[
        { label: "Back to home", href: homeHref, leadingArrow: true },
        { label: "Contact the team", href: contactHref, variant: "outline" },
      ]}
    />
  );
}

/** 403 — access forbidden. */
export function Forbidden({ linkComponent = "a", homeHref = "/", contactHref = "/contact" }: StatusPageProps) {
  return (
    <StatusScreen
      accent="warn"
      eyebrow="Access forbidden"
      code="403"
      headlineLead="This harbour is "
      headlineAccent="private"
      subtext="You don't have the credentials to view this page. If you believe you should, our team can open the right doors."
      linkComponent={linkComponent}
      links={[
        { label: "Back to home", href: homeHref, leadingArrow: true },
        { label: "Request access", href: contactHref, variant: "outline" },
      ]}
    />
  );
}

export interface ServerErrorProps {
  /** CTA element — `next/link` for soft nav; defaults to `<a>` (full load). */
  linkComponent?: React.ElementType;
  /** "Back to home" target — `/` for the landing, `/cabinet` for the cabinet. */
  homeHref?: string;
  /** Retry handler (e.g. Next's error-boundary `reset`); falls back to a reload. */
  reset?: () => void;
}

/** 500 — server error. The "Try again" button runs `reset` or reloads the page. */
export function ServerError({ linkComponent = "a", homeHref = "/", reset }: ServerErrorProps) {
  return (
    <StatusScreen
      accent="error"
      eyebrow="Server error"
      code="500"
      headlineLead="Our systems are "
      headlineAccent="recalibrating"
      subtext="Something broke on our end — not yours. We've been alerted and are restoring service. Please try again in a moment."
      linkComponent={linkComponent}
      links={[{ label: "Back to home", href: homeHref, variant: "outline", leadingArrow: true }]}
    >
      <button
        type="button"
        className={statusCtaClass("error", "primary")}
        onClick={() => (reset ? reset() : window.location.reload())}
      >
        Try again
      </button>
    </StatusScreen>
  );
}
