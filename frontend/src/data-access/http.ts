import { KyInstance } from 'ky';
import { HttpError, toHttpError } from './HttpError';
import { fromPromise, ResultAsync } from '@/utils/Result';
import { isParseError, ParseError, ParseFn } from '@/schemas/ParseResult';

export type HttpResult<T> = ResultAsync<T, HttpError | ParseError>;

export const getRequest =
  (client: KyInstance) =>
  <T>(
    path: string,
    params: Record<string, string> = {},
    parse: ParseFn<T>,
  ): HttpResult<T> =>
    fromPromise(
      client
        .get(path, {
          searchParams: params,
        })
        .json(),
      e => e,
    )
      .andThen(parse)
      .mapErr(async e => {
        if (isParseError(e)) {
          return e;
        }
        return await toHttpError(e);
      });
