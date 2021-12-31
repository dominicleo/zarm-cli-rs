import * as React from "react";

export type CSSVariables<T extends object = never> = React.CSSProperties &
  Partial<T>;

export type MountContainer = HTMLElement | (() => HTMLElement);
export type TargetContainer = HTMLElement | (() => HTMLElement);
