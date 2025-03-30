import { createFetchAssets } from '@/data-access/FetchAssets';
import { createBrowserConfig } from '@/primitives/createBrowserConfig';
import { createTranslate, Translate } from '@/primitives/createTranslate';
import { Accessor, createContext, JSX } from 'solid-js';

export const TranslateContext = createContext<Accessor<Translate>>();

export type TranslateProviderProps = {
  children: JSX.Element;
};

export const TranslateProvider = (props: TranslateProviderProps) => {
  const browserConfig = createBrowserConfig();
  const translate = createTranslate(createFetchAssets(), browserConfig.lang);

  return (
    <TranslateContext.Provider value={translate}>
      {props.children}
    </TranslateContext.Provider>
  );
};
