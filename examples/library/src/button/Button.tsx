import classnames from "classnames";
import * as React from "react";
import type { ButtonProps } from "./types";

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (props, ref) => {
    const { type = "default", className, disabled, children, ...rest } = props;
    const prefixCls = "button";

    const classes = classnames(
      prefixCls,
      {
        [`${prefixCls}-${type}`]: true,
      },
      className
    );

    return (
      <button
        role="button"
        ref={ref}
        className={classes}
        disabled={disabled}
        aria-disabled={disabled}
        {...rest}
      >
        {children}
      </button>
    );
  }
);

export default Button;
