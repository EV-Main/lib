import { useState } from "react";
import { ToggleGroup, ToggleGroupItem, type SmartHeaderState } from "@evinvest/uikit";
import { Block, Caption, PageRoot } from "../../gallery/ui";
import { HEADER_CAPTIONS, SmartHeaderDemo } from "../parts/header-parts";

const STATES: readonly SmartHeaderState[] = ["default", "collapsed", "banner", "search"];
const isState = (v: unknown): v is SmartHeaderState => STATES.some((s) => s === v);

export default function SmartHeaderPage() {
  const [state, setState] = useState<SmartHeaderState>("default");
  return (
    <PageRoot
      title="Smart header"
      lede="One bar whose rows come and go. The brand and the account never move: only the rows around them change. The caller owns the state: a scroll observer, a feed status, the search button."
    >
      <Block title="Switch the state" note={HEADER_CAPTIONS[state]}>
        <div className="flex flex-col gap-3">
          <ToggleGroup
            type="single"
            variant="outline"
            aria-label="Header state"
            value={state}
            onValueChange={(v) => {
              if (typeof v === "string" && isState(v)) setState(v);
            }}
            className="self-start"
          >
            {STATES.map((s) => (
              <ToggleGroupItem key={s} value={s}>
                {s}
              </ToggleGroupItem>
            ))}
          </ToggleGroup>
          <div className="overflow-x-auto">
            <div className="min-w-[760px]">
              <SmartHeaderDemo state={state} />
            </div>
          </div>
        </div>
      </Block>
      <Block title="All four states">
        <div className="flex flex-col gap-6">
          {STATES.map((s) => (
            <div key={s} className="flex min-w-0 flex-col gap-2">
              <Caption>{HEADER_CAPTIONS[s]}</Caption>
              <div className="overflow-x-auto">
                <div className="min-w-[760px]">
                  <SmartHeaderDemo state={s} />
                </div>
              </div>
            </div>
          ))}
        </div>
      </Block>
    </PageRoot>
  );
}
