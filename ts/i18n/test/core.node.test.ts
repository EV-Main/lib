import { readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it, vi } from "vitest";

import {
  DEFAULT_LOCALE,
  LOCALES,
  LOCALE_LABELS,
  formatMessage,
  isLocale,
  localeOfElement,
  pickMessages,
  translator,
  type Locale,
  type MessageValues,
  type Messages,
} from "../src/index";

describe("registry", () => {
  it("uses the ISO 639-1 language code for Vietnamese, not the country code", () => {
    // The chat thread proposed `vn`, which is the ISO 3166 country code. Google
    // discards invalid hreflang values silently, so this is worth pinning.
    expect(LOCALES).toContain("vi");
    expect(LOCALES as readonly string[]).not.toContain("vn");
  });

  it("puts the default locale first", () => {
    expect(LOCALES[0]).toBe(DEFAULT_LOCALE);
  });

  it("labels every locale in its own language", () => {
    for (const locale of LOCALES) {
      expect(LOCALE_LABELS[locale]).toBeTruthy();
    }
    expect(LOCALE_LABELS.ru).toBe("Русский");
  });

  it("guards untrusted input of any type", () => {
    expect(isLocale(undefined)).toBe(false);
    expect(isLocale(42)).toBe(false);
  });
});

// Shared with ev_lib::i18n (rust/tests/i18n.rs).
const VECTORS = JSON.parse(readFileSync(join(import.meta.dirname, "../../../tests/fixtures/i18n/format.json"), "utf8")) as {
  format: [string, Locale, MessageValues, string][];
  plural: [Locale, number, string][];
  number: [Locale, number, string][];
  translate: [Locale, Messages, string, string, MessageValues, string][];
};

describe("shared vectors", () => {
  it.each(VECTORS.format)("formats %j in %s with %j as %j", (pattern, locale, values, expected) => {
    expect(formatMessage(pattern, locale, values)).toBe(expected);
  });

  it.each(VECTORS.plural)("%s selects the category of %d as %s", (locale, n, category) => {
    const every = "{n, plural, zero {zero} one {one} two {two} few {few} many {many} other {other}}";
    expect(formatMessage(every, locale, { n })).toBe(category);
  });

  it.each(VECTORS.number)("%s writes %d as %j", (locale, n, written) => {
    expect(formatMessage("{n, plural, other {#}}", locale, { n })).toBe(written);
  });

  it.each(VECTORS.translate)("translates in %s from %j: %s", (locale, messages, key, en, values, expected) => {
    expect(translator(messages, locale)(key, en, values)).toBe(expected);
  });
});

describe("translator — onMissing", () => {
  const messages = { "hero.title": "Инвестируйте в нарратив «Китай+1»" };

  it("reports a key the catalogue lacks", () => {
    const onMissing = vi.fn();
    translator(messages, "ru", onMissing)("hero.subtitle", "Five hundred years of compounding");
    expect(onMissing).toHaveBeenCalledWith("hero.subtitle", "ru");
  });

  it("does not report a key that exists", () => {
    const onMissing = vi.fn();
    translator(messages, "ru", onMissing)("hero.title", "Invest in the China+1 narrative");
    expect(onMissing).not.toHaveBeenCalled();
  });

  it("never reports for the default locale, which has no catalogue to miss", () => {
    const onMissing = vi.fn();
    translator({}, "en", onMissing)("anything", "Anything");
    expect(onMissing).not.toHaveBeenCalled();
  });
});

describe("localeOfElement", () => {
  // The structural stand-in for a DOM subtree: `closest` walks up to the
  // nearest ancestor carrying `lang`, exactly as the platform's does.
  const mount = (...langs: (string | null)[]): Parameters<typeof localeOfElement>[0] => {
    const node = (i: number): Parameters<typeof localeOfElement>[0] => ({
      getAttribute: name => (name === "lang" ? (langs[i] ?? null) : null),
      closest: () => {
        for (let up = i; up < langs.length; up++) if (langs[up] != null) return node(up);
        return null;
      },
    });
    return node(0);
  };

  it("reads the nearest lang", () => {
    expect(localeOfElement(mount("ru"))).toBe("ru");
    expect(localeOfElement(mount(null, null, "de"))).toBe("de");
  });

  it("takes the base language of a regional tag", () => {
    expect(localeOfElement(mount("ru-RU"))).toBe("ru");
  });

  it("falls back to the default when there is no lang, or an unpublished one", () => {
    expect(localeOfElement(mount(null))).toBe("en");
    expect(localeOfElement(mount("ja"))).toBe("en");
  });
});

describe("pickMessages", () => {
  const CATALOGUE = { "nav.home": "Главная", "nav.wallet": "Кошелёк", "hero.title": "Заголовок" };

  it("keeps only the requested keys", () => {
    expect(pickMessages(CATALOGUE, ["nav.home", "hero.title"])).toEqual({
      "nav.home": "Главная",
      "hero.title": "Заголовок",
    });
  });

  it("leaves out a key the catalogue lacks instead of inventing one", () => {
    const picked = pickMessages(CATALOGUE, ["nav.home", "nav.gone"]);

    expect(picked).toEqual({ "nav.home": "Главная" });
    expect(Object.hasOwn(picked, "nav.gone")).toBe(false);
  });

  it("does not pick inherited members such as toString or __proto__", () => {
    const picked = pickMessages(CATALOGUE, ["toString", "__proto__", "constructor"]);

    expect(Object.keys(picked)).toEqual([]);
    expect(Object.getPrototypeOf(picked)).toBe(Object.prototype);
  });

  it("keeps an own __proto__ key as data, not as the prototype", () => {
    // JSON.parse makes `__proto__` an own key; an assignment would set the prototype instead.
    const catalogue = JSON.parse('{"__proto__": "прото", "a": "А"}') as Record<string, string>;

    const picked = pickMessages(catalogue, ["__proto__", "a"]);

    expect(Object.getPrototypeOf(picked)).toBe(Object.prototype);
    expect(Object.keys(picked)).toEqual(["__proto__", "a"]);
    expect(Object.getOwnPropertyDescriptor(picked, "__proto__")?.value).toBe("прото");
  });

  it("returns a plain object an RSC payload can serialise", () => {
    const picked = pickMessages(Object.assign(Object.create(null) as Record<string, string>, CATALOGUE), [
      "nav.home",
    ]);

    expect(Object.getPrototypeOf(picked)).toBe(Object.prototype);
    expect(picked).toEqual({ "nav.home": "Главная" });
  });

  it("accepts any iterable of keys", () => {
    expect(pickMessages(CATALOGUE, new Set(["nav.wallet"]))).toEqual({ "nav.wallet": "Кошелёк" });
  });
});
