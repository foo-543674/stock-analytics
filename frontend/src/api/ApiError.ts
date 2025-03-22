import { ValidationError } from '@/schemas/ValidationError';

export type NotFound = {
  _tag: 'NotFound';
};
export const notFound = (): NotFound => ({ _tag: 'NotFound' });

export type ValidationFailed = {
  _tag: 'ValidationFailed';
  description: ValidationError;
};
export const validationFailed = (
  description: ValidationError,
): ValidationFailed => ({
  _tag: 'ValidationFailed',
  description,
});

export type BadRequest = {
  _tag: 'BadRequest';
  message: string;
};
export const badRequest = (message: string): BadRequest => ({
  _tag: 'BadRequest',
  message,
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
