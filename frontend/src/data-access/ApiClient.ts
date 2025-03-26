import { ParseError, ParseFn } from '@/schemas/ParseResult';
import { ResultAsync } from '@/utils/Result';
import { HttpError } from './HttpError';
import ky from 'ky';
import { getRequest } from './http';

export interface ApiClient {
  get: <T>(
    path: string,
    query: Record<string, string>,
    parse: ParseFn<T>,
  ) => ResultAsync<T, HttpError | ParseError>;
}

export const createApiClient = (): ApiClient => {
  const client = ky.create({ prefixUrl: '/api', retry: 5 });

  return {
    get: getRequest(client),
  };
};
