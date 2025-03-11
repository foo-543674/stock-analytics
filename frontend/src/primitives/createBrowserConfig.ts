import { Language } from '@/data-access/fetchLocales';
import browserLang from 'browser-lang';

export type BrowserConfig = {
  lang: Language;
};

export const createBrowserConfig = () => {
  const lang = browserLang({
    languages: ['en', 'ja'],
    fallback: 'en',
  }) as Language;

  return { lang };
};
