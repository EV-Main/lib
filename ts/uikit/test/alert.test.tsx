import { describe, it, expect } from "vitest";
import { render } from "@testing-library/react";
import { Alert, AlertTitle, AlertDescription } from "../src/components/atomic/alert";
import { toneClasses } from "../src/generated/tone";

describe("Alert", () => {
  it("renders the neutral tone by default", () => {
    const { getByRole } = render(<Alert>body</Alert>);
    const el = getByRole("alert");
    expect(el.tagName).toBe("DIV");
    expect(el).toHaveClass(toneClasses.neutral);
    expect(el).toHaveAttribute("data-slot", "alert");
  });

  it.each(["error", "positive", "info"] as const)("fuses the %s tone onto the root", (tone) => {
    const { getByRole } = render(<Alert tone={tone}>x</Alert>);
    expect(getByRole("alert")).toHaveClass(toneClasses[tone]);
  });

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
    expect(desc).toHaveClass("text-ink-mid");
  });

  it("lets className override the base", () => {
    const { getByRole } = render(<Alert className="px-6">x</Alert>);
    const el = getByRole("alert");
    expect(el).toHaveClass("px-6");
    expect(el).not.toHaveClass("px-4");
  });
});
