import { ParseFn } from '@/schemas/ParseResult';
import ky from 'ky';
import { getRequest, HttpResult } from './http';

export interface ApiClient {
  get: <T>(
    path: string,
    query: Record<string, string>,
    parse: ParseFn<T>,
  ) => HttpResult<T>;
}

export const createApiClient = (baseUrl: string): ApiClient => {
  const client = ky.create({ prefixUrl: `${baseUrl}/api`, retry: 5 });

  return {
    get: getRequest(client),
  };
};
