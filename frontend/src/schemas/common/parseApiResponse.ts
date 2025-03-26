import { z } from 'zod';
import { ParseResult } from '../ParseResult';
import { camelizeKeys, isObject } from '@/utils/ObjectHelper';
import { parseError } from '../ParseError';
import { ok } from '@/utils/Result';

export const parseApiResponse =
  <T>(schema: z.ZodType<T>) =>
  (source: unknown): ParseResult<T> => {
    if (!isObject(source)) {
      return parseError(source, 'source was not object');
    }

    const result = schema.safeParse(camelizeKeys(source));

    if (result.success) {
      return ok(result.data);
    } else {
      return parseError(
        source,
        result.error.issues.map(i => i.message).join('. '),
      );
    }
  };
