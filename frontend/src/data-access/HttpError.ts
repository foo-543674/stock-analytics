import {
  BAD_REQUEST,
  CONFLICT,
  FORBIDDEN,
  isServerError,
  NOT_FOUND,
  UNAUTHORIZED,
} from '@/utils/HttpHelper';
import { fromThrowable } from '@/utils/Result';
import { HTTPError } from 'ky';

export type NotFound = {
  _tag: 'NotFound';
};
export const notFound = (): NotFound => ({ _tag: 'NotFound' });

export type BadRequest = {
  _tag: 'BadRequest';
  details: unknown;
};
export const badRequest = (details: unknown): BadRequest => ({
  _tag: 'BadRequest',
  details,
});

export type Conflict = {
  _tag: 'Conflict';
};
export const conflict = (): Conflict => ({ _tag: 'Conflict' });

export type Unauthorized = {
  _tag: 'Unauthorized';
};
export const unauthorized = (): Unauthorized => ({ _tag: 'Unauthorized' });

export type Forbidden = {
  _tag: 'Forbidden';
};
export const forbidden = (): Forbidden => ({ _tag: 'Forbidden' });

export type ServerError = {
  _tag: 'ServerError';
};
export const serverError = (): ServerError => ({ _tag: 'ServerError' });

export type UnknownError = {
  _tag: 'UnknownError';
};
export const unknownError = (): UnknownError => ({ _tag: 'UnknownError' });

export type HttpError =
  | NotFound
  | BadRequest
  | Conflict
  | Unauthorized
  | Forbidden
  | ServerError
  | UnknownError;

export const toHttpError = async (error: unknown): Promise<HttpError> => {
  if (!(error instanceof HTTPError)) {
    return unknownError();
  }

  if (isServerError(error.response.status)) {
    return serverError();
  }

  switch (error.response.status) {
    case BAD_REQUEST: {
      const body = await error.response.text();
      const result = fromThrowable(() => JSON.parse(body))();
      return badRequest(result.unwrapOr(body));
    }
    case UNAUTHORIZED:
      return unauthorized();
    case FORBIDDEN:
      return forbidden();
    case NOT_FOUND:
      return notFound();
    case CONFLICT:
      return conflict();
    default:
      return unknownError();
  }
};
