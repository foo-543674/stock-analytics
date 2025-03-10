export type Replacement = {
  [key: string]: string;
};
export type Translate = (key: string, replacements?: Replacement) => string;

export const useLocales = () => {};
