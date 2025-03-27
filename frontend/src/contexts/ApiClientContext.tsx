import { ApiClient } from '@/data-access/ApiClient';
import { createContext } from 'solid-js';

export const ApiClientContext = createContext<ApiClient>();
export const ApiClientProvider = ApiClientContext.Provider;
