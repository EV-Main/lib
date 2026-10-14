// The browser's who-am-I: `GET /api/auth/session` on evinvest.ltd. The shape is
// concierge's `SessionInfo` (runner/src/web/routes.rs): a camelCase `user` when
// signed in, the seat's concrete `permissions` either way.
//
// It tells a host who holds the evinvest.ltd session — not what the host's own
// backend lets that user do. Authorisation stays with the host's RP session.

import type { ParseResult } from '@evinvest/mfe';

import { CONCIERGE_ORIGIN } from './elements';

export const SESSION_URL = `${CONCIERGE_ORIGIN}/api/auth/session`;

export interface SessionUser {
  userId: string;
  email: string;
  emailVerified: boolean;
  /** Null until the user picks one. */
  username: string | null;
  status: string;
  role: string;
  isAdmin: boolean;
  /** The role comes from the emergency allowlist, not the directory: temporary by construction. */
  roleIsBreakGlass: boolean;
}

export type Session =
  | { authenticated: true; user: SessionUser; permissions: readonly string[] }
  | { authenticated: false; user: null; permissions: readonly string[] };

function isStringArray(value: unknown): value is string[] {
  return Array.isArray(value) && value.every(item => typeof item === 'string');
}

function parseUser(raw: unknown): SessionUser | null {
  if (typeof raw !== 'object' || raw === null) return null;
  const userId: unknown = Reflect.get(raw, 'userId');
  const email: unknown = Reflect.get(raw, 'email');
  const emailVerified: unknown = Reflect.get(raw, 'emailVerified');
  const username: unknown = Reflect.get(raw, 'username');
  const status: unknown = Reflect.get(raw, 'status');
  const role: unknown = Reflect.get(raw, 'role');
  const isAdmin: unknown = Reflect.get(raw, 'isAdmin');
  const roleIsBreakGlass: unknown = Reflect.get(raw, 'roleIsBreakGlass');
  if (typeof userId !== 'string' || userId === '' || typeof email !== 'string') return null;
  if (typeof emailVerified !== 'boolean' || typeof isAdmin !== 'boolean' || typeof roleIsBreakGlass !== 'boolean') return null;
  if (typeof status !== 'string' || typeof role !== 'string') return null;
  // Concierge serialises an unset username as `null`; an absent key reads the same.
  if (username !== null && username !== undefined && typeof username !== 'string') return null;
  return {
    userId,
    email,
    emailVerified,
    username: typeof username === 'string' ? username : null,
    status,
    role,
    isAdmin,
    roleIsBreakGlass,
  };
}

/** Validate a session body. Unknown fields are ignored; a missing or mistyped known one fails. */
export function parseSession(raw: unknown): ParseResult<Session> {
  if (typeof raw !== 'object' || raw === null) return { ok: false, error: 'session is not an object' };
  const authenticated: unknown = Reflect.get(raw, 'authenticated');
  const permissions: unknown = Reflect.get(raw, 'permissions');
  if (typeof authenticated !== 'boolean') return { ok: false, error: 'session `authenticated` is not a boolean' };
  if (!isStringArray(permissions)) return { ok: false, error: 'session `permissions` is not a string array' };
  if (!authenticated) return { ok: true, value: { authenticated: false, user: null, permissions } };
  const user = parseUser(Reflect.get(raw, 'user'));
  if (!user) return { ok: false, error: 'session `user` is missing or malformed' };
  return { ok: true, value: { authenticated: true, user, permissions } };
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
