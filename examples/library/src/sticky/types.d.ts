import type * as React from "react";
import type { CSSVariables } from "../utils/types";

export type StickyCSSVariables = CSSVariables<{
  /** 顶部距离 */
  "--top": React.CSSProperties["top"];
}>;

export interface StickyProps extends React.HTMLAttributes<HTMLButtonElement> {
  style?: StickyCSSVariables;
}
