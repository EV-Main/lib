// The browser's who-am-I: `GET /api/auth/session` on evinvest.ltd. Concierge's
// `SessionInfo` (runner/src/web/routes.rs) carries more than a host should
// hold — the platform role, `isAdmin`, the break-glass flag and the seat's
// platform `permissions`. The parser keeps only what the account chip shows:
// a host that holds platform privileges in its state is one `if` away from
// gating its own UI on them, and they say nothing about the host's backend.
//
// It tells a host who holds the evinvest.ltd session — not what the host's own
// backend lets that user do. Authorisation stays with the host's RP session.

import type { ParseResult } from '@evinvest/mfe';

import { CONCIERGE_ORIGIN } from './elements';

export const SESSION_URL = `${CONCIERGE_ORIGIN}/api/auth/session`;

/** Who holds the evinvest.ltd session, as much as a chip shows. Platform role and permissions are left out on purpose. */
export interface SessionUser {
  userId: string;
  email: string;
  emailVerified: boolean;
  /** Null until the user picks one. */
  username: string | null;
  /** Null when concierge sends none (it does not yet). */
  displayName: string | null;
}

export type Session = { authenticated: true; user: SessionUser } | { authenticated: false; user: null };

// Concierge serialises an unset optional string as `null`; an absent key reads
// the same. Anything else in the slot is a mistyped body, not an absent value.
function optionalString(raw: object, key: string): string | null | undefined {
  const value: unknown = Reflect.get(raw, key);
  if (value === null || value === undefined) return null;
  return typeof value === 'string' ? value : undefined;
}

function parseUser(raw: unknown): SessionUser | null {
  if (typeof raw !== 'object' || raw === null) return null;
  const userId: unknown = Reflect.get(raw, 'userId');
  const email: unknown = Reflect.get(raw, 'email');
  const emailVerified: unknown = Reflect.get(raw, 'emailVerified');
  const username = optionalString(raw, 'username');
  const displayName = optionalString(raw, 'displayName');
  if (typeof userId !== 'string' || userId === '' || typeof email !== 'string') return null;
  if (typeof emailVerified !== 'boolean' || username === undefined || displayName === undefined) return null;
  return { userId, email, emailVerified, username, displayName: displayName === '' ? null : displayName };
}

/**
 * Validate a session body and keep only {@link SessionUser}'s fields. Unknown
 * fields — `permissions`, `role`, `isAdmin`, `roleIsBreakGlass` among them —
 * are dropped; a missing or mistyped kept one fails.
 */
export function parseSession(raw: unknown): ParseResult<Session> {
  if (typeof raw !== 'object' || raw === null) return { ok: false, error: 'session is not an object' };
  const authenticated: unknown = Reflect.get(raw, 'authenticated');
  if (typeof authenticated !== 'boolean') return { ok: false, error: 'session `authenticated` is not a boolean' };
  if (!authenticated) return { ok: true, value: { authenticated: false, user: null } };
  const user = parseUser(Reflect.get(raw, 'user'));
  if (!user) return { ok: false, error: 'session `user` is missing or malformed' };
  return { ok: true, value: { authenticated: true, user } };
}

/**
 * Read the session from the browser. `credentials: "include"` because a scope
 * host (invest.evinvest.ltd, sa.evinvest.ltd) calls evinvest.ltd cross-origin
 * and the session cookie lives there. Pointless on a server: the cookie is
 * first-party to evinvest.ltd and never reaches a host's backend. Network,
 * HTTP and shape failures are results, not throws.
 */
export async function fetchSession(url: string | URL = SESSION_URL, init: RequestInit = {}): Promise<ParseResult<Session>> {
  try {
    const headers = new Headers(init.headers);
    if (!headers.has('accept')) headers.set('accept', 'application/json');
    const response = await fetch(url, { credentials: 'include', cache: 'no-store', ...init, headers });
    if (!response.ok) return { ok: false, error: `session ${String(url)}: HTTP ${response.status}` };
    return parseSession(await response.json());
  } catch (cause) {
    const reason = cause instanceof Error ? cause.message : String(cause);
    return { ok: false, error: `session ${String(url)}: ${reason}` };
  }
}
