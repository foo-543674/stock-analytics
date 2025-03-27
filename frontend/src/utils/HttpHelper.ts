export const isSuccess = (code: number): boolean => code >= 200 && code < 300;
export const isClientError = (code: number): boolean =>
  code >= 400 && code < 500;
export const isServerError = (code: number): boolean =>
  code >= 500 && code < 600;

export const BAD_REQUEST = 400;
export const UNAUTHORIZED = 401;
export const FORBIDDEN = 403;
export const NOT_FOUND = 404;
export const CONFLICT = 409;
