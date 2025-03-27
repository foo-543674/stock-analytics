import { fetchLocales, Language } from '@/data-access/fetchLocales';
import { createMemo, createResource } from 'solid-js';

export type Replacement = Record<string, string>;
export type Translate = (key: string, replacements?: Replacement) => string;

export const createTranslate = (lang: Language): (() => Translate) => {
  const [locales] = createResource(
    lang,
    async lang => await fetchLocales(lang),
  );
  const memoized = createMemo(() => {
    if (locales.loading) return () => '';

    const fetched = locales();
    if (!fetched || fetched.isErr()) {
      //TODO: handle error
      return () => '';
    }

    const dict = fetched.value;
    return (key: string, replacements?: Replacement) => {
      const source = dict[key];
      if (!source) {
        return key;
      }

      const replaced = replacements
        ? Object.entries(replacements).reduce(
            (acc, [key, value]) => acc.replace(`{{ ${key} }}`, value),
            source,
          )
        : source;

      return replaced;
    };
  });

  return memoized;
};
