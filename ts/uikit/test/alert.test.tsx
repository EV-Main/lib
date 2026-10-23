import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { Alert, AlertTitle, AlertDescription } from "../src/components/alert";

describe("Alert", () => {
  it("renders the default variant", () => {
    const { getByRole } = render(<Alert>body</Alert>);
    const el = getByRole("alert");
    expect(el.tagName).toBe("DIV");
    expect(el).toHaveClass("bg-card");
    expect(el).toHaveAttribute("data-slot", "alert");
  });

  it.each([
    ["error", "text-accent-error-ink", "border-chip-error-border"],
    ["positive", "text-positive-ink", "border-chip-positive-border"],
    ["warn", "text-accent-warn-ink", "border-chip-warn-border"],
    ["info", "text-accent-info-ink", "border-chip-info-border"],
  ] as const)("renders the %s tone in its ink and chip edge", (variant, ink, edge) => {
    const { getByRole } = render(<Alert variant={variant}>x</Alert>);
    expect(getByRole("alert")).toHaveClass(ink, edge);
  });

  // the role tint stays on the root (icon + title inherit it); the description
  // child is lifted to ink so body copy and nested controls hold the contrast floor
  it.each(["error", "positive", "warn", "info"] as const)(
    "keeps the %s description in ink",
    (variant) => {
      const { getByRole } = render(
        <Alert variant={variant}>
          <AlertDescription>d</AlertDescription>
        </Alert>,
      );
      const el = getByRole("alert");
      expect(el).toHaveClass("*:data-[slot=alert-description]:text-ink-mid");
      expect(el.className).not.toMatch(/\/90/);
    },
  );

  it("renders title and description slots", () => {
    const { getByText } = render(
      <Alert>
        <AlertTitle>t</AlertTitle>
        <AlertDescription>d</AlertDescription>
      </Alert>,
    );
    expect(getByText("t")).toHaveAttribute("data-slot", "alert-title");
    const desc = getByText("d");
    expect(desc).toHaveAttribute("data-slot", "alert-description");
    expect(desc).toHaveClass("text-ink-soft");
  });

  it("lets className override the base", () => {
    const { getByRole } = render(<Alert className="px-6">x</Alert>);
    const el = getByRole("alert");
    expect(el).toHaveClass("px-6");
    expect(el).not.toHaveClass("px-4");
  });
});
