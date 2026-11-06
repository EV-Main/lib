import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { Badge } from "../src/components/atomic/badge";
import { toneClasses } from "../src/generated/tone";

describe("Badge", () => {
  it("renders the neutral tone by default", () => {
    const { getByText } = render(<Badge>hi</Badge>);
    const el = getByText("hi");
    expect(el.tagName).toBe("SPAN");
    expect(el).toHaveClass(toneClasses.neutral, "h-5");
    expect(el).toHaveAttribute("data-slot", "badge");
  });

  it.each(["primary", "positive", "info", "warn", "error"] as const)("fuses the %s tone onto the chip", (tone) => {
    const { getByText } = render(<Badge tone={tone}>ok</Badge>);
    expect(getByText("ok")).toHaveClass(toneClasses[tone], "text-caption");
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
    expect(el).toHaveClass(toneClasses.neutral);
  });
});
