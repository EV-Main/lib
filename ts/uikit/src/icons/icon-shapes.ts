import * as React from "react";
import { ICONS, type IconName } from "../generated/icons";

type Shape = readonly [tag: string, attributes: Readonly<Record<string, string>>];

/** An icon's children, for the `<svg>` frame a site draws around them. */
export function iconShapes(name: IconName): React.ReactNode {
  const shapes: readonly Shape[] = ICONS[name];
  return shapes.map(([tag, attributes], i) => React.createElement(tag, { key: i, ...attributes }));
}
