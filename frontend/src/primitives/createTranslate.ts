import { fetchLocales, Language } from '@/data-access/fetchLocales';
import { createMemo, createResource } from 'solid-js';

export type Replacement = Record<string, string>;
export type Translate = (
  key: string,
  replacements?: Replacement,
) => () => string;

export const createTranslate = (lang: Language): Translate => {
  const [locales] = createResource(lang, fetchLocales);

  return (key: string, replacements?: Replacement) => {
    const memoized = createMemo(() => {
      if (locales.loading || locales.error) {
        return '';
      }

      const dict = locales() as Record<string, string>;
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
    });

    return memoized;
  };
};
