// Outbound contact links — the conversions a small-business landing exists
// for. Classification lives in the server-safe core so the same rule decides
// both what a link *is* when rendered and what the click tracker reports.

import { type ContactChannel, WHATSAPP_HOSTS } from "../generated/contact";

export type { ContactChannel };

// `whatsapp://send` is the native deep link some pages use directly.
const whatsappHosts = new Set<string>(WHATSAPP_HOSTS);

/**
 * Which contact channel `href` opens, or `null` when it is not a contact link.
 * Relative and unparsable hrefs are never a contact link.
 */
export function contactChannel(href: string): ContactChannel | null {
  const value = href.trim().toLowerCase();
  if (value.startsWith("tel:")) return "phone";
  if (value.startsWith("mailto:")) return "email";
  if (value.startsWith("whatsapp:")) return "whatsapp";
  if (!/^https?:\/\//.test(value)) return null;
  try {
    return whatsappHosts.has(new URL(value).hostname) ? "whatsapp" : null;
  } catch {
    return null;
  }
}

/**
 * `tel:` href from a human-formatted number. RFC 3966 allows only digits,
 * `+` and visual separators; dropping everything else keeps "01 23 45 67 89"
 * and "+33 (0)1…" dialable on every phone.
 */
export function telHref(phone: string): string {
  return `tel:${phone.replace(/\(0\)/g, "").replace(/[^\d+]/g, "")}`;
}

/**
 * `wa.me` click-to-chat href. wa.me wants the international number as bare
 * digits — no `+`, no `00` exit prefix, no separators — and rejects anything
 * else with a generic "invalid link" page, which is why this normalises rather
 * than trusting the caller's formatting. A *national* number ("06 12 …") cannot
 * be fixed here without knowing the country; it is left as is so the broken
 * link is visible instead of silently dialling someone else.
 */
export function whatsappHref(phone: string, message?: string): string {
  const digits = phone
    .replace(/\(0\)/g, "")
    .replace(/\D/g, "")
    .replace(/^00/, "");
  const text = message ? `?text=${encodeURIComponent(message)}` : "";
  return `https://wa.me/${digits}${text}`;
}
