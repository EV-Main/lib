import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { Badge } from "../src/components/badge";

describe("Badge", () => {
  it("renders the default variant", () => {
    const { getByText } = render(<Badge>hi</Badge>);
    const el = getByText("hi");
    expect(el.tagName).toBe("SPAN");
    expect(el).toHaveClass("bg-chip-neutral-bg", "text-ink-mid", "h-5");
    expect(el).toHaveAttribute("data-slot", "badge");
  });

  it.each([
    ["primary", "text-primary-ink"],
    ["positive", "text-positive-ink"],
    ["info", "text-accent-info-ink"],
    ["warn", "text-accent-warn-ink"],
    ["error", "text-accent-error-ink"],
  ] as const)("renders the %s tone on its chip tokens (canon parity with Rust)", (variant, ink) => {
    const { getByText } = render(<Badge variant={variant}>ok</Badge>);
    expect(getByText("ok")).toHaveClass(`bg-chip-${variant}-bg`, `border-chip-${variant}-border`, ink, "text-caption");
  });

  it("lets className override the base", () => {
    const { getByText } = render(<Badge className="px-6">x</Badge>);
    const el = getByText("x");
    expect(el).toHaveClass("px-6");
    expect(el).not.toHaveClass("px-1.5");
  });

  it("renders as child when asChild", () => {
    const { getByRole } = render(
      <Badge asChild>
        <a href="#">link</a>
      </Badge>,
    );
    const el = getByRole("link");
    expect(el.tagName).toBe("A");
    expect(el).toHaveClass("bg-chip-neutral-bg");
  });
});
