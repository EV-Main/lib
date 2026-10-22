import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { FormRow } from "../src/components/form-row";

describe("FormRow", () => {
  it("labels the control it wraps when given htmlFor", () => {
    render(
      <FormRow label="Email" htmlFor="email">
        <input id="email" />
      </FormRow>,
    );
    expect(screen.getByLabelText("Email")).toBeInTheDocument();
  });

  it("renders description under the label and hint under the control", () => {
    const { container } = render(
      <FormRow label="Name" description="Shown on invoices" hint="Max 40 characters">
        <span>JD</span>
      </FormRow>,
    );
    expect(container.querySelector('[data-slot="form-row-label"]')).toHaveTextContent("Shown on invoices");
    expect(container.querySelector('[data-slot="form-row-control"]')).toHaveTextContent("Max 40 characters");
  });

  it("draws a divider by default and drops it on request", () => {
    const { rerender, container } = render(<FormRow label="A">x</FormRow>);
    expect(container.firstChild).toHaveClass("border-b");
    rerender(
      <FormRow label="A" divider={false}>
        x
      </FormRow>,
    );
    expect(container.firstChild).not.toHaveClass("border-b");
  });
});
