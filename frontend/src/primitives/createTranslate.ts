import { fetchLocales, Language } from '@/data-access/fetchLocales';
import { createMemo, createResource } from 'solid-js';

export type Replacement = Record<string, string>;
export type Translate = (key: string, replacements?: Replacement) => string;

export const createTranslate = (lang: Language): Translate => {
  const [locales] = createResource(lang, fetchLocales);

  return (key: string, replacements?: Replacement) => {
    const text = createMemo(() => {
      if (locales.loading || locales.error) {
        return '';
      }
      const source = (locales() as Record<string, string>)[key] || key;
      const replaced = replacements
        ? Object.entries(replacements).reduce(
            (acc, [key, value]) => acc.replace(`{{ ${key} }}`, value),
            source,
          )
        : source;

      return replaced;
    });
    return text();
  };
};
