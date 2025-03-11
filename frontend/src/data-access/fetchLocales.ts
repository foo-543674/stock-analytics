import axios from 'axios';

export type Language = 'en' | 'ja';

export const fetchLocales = async (
  lang: Language,
): Promise<Record<string, string>> => {
  const response = await axios.get(`/locales/${lang}.json`);

  if (response.status !== 200) {
    throw new Error('Failed to fetch locales');
  }

  return response.data as Record<string, string>;
};
