import { ApiClientContext } from '@/contexts/ApiClientContext';
import { TranslateContext } from '@/contexts/TranslateContext';
import { ApiClient } from '@/data-access/ApiClient';
import { Translate } from '@/primitives/createTranslate';
import { Accessor, Component, Show, useContext } from 'solid-js';

export type PageComponentProps = {
  apiClient: Accessor<ApiClient>;
  translate: Accessor<Translate>;
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
          {t => p.children({ apiClient: a(), translate: t() })}
        </Show>
      )}
    </Show>
  );
};
