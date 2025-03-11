import { Replacement, Translate } from '@/primitives/createTranslate';

const dict: { [key: string]: string } = {
  serviceName: 'Stock Analytics',
  brandPageLink: 'Brands',
};

export const translateStub: Translate = (
  key: string,
  replacements?: Replacement,
) => {
  if (dict[key]) {
    const source = dict[key];
    const replaced = replacements
      ? Object.entries(replacements).reduce(
          (acc, [key, value]) => acc.replace(`{{ ${key} }}`, value),
          source,
        )
      : source;

    return replaced;
  }

  return key;
};
