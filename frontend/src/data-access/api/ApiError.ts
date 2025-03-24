import { ValidationError } from '@/schemas/ValidationError';

export type NotFound = {
  _tag: 'NotFound';
};
export const notFound = (): NotFound => ({ _tag: 'NotFound' });

export type ValidationFailed = {
  _tag: 'ValidationFailed';
  details: ValidationError;
};
export const validationFailed = (
  details: ValidationError,
): ValidationFailed => ({
  _tag: 'ValidationFailed',
  details,
});

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

export type ApiError =
  | NotFound
  | ValidationFailed
  | BadRequest
  | Conflict
  | Unauthorized
  | Forbidden
  | ServerError
  | UnknownError;
