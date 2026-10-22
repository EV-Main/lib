import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { SystemBanner } from "../src/components/system-banner";
import { SectionLabel } from "../src/components/page-frame";

describe("SystemBanner tones", () => {
  it("keeps the strip on the card fill whatever the tone and colours only the icon", () => {
    const { container } = render(
      <SystemBanner tone="warn" icon={<svg data-testid="glyph" />}>
        Maintenance tonight
      </SystemBanner>,
    );
    expect(screen.getByRole("status")).toHaveClass("bg-card", "border-b");
    expect(screen.getByRole("status")).not.toHaveClass("bg-accent-warn/10");
    expect(screen.getByTestId("glyph").parentElement).toHaveClass("text-accent-warn-ink");
    expect(container.querySelector('[data-tone="warn"]')).not.toBeNull();
  });

  it.each([
    ["neutral", "text-ink-soft"],
    ["primary", "text-primary-ink"],
    ["info", "text-accent-info-ink"],
  ] as const)("tints the %s icon", (tone, cls) => {
    render(
      <SystemBanner tone={tone} icon={<svg data-testid="glyph" />}>
        Hello
      </SystemBanner>,
    );
    expect(screen.getByTestId("glyph").parentElement).toHaveClass(cls);
  });

  it("renders an action after the message", () => {
    render(<SystemBanner action={<a href="#d">Details</a>}>Hello</SystemBanner>);
    expect(screen.getByRole("link", { name: "Details" })).toBeInTheDocument();
  });
});

describe("SectionLabel", () => {
  it("is plain body-s text, not tracked uppercase", () => {
    render(<SectionLabel>General</SectionLabel>);
    const el = screen.getByText("General");
    expect(el).toHaveClass("text-body-s", "text-ink-soft");
    expect(el).not.toHaveClass("uppercase", "tracking-widest");
  });
});
