import "server-only";
import { sitesPanelBrand } from "./env";
import { unreachable, type Down } from "./fetched";

/**
 * An operator's override of one experiment, as the panel serves it — the
 * shape `applyOverrides` of `@evinvest/experiments` takes. Only the types are
 * checked here; whether a field fits the experiment in code (weights of the
 * right length, a holdout in `[0, 1)`) is `applyOverrides`' call, because only
 * it knows the code's variants.
 */
export interface ExperimentOverride {
  readonly enabled?: boolean;
  readonly weights?: readonly number[];
  readonly holdout?: number;
}

export type ExperimentOverrides = Readonly<Record<string, ExperimentOverride>>;

/**
 * The operator's overrides of the brand's experiments: `GET <sites_panel
 * brand>/experiments`, the panel's `{ "experiments": { "<key>": {…} } }`.
 *
 * Read by the proxy on every request, so the answer comes from memory: the
 * first call waits for the panel (bounded by `timeoutMs`), every later one is
 * served from the cache — past the TTL too, while one refresh runs behind it
 * (stale-while-revalidate). The source never throws and never makes a page
 * wait on the panel twice. A failed refresh — an unreachable panel, a
 * non-200, a body that is not the panel's shape — keeps the last good answer:
 * a kill switch must not turn itself back on because the panel blinked. With
 * no good answer yet (or no `SITES_PANEL_INTERNAL_URL`) it is `{}`, the config in code.
 */
export interface ExperimentsSourceOptions {
  /** How long an answer is fresh; 30 s. */
  ttlMs?: number;
  /** How long the panel is given; 1.5 s. The proxy is waiting on the first call. */
  timeoutMs?: number;
  now?: () => number;
  log?: Pick<Console, "error">;
}

export interface ExperimentsSource {
  /** The overrides to apply now; `{}` when there are none or the panel cannot say. Never throws. */
  overrides(): Promise<ExperimentOverrides>;
}

export const EXPERIMENTS_TTL_MS = 30_000;
const TIMEOUT_MS = 1_500;
const NONE: ExperimentOverrides = Object.freeze({});

const isRecord = (v: unknown): v is Record<string, unknown> => typeof v === "object" && v !== null && !Array.isArray(v);

/**
 * The panel's answer as overrides, or `null` when it is not the panel's shape.
 * A field of the wrong type is dropped, an entry that is not an object is
 * skipped; the rest stands.
 */
export function parseExperimentOverrides(body: unknown): ExperimentOverrides | null {
  if (!isRecord(body) || !isRecord(body["experiments"])) return null;
  const out: Record<string, ExperimentOverride> = {};
  for (const [key, entry] of Object.entries(body["experiments"])) {
    if (!isRecord(entry)) continue;
    const { enabled, weights, holdout } = entry;
    out[key] = {
      ...(typeof enabled === "boolean" ? { enabled } : {}),
      ...(Array.isArray(weights) && weights.every((w): w is number => typeof w === "number") ? { weights } : {}),
      ...(typeof holdout === "number" ? { holdout } : {}),
    };
  }
  return out;
}

type FetchedExperiments = { kind: "live" | "empty"; overrides: ExperimentOverrides } | Down;

/** The overrides as the panel answers them now; shared with the site report. */
export async function fetchExperiments(brand: string, timeoutMs: number): Promise<FetchedExperiments> {
  let response: Response;
  try {
    // `no-store`: the cache is the caller's; Next's data cache would add a second TTL.
    response = await fetch(`${brand}/experiments`, { cache: "no-store", headers: { Accept: "application/json" }, signal: AbortSignal.timeout(timeoutMs) });
  } catch (cause) {
    return unreachable(cause);
  }
  if (response.status !== 200) return { kind: "down", detail: `answered ${response.status}` };
  let body: unknown;
  try {
    body = await response.json();
  } catch (cause) {
    return { kind: "down", detail: "not JSON", cause };
  }
  const overrides = parseExperimentOverrides(body);
  if (overrides === null) return { kind: "down", detail: "not { experiments: {…} }" };
  return { kind: Object.keys(overrides).length === 0 ? "empty" : "live", overrides };
}

export function createExperimentsSource(site: { brand: { id: string } }, options: ExperimentsSourceOptions = {}): ExperimentsSource {
  const ttl = options.ttlMs ?? EXPERIMENTS_TTL_MS;
  const timeoutMs = options.timeoutMs ?? TIMEOUT_MS;
  const now = options.now ?? Date.now;
  const log = options.log ?? console;
  let brand: string | null | undefined;
  let cached: { at: number; value: ExperimentOverrides } | null = null;
  let inflight: Promise<ExperimentOverrides> | null = null;

  // One request at a time, however many visitors arrive while it runs.
  function refresh(base: string): Promise<ExperimentOverrides> {
    inflight ??= fetchExperiments(base, timeoutMs)
      .then(fetched => {
        if (fetched.kind === "down") log.error(`experiments: the panel ${fetched.detail}, keeping the last good overrides`, ...(fetched.cause === undefined ? [] : [fetched.cause]));
        // A failure still restarts the clock, so a down panel is asked once per TTL.
        const value = fetched.kind === "down" ? (cached?.value ?? NONE) : fetched.overrides;
        cached = { at: now(), value };
        return value;
      })
      .finally(() => {
        inflight = null;
      });
    return inflight;
  }

  return {
    async overrides() {
      if (brand === undefined) brand = sitesPanelBrand(site, process.env);
      if (brand === null) return NONE;
      if (cached === null) return refresh(brand);
      if (now() - cached.at >= ttl) void refresh(brand);
      return cached.value;
    },
  };
}
