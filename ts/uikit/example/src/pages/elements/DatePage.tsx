import { useState } from "react";
import { Calendar, DateTimePicker, Label } from "@evinvest/uikit";
import { Block, PageRoot, Specimen } from "../../gallery/ui";

function CalendarDemo() {
  const [day, setDay] = useState<Date | undefined>(undefined);
  return (
    <div className="flex flex-col gap-2">
      <Calendar {...(day ? { selected: day } : {})} onSelect={setDay} />
      <span className="text-body-s text-ink-soft">{day ? day.toDateString() : "nothing picked yet"}</span>
    </div>
  );
}

function PickerDemo() {
  const [moment, setMoment] = useState<Date | null>(null);
  return (
    <div className="flex w-72 flex-col gap-2">
      <Label htmlFor="moment">Starts at</Label>
      <DateTimePicker id="moment" value={moment} onChange={setMoment} locale="en-GB" min={new Date()} placeholder="Pick a moment" />
      <span className="text-body-s text-ink-soft">{moment ? moment.toISOString() : "nothing picked yet"}</span>
    </div>
  );
}

export default function DatePage() {
  return (
    <PageRoot title="Dates" lede="A calendar for a day, and a picker that adds the time. Both are keyboard-complete and bounded by min and max.">
      <Block title="Calendar">
        <Specimen>
          <CalendarDemo />
        </Specimen>
      </Block>
      <Block title="DateTimePicker" note="Controlled; bounded below by now.">
        <Specimen className="min-h-64 items-start">
          <PickerDemo />
        </Specimen>
      </Block>
    </PageRoot>
  );
}
