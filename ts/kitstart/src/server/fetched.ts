import "server-only";

/** A source that could not answer, and why — what a page logs and the site report shows. */
export type Down = { kind: "down"; detail: string; cause?: unknown };

export const unreachable = (cause: unknown): Down => ({ kind: "down", detail: `unreachable (${cause instanceof Error ? cause.message : String(cause)})`, cause });

export const isEmptyObject = (v: unknown): boolean => typeof v === "object" && v !== null && !Array.isArray(v) && Object.keys(v).length === 0;
