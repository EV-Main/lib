import * as React from "react";
import { describe, expect, it } from "vitest";
import { render } from "@testing-library/react";
import { isLabelable } from "../src/primitives/labelable";
import { Button } from "../src/components/atomic/button";
import { Checkbox } from "../src/components/atomic/checkbox";
import { Field, FieldLabel } from "../src/components/derived/field";
import { Input } from "../src/components/atomic/input";
import { Label } from "../src/components/atomic/label";
import { NativeSelect } from "../src/components/atomic/native-select";
import { Select, SelectContent, SelectTrigger, SelectValue } from "../src/components/atomic/select";
import { Switch } from "../src/components/atomic/switch";
import { Textarea } from "../src/components/atomic/textarea";

// Each control marks itself in its own module; `FieldLabel` reads the mark to
// tell a label that wraps its control (no `for`) from one beside it.
describe("isLabelable", () => {
  it.each([
    ["Input", Input],
    ["Textarea", Textarea],
    ["NativeSelect", NativeSelect],
    ["Checkbox", Checkbox],
    ["Switch", Switch],
    ["SelectTrigger", SelectTrigger],
    ["Field", Field],
  ])("is true for the kit's %s", (_name, component) => {
    expect(isLabelable(component)).toBe(true);
  });

  it.each([
    ["Label", Label],
    ["Button", Button],
    ["SelectContent", SelectContent],
    ["a component of the caller's", function Mine() {
      return null;
    }],
    ["an object component of the caller's", React.memo(() => null)],
    ["null", null],
    ["undefined", undefined],
    ["a host tag", "input"],
  ])("is false for %s", (_name, type) => {
    expect(isLabelable(type)).toBe(false);
  });
});

describe("FieldLabel around a kit control", () => {
  it.each([
    ["Input", <Input />],
    ["Textarea", <Textarea />],
    ["NativeSelect", <NativeSelect />],
    ["Checkbox", <Checkbox />],
    ["Switch", <Switch />],
    [
      "SelectTrigger",
      <Select>
        <SelectTrigger>
          <SelectValue />
        </SelectTrigger>
      </Select>,
    ],
    [
      "Field",
      <Field orientation="horizontal">
        <Checkbox />
      </Field>,
    ],
  ])("gives the label no for when it wraps a %s", (_name, control) => {
    const { container } = render(
      <Field controlId="plan">
        <FieldLabel>
          {control} Plan
        </FieldLabel>
      </Field>,
    );
    expect(container.querySelector("label")).not.toHaveAttribute("for");
  });

  it("points the label at the Field's id when it wraps a component without the mark", () => {
    const Mine = () => <input />;
    const { container } = render(
      <Field controlId="plan">
        <FieldLabel>
          <Mine /> Plan
        </FieldLabel>
      </Field>,
    );
    expect(container.querySelector("label")).toHaveAttribute("for", "plan");
  });
});
