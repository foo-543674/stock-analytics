import { err, Result } from '@/utils/Result';

export type ParseError = {
  _tag: 'ParseError';
  source: unknown;
  message: string;
};

export const parseError = <T>(
  source: unknown,
  message: string = '',
): Result<T, ParseError> =>
  err({
    _tag: 'ParseError',
    source,
    message,
  });

export type ParseResult<T> = Result<T, ParseError>;
export type ParseFn<T> = (source: unknown) => ParseResult<T>;
