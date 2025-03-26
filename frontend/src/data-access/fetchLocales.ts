import { ok } from '@/utils/Result';
import { fetchAssets, HttpResult } from './http';
import { isObject } from '@/utils/ObjectHelper';
import { parseError } from '@/schemas/ParseResult';

export type Language = 'en' | 'ja';

export const fetchLocales = (
  lang: Language,
): HttpResult<Record<string, string>> => {
  return fetchAssets(`/locales/${lang}.json`, {}, data =>
    isObject(data)
      ? ok(data as Record<string, string>)
      : parseError(data, 'Invalid locales'),
  );
};
