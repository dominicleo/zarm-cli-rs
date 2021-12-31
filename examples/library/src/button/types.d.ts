import type * as React from "react";
import type { CSSVariables } from "../utils/types";

export type ButtonCSSVariables = CSSVariables<{
  /** 按钮颜色 */
  "--color": React.CSSProperties["color"];
  /** 按钮文字大小 */
  "--font-size": React.CSSProperties["fontSize"];
  /** 按钮文字颜色 */
  "--text-color": React.CSSProperties["color"];
  /** 按钮边框颜色 */
  "--border-color": React.CSSProperties["borderColor"];
  /** 按钮边框颜色 */
  "--background-color": React.CSSProperties["backgroundColor"];
}>;

export interface ButtonProps extends React.HTMLAttributes<HTMLButtonElement> {
  style?: ButtonCSSVariables;
  /**
   * 按钮类型
   * @default 'default'
   */
  type?: "default" | "primary" | "info" | "success" | "error";
  /**
   * 按钮禁用状态
   */
  disabled?: boolean;
}
