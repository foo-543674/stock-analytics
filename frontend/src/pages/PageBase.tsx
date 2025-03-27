import { ApiClientContext } from '@/contexts/ApiClientContext';
import { TranslateContext } from '@/contexts/TranslateContext';
import { ApiClient } from '@/data-access/ApiClient';
import { Translate } from '@/primitives/createTranslate';
import { Component, Show, useContext } from 'solid-js';

export type PageComponentProps = {
  initialApiClient: ApiClient;
  translate: Translate;
};

export type PageBaseProps = {
  children: Component<PageComponentProps>;
};

export const PageBase = (p: PageBaseProps) => {
  const apiClient = useContext(ApiClientContext);
  const translate = useContext(TranslateContext);

  return (
    <Show when={apiClient}>
      {a => (
        <Show when={translate}>
          {t => p.children({ initialApiClient: a(), translate: t() })}
        </Show>
      )}
    </Show>
  );
};
