import { describe, expect, it } from "vitest";

import { cn } from "../src/lib/cn";

describe("cn", () => {
  it("lets the rightmost conflicting utility win", () => {
    expect(cn("p-4", "p-2")).toBe("p-2");
  });

  it("files the kit's type scale under font size, not text colour", () => {
    expect(cn("text-body", "text-ink")).toBe("text-body text-ink");
    expect(cn("text-body-s text-ink-mid", "text-body-l")).toBe("text-ink-mid text-body-l");
    expect(cn("text-figure-m", "text-sm")).toBe("text-sm");
  });
});
