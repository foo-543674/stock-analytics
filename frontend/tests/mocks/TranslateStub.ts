import { Replacement, Translate } from '@/primitives/createTranslate';
import dict from '@/assets/locales/en.json';

const hasInDict = (key: string): key is keyof typeof dict => key in dict;

export const translateStub: Translate = (
  key: string,
  replacements?: Replacement,
) => {
  if (hasInDict(key)) {
    const source = dict[key];
    const replaced = replacements
      ? Object.entries(replacements).reduce(
          (acc, [key, value]) => acc.replace(`{{ ${key} }}`, value),
          source,
        )
      : source;

    return () => replaced;
  }

  return () => key;
};
