import ky from 'ky';
import { getRequest, HttpResult } from './http';
import { ParseFn } from '@/schemas/ParseResult';

export type FetchAssets = <T>(
  path: string,
  params: Record<string, string>,
  parse: ParseFn<T>,
) => HttpResult<T>;

export const createFetchAssets = (): FetchAssets =>
  getRequest(ky.create({ prefixUrl: `/assets`, retry: 5 }));
