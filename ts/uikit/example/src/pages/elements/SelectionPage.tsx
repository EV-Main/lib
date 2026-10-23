import { useState } from "react";
import {
  Checkbox,
  Label,
  RadioGroup,
  RadioGroupItem,
  Slider,
  Switch,
  Toggle,
  ToggleGroup,
  ToggleGroupItem,
} from "@evinvest/uikit";
import { BoldIcon, ItalicIcon } from "../../gallery/icons";
import { Block, Caption, LIVE_STATES, Matrix, PageRoot, Specimen } from "../../gallery/ui";

function Radio({ checked = false, disabled = false, invalid = false }: { checked?: boolean; disabled?: boolean; invalid?: boolean }) {
  return (
    <RadioGroup {...(checked ? { defaultValue: "a" } : {})} aria-label="Radio">
      <RadioGroupItem value="a" disabled={disabled} aria-invalid={invalid} aria-label="Option" />
    </RadioGroup>
  );
}

function ControlMatrix() {
  return (
    <Matrix
      columns={["Checkbox", "Radio", "Switch"]}
      rows={[
        {
          label: "Off",
          cells: [<Checkbox key="c" aria-label="Off" />, <Radio key="r" />, <Switch key="s" aria-label="Off" />],
        },
        {
          label: "On",
          cells: [<Checkbox key="c" aria-label="On" defaultChecked />, <Radio key="r" checked />, <Switch key="s" aria-label="On" defaultChecked />],
        },
        {
          label: "Disabled off",
          cells: [<Checkbox key="c" aria-label="Disabled off" disabled />, <Radio key="r" disabled />, <Switch key="s" aria-label="Disabled off" disabled />],
        },
        {
          label: "Disabled on",
          cells: [
            <Checkbox key="c" aria-label="Disabled on" defaultChecked disabled />,
            <Radio key="r" checked disabled />,
            <Switch key="s" aria-label="Disabled on" defaultChecked disabled />,
          ],
        },
        {
          label: "Error",
          cells: [<Checkbox key="c" aria-label="Error" aria-invalid />, <Radio key="r" invalid />, <Caption key="s">No error state</Caption>],
        },
      ]}
    />
  );
}

function Labelled() {
  const [wifi, setWifi] = useState(true);
  const [agree, setAgree] = useState(false);
  return (
    <Specimen>
      <div className="flex items-center gap-2">
        <Switch id="wifi" checked={wifi} onCheckedChange={setWifi} />
        <Label htmlFor="wifi">Wi-Fi {wifi ? "on" : "off"}</Label>
      </div>
      <div className="flex items-center gap-2">
        <Checkbox id="agree" checked={agree} onCheckedChange={setAgree} />
        <Label htmlFor="agree">I agree</Label>
      </div>
      <RadioGroup defaultValue="b" aria-label="Plan" className="flex-row gap-4">
        {["a", "b", "c"].map((v) => (
          <div key={v} className="flex items-center gap-2">
            <RadioGroupItem id={`plan-${v}`} value={v} />
            <Label htmlFor={`plan-${v}`}>Plan {v.toUpperCase()}</Label>
          </div>
        ))}
      </RadioGroup>
    </Specimen>
  );
}

function Toggles() {
  return (
    <Specimen>
      <Toggle aria-label="Bold" defaultPressed>
        <BoldIcon />
      </Toggle>
      <Toggle aria-label="Italic">
        <ItalicIcon />
      </Toggle>
      <Toggle aria-label="Disabled" disabled>
        <ItalicIcon />
      </Toggle>
      <ToggleGroup type="multiple" aria-label="Days" defaultValue={["mon", "tue", "wed", "thu", "fri"]}>
        {["mon", "tue", "wed", "thu", "fri", "sat", "sun"].map((day) => (
          <ToggleGroupItem key={day} value={day} aria-label={day}>
            {day.slice(0, 2)}
          </ToggleGroupItem>
        ))}
      </ToggleGroup>
      <ToggleGroup type="single" aria-label="Range" defaultValue="1m">
        {["1d", "1w", "1m", "1y"].map((r) => (
          <ToggleGroupItem key={r} value={r}>
            {r}
          </ToggleGroupItem>
        ))}
      </ToggleGroup>
    </Specimen>
  );
}

function Sliders() {
  const [value, setValue] = useState(40);
  return (
    <Specimen>
      <div className="flex w-64 flex-col gap-2">
        <Slider aria-label="Allocation" value={value} onValueChange={setValue} />
        <Caption>{value}% allocated</Caption>
      </div>
      <div className="w-64">
        <Slider aria-label="Disabled" defaultValue={60} disabled />
      </div>
    </Specimen>
  );
}

export default function SelectionPage() {
  return (
    <PageRoot title="Selection" lede="Checkbox, radio, switch, toggle and slider. The unchecked edge uses the control-edge token, so it reaches 3:1.">
      <Block title="Checkbox, Radio, Switch" note={LIVE_STATES}>
        <ControlMatrix />
      </Block>
      <Block title="With labels">
        <Labelled />
      </Block>
      <Block title="Toggle and ToggleGroup" note="Pressed is the active fill. A group is one tab stop with arrow keys inside.">
        <Toggles />
      </Block>
      <Block title="Slider">
        <Sliders />
      </Block>
    </PageRoot>
  );
}
