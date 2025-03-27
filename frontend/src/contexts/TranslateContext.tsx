import { Translate } from '@/primitives/createTranslate';
import { createContext } from 'solid-js';

export const TranslateContext = createContext<Translate>();
export const TranslateProvider = TranslateContext.Provider;
