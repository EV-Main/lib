import { Caption, PageRoot } from "../../gallery/ui";
import { HEADER_CAPTIONS, SmartHeaderDemo } from "../parts/header-parts";
import type { SmartHeaderState } from "@evinvest/uikit";

const ORDER: readonly SmartHeaderState[] = ["default", "collapsed", "banner", "search"];

export default function SmartHeaderScreen() {
  return (
    <PageRoot title="Smart header" lede="The four variants, stacked, as the design shows them. The sample content is a trading terminal; the structure is what the kit ships.">
      {ORDER.map((state) => (
        <section key={state} className="flex min-w-0 flex-col gap-2">
          <Caption>{HEADER_CAPTIONS[state]}</Caption>
          <div className="overflow-x-auto">
            <div className="min-w-[760px]">
              <SmartHeaderDemo state={state} />
            </div>
          </div>
        </section>
      ))}
    </PageRoot>
  );
}
