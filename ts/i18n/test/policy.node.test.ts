import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import type { Locale, Messages } from "../src/index.js";
import {
  auditCatalogues,
  availableIn,
  resolveCatalogue,
  type MissingContentPolicy,
  type TranslatedCatalogue,
} from "../src/policy/index.js";

type Catalogues = [Locale, Messages, TranslatedCatalogue];

// Shared with ev_lib::i18n::policy (rust/tests/i18n.rs).
const VECTORS = JSON.parse(readFileSync(join(import.meta.dirname, "../../../tests/fixtures/i18n/policy.json"), "utf8")) as {
  resolve: {
    locale: Locale;
    source: Messages;
    translated: TranslatedCatalogue;
    messages: Messages;
    rejected: [string, string, string][];
    missing: string[];
    coverage: number;
  }[];
  audit: { catalogues: Catalogues[]; floor: number; ok: boolean; report: string }[];
  available: [Locale, [string, Locale[]][], MissingContentPolicy, string[]][];
};

describe("shared vectors", () => {
  it.each(VECTORS.resolve)("resolves $locale over $source", ({ locale, source, translated, ...expected }) => {
    const r = resolveCatalogue(locale, source, translated);
    expect({
      messages: r.messages,
      rejected: r.rejected.map(({ key, reason, detail }) => [key, reason, detail]),
      missing: r.missing,
      coverage: r.coverage,
    }).toEqual(expected);
  });

  it.each(VECTORS.audit)("audits at floor $floor: $report", ({ catalogues, floor, ok, report }) => {
    const resolved = catalogues.map(([locale, source, translated]) => resolveCatalogue(locale, source, translated));
    expect(auditCatalogues(resolved, floor)).toEqual({ ok, report });
  });

  it.each(VECTORS.available)("shows %s, of %j under %s, %j", (locale, items, policy, shown) => {
    expect(availableIn(locale, items, ([, locales]) => locales, policy).map(([name]) => name)).toEqual(shown);
  });
});

describe("rule 1.3 — untranslated compiled content is hidden", () => {
  it("treats an item that declares no locales as untranslated", () => {
    const undeclared = [{ slug: "c" }];
    expect(availableIn("ru", undeclared, () => undefined)).toEqual([]);
  });
});
