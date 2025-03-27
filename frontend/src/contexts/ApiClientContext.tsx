import { ApiClient, createApiClient } from '@/data-access/ApiClient';
import { Accessor, createContext, JSX } from 'solid-js';

export const ApiClientContext = createContext<Accessor<ApiClient>>();

export type ApiClientProviderProps = {
  baseUrl: string;
  children: JSX.Element;
};

export const ApiClientProvider = (props: ApiClientProviderProps) => {
  const baseUrl = () => props.baseUrl;
  const apiClient = createApiClient(baseUrl());

  return (
    <ApiClientContext.Provider value={() => apiClient}>
      {props.children}
    </ApiClientContext.Provider>
  );
};
