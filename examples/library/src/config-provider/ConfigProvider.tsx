import { FormProvider } from "rc-field-form";
import * as React from "react";
import type { ConfigProviderProps, ValidateMessages } from "./types";

export const ConfigContext = React.createContext<ConfigProviderProps>({});

export const defaultPrefixCls = "za";

// let globalPrefixCls: string
// function getGlobalPrefixCls() {
//   return globalPrefixCls || defaultPrefixCls
// }

const ConfigProvider: React.FC<ConfigProviderProps> = (props) => {
  const { form, children } = props;

  let childNode = children;
  let validateMessages: ValidateMessages = {};

  if (form?.validateMessages) {
    validateMessages = { ...validateMessages, ...form.validateMessages };
  }

  if (Object.keys(validateMessages).length > 0) {
    childNode = (
      <FormProvider validateMessages={validateMessages}>
        {children}
      </FormProvider>
    );
  }

  return (
    <ConfigContext.Provider value={{}}>{childNode}</ConfigContext.Provider>
  );
};

export default ConfigProvider;
