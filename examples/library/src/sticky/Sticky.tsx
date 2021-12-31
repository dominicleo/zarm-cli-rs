import classnames from "classnames";
import * as React from "react";
import type { StickyProps } from "./types";

const Sticky = React.forwardRef<HTMLDivElement, StickyProps>((props, ref) => {
  const { className, children, ...rest } = props;
  const prefixCls = "sticky";

  const classes = classnames(prefixCls, className);

  return (
    <div ref={ref} className={classes} {...rest}>
      {children}
    </div>
  );
});

export default Sticky;
