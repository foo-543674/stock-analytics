import { HttpResult } from '@/data-access/http';
import { unknownError } from '@/data-access/HttpError';
import { err, fromPromise } from '@/utils/Result';

export const errorResponseStub = err(unknownError());

export const neverReturningResponseImpl = <T>(): HttpResult<T> =>
  fromPromise(new Promise(() => {}), () => unknownError());
