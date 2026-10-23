import { useState } from "react";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldLabel,
  Input,
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
  InputGroupText,
  InputOTP,
  InputOTPGroup,
  InputOTPSeparator,
  InputOTPSlot,
  NativeSelect,
  NativeSelectOption,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Textarea,
} from "@evinvest/uikit";
import { CopyIcon, SearchIcon } from "../../gallery/icons";
import { Block, LIVE_STATES, Matrix, PageRoot, Specimen } from "../../gallery/ui";

// fields name their 44 px size `lg`; Button and IconButton name it `touch`
const SIZES = ["sm", "md", "lg"] as const;
const COLUMNS = ["sm", "md", "touch (lg)"] as const;

function InputMatrix() {
  return (
    <Matrix
      columns={COLUMNS}
      rows={[
        { label: "Placeholder", cells: SIZES.map((s) => <Input key={s} size={s} aria-label={`Placeholder ${s}`} placeholder="Search" className="w-36" />) },
        { label: "Filled", cells: SIZES.map((s) => <Input key={s} size={s} aria-label={`Filled ${s}`} defaultValue="2,398.73" className="w-36" />) },
        { label: "Disabled", cells: SIZES.map((s) => <Input key={s} size={s} aria-label={`Disabled ${s}`} defaultValue="Locked" disabled className="w-36" />) },
        { label: "Error", cells: SIZES.map((s) => <Input key={s} size={s} aria-label={`Error ${s}`} defaultValue="12 x" aria-invalid className="w-36" />) },
      ]}
    />
  );
}

function SelectMatrix() {
  const trigger = (size: (typeof SIZES)[number], props: { disabled?: boolean; invalid?: boolean; value?: string }) => (
    <Select defaultValue={props.value}>
      <SelectTrigger size={size} className="w-36" aria-label={`Select ${size}`} aria-invalid={props.invalid} disabled={props.disabled}>
        <SelectValue placeholder="Select…" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value="usd">USD</SelectItem>
        <SelectItem value="eur">EUR</SelectItem>
        <SelectItem value="gbp">GBP</SelectItem>
      </SelectContent>
    </Select>
  );
  return (
    <Matrix
      columns={COLUMNS}
      rows={[
        { label: "Placeholder", cells: SIZES.map((s) => <span key={s}>{trigger(s, {})}</span>) },
        { label: "Filled", cells: SIZES.map((s) => <span key={s}>{trigger(s, { value: "usd" })}</span>) },
        { label: "Disabled", cells: SIZES.map((s) => <span key={s}>{trigger(s, { value: "usd", disabled: true })}</span>) },
        { label: "Error", cells: SIZES.map((s) => <span key={s}>{trigger(s, { invalid: true })}</span>) },
      ]}
    />
  );
}

function OtpDemo() {
  const [code, setCode] = useState("");
  return (
    <Field className="w-fit">
      <FieldLabel>Verification code</FieldLabel>
      <InputOTP maxLength={6} value={code} onChange={setCode} aria-label="Verification code">
        <InputOTPGroup>
          <InputOTPSlot index={0} />
          <InputOTPSlot index={1} />
          <InputOTPSlot index={2} />
        </InputOTPGroup>
        <InputOTPSeparator />
        <InputOTPGroup>
          <InputOTPSlot index={3} />
          <InputOTPSlot index={4} />
          <InputOTPSlot index={5} />
        </InputOTPGroup>
      </InputOTP>
      <FieldDescription>{code.length === 6 ? "Code complete" : `${code.length} of 6 digits`}</FieldDescription>
    </Field>
  );
}

function GroupDemo() {
  return (
    <div className="flex w-full max-w-sm flex-col gap-3">
      <InputGroup>
        <InputGroupAddon>
          <SearchIcon />
        </InputGroupAddon>
        <InputGroupInput aria-label="Search markets" placeholder="Search markets" />
      </InputGroup>
      <InputGroup>
        <InputGroupAddon>
          <InputGroupText>https://</InputGroupText>
        </InputGroupAddon>
        <InputGroupInput aria-label="Domain" defaultValue="evinvest.ltd" />
        <InputGroupAddon align="inline-end">
          <InputGroupButton aria-label="Copy address">
            <CopyIcon />
          </InputGroupButton>
        </InputGroupAddon>
      </InputGroup>
    </div>
  );
}

export default function InputsPage() {
  return (
    <PageRoot title="Inputs" lede="Text fields and pickers share one control edge, one fill, and three sizes. Error is marked with aria-invalid, never by colour alone.">
      <Block title="Input" note={LIVE_STATES}>
        <InputMatrix />
      </Block>
      <Block title="Select">
        <SelectMatrix />
      </Block>
      <Block title="Field" note="A label, a control, a hint and an error, linked by the Field that mints the ids.">
        <Specimen>
          <Field className="w-64">
            <FieldLabel>Amount</FieldLabel>
            <Input defaultValue="12 x" aria-invalid />
            <FieldError>Enter a number.</FieldError>
          </Field>
          <Field className="w-64">
            <FieldLabel>Notes</FieldLabel>
            <Textarea placeholder="Anything the desk should know" />
            <FieldDescription>Visible to your account manager.</FieldDescription>
          </Field>
        </Specimen>
      </Block>
      <Block title="Textarea">
        <Matrix
          columns={COLUMNS}
          rows={[
            { label: "Placeholder", cells: SIZES.map((s) => <Textarea key={s} size={s} aria-label={`Placeholder ${s}`} placeholder="Notes" className="w-36" />) },
            { label: "Disabled", cells: SIZES.map((s) => <Textarea key={s} size={s} aria-label={`Disabled ${s}`} defaultValue="Locked" disabled className="w-36" />) },
            { label: "Error", cells: SIZES.map((s) => <Textarea key={s} size={s} aria-label={`Error ${s}`} defaultValue="Too short" aria-invalid className="w-36" />) },
          ]}
        />
      </Block>
      <Block title="InputOTP">
        <Specimen>
          <OtpDemo />
        </Specimen>
      </Block>
      <Block title="InputGroup">
        <Specimen>
          <GroupDemo />
        </Specimen>
      </Block>
      <Block title="Native select" note="For server-rendered forms that must post before any script runs.">
        <Specimen>
          <Field className="w-48">
            <FieldLabel>Service</FieldLabel>
            <NativeSelect name="service" placeholder="Pick one">
              <NativeSelectOption value="leak">Leak</NativeSelectOption>
              <NativeSelectOption value="boiler">Boiler</NativeSelectOption>
            </NativeSelect>
          </Field>
        </Specimen>
      </Block>
    </PageRoot>
  );
}
