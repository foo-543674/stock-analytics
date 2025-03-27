import ky from 'ky';
import { getRequest, HttpResult } from './http';
import { ParseFn } from '@/schemas/ParseResult';

export type FetchAssets = <T>(
  path: string,
  params: Record<string, string>,
  parse: ParseFn<T>,
) => HttpResult<T>;

export const createFetchAssets = (baseUrl: string): FetchAssets =>
  getRequest(ky.create({ prefixUrl: `${baseUrl}/assets`, retry: 5 }));
