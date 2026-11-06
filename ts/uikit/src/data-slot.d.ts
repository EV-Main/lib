import type { DataSlot } from "./generated/slot";
import type { TsOnlySlot } from "./ts-only-slots";

declare module "react" {
  interface HTMLAttributes<T> {
    "data-slot"?: DataSlot | TsOnlySlot;
  }
  interface SVGAttributes<T> {
    "data-slot"?: DataSlot | TsOnlySlot;
  }
}
