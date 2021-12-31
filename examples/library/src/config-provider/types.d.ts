import type { ValidateMessages } from "rc-field-form/lib/interface";
import type { MountContainer, TargetContainer } from "../utils/types";

export type { ValidateMessages };
export interface ConfigProviderProps {
  /**
   * 弹出框（Popup, Modal, Toast 等等）渲染父节点，默认渲染到 body 上。
   * @default () => document.body
   */
  mountContainer?: MountContainer;
  /**
   * 滚动监听容器。
   * @default () => window
   */
  targetContainer?: TargetContainer;
  /**
   * 语言包配置
   */
  locale?: object;
  /**
   * 设置统一样式前缀。注意：需要配合 less 变量 `@prefix` 使用
   */
  prefixCls?: string;
  /**
   * 设置 Form 组件的通用属性
   */
  form?: {
    validateMessages?: ValidateMessages;
    // requiredMark?: RequiredMark;
  };
}
