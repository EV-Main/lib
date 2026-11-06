import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { DATA_SLOTS } from "../src/generated/slot";
import { TS_ONLY_SLOTS } from "../src/ts-only-slots";

// `data-slot` names are Rust's (`ev_lib_classes::Slot`); `src/data-slot.d.ts`
// makes `tsc` hold every attribute literal to them. This holds the rest:
// TS-only names stay TS-only, and selectors name slots that exist.
const SRC = join(import.meta.dirname, "../src");

const sources = (dir: string): string[] =>
  readdirSync(dir, { withFileTypes: true }).flatMap(e => {
    const path = join(dir, e.name);
    return e.isDirectory() ? sources(path) : /\.tsx?$/.test(e.name) ? [path] : [];
  });

describe("data-slot names", () => {
  it("keeps TS-only names disjoint from Rust's", () => {
    const rust = new Set<string>(DATA_SLOTS);
    expect(TS_ONLY_SLOTS.filter(s => rust.has(s))).toEqual([]);
  });

  it("selects only slots that exist", () => {
    const known = new Set<string>([...DATA_SLOTS, ...TS_ONLY_SLOTS]);
    const unknown = sources(SRC).flatMap(path =>
      [...readFileSync(path, "utf8").matchAll(/\[data-slot=["\x27]?([a-z0-9-]+)/g)]
        .map(([, name = ""]) => name)
        .filter(name => !known.has(name))
        .map(name => `${path.slice(SRC.length + 1)}: ${name}`),
    );
    expect(unknown).toEqual([]);
  });
});
