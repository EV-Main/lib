import { useState } from "react";
import {
  Button,
  Checkbox,
  Field,
  FieldLabel,
  Label,
  PopoverForm,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  toast,
} from "@evinvest/uikit";
import { Block, Caption, PageRoot, Specimen } from "../../gallery/ui";

export default function PopoverFormPage() {
  const [applied, setApplied] = useState("Status: All");
  return (
    <PageRoot title="Popover form" lede="A small form in a popover: a title, fields, and Reset and Apply. Submit closes it; Reset clears the native fields.">
      <Block title="Filter" note="Focus moves in on open, Tab stays inside, and the focus returns to the trigger on close.">
        <Specimen>
          <PopoverForm
            trigger={<Button variant="secondary">Filter</Button>}
            title="Filter"
            onSubmit={(e) => {
              e.preventDefault();
              setApplied("Filter applied");
              toast("Filter applied");
            }}
            onReset={() => setApplied("Status: All")}
          >
            <Field>
              <FieldLabel>Status</FieldLabel>
              <Select defaultValue="all">
                <SelectTrigger className="w-full" aria-label="Status">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="all">All</SelectItem>
                  <SelectItem value="open">Open</SelectItem>
                  <SelectItem value="closed">Closed</SelectItem>
                </SelectContent>
              </Select>
            </Field>
            <div className="flex items-center gap-2">
              <Checkbox id="pf-meta" defaultChecked />
              <Label htmlFor="pf-meta">Show metadata</Label>
            </div>
            <div className="flex items-center gap-2">
              <Checkbox id="pf-fav" />
              <Label htmlFor="pf-fav">Only favorites</Label>
            </div>
          </PopoverForm>
          <Caption>{applied}</Caption>
        </Specimen>
      </Block>
    </PageRoot>
  );
}
