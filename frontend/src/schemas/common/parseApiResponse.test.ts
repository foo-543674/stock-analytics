import { describe, it, expect } from 'vitest';
import { z } from 'zod';
import { parseApiResponse } from './parseApiResponse';

describe('parseApiResponse', () => {
  it('should be success when parsed', () => {
    const schema = z.object({ id: z.number(), name: z.string() });
    const source = { id: 1, name: 'Test' };
    const result = parseApiResponse(schema)(source);
    expect(result.isOk()).toBe(true);
    expect(result._unsafeUnwrap()).toEqual({ id: 1, name: 'Test' });
  });

  it('should be error when parsed', () => {
    const schema = z.object({ id: z.number(), name: z.string() });
    const source = { id: '1', name: 'Test' };
    const result = parseApiResponse(schema)(source);
    expect(result.isErr()).toBe(true);
    expect(result._unsafeUnwrapErr()).toEqual({
      _tag: 'ParseError',
      source: { id: '1', name: 'Test' },
      message: 'Expected number, received string',
    });
  });

  it('should be error when source is not object', () => {
    const schema = z.string();
    const source = 'string';
    const result = parseApiResponse(schema)(source);
    expect(result.isErr()).toBe(true);
    expect(result._unsafeUnwrapErr()).toEqual({
      _tag: 'ParseError',
      source: 'string',
      message: 'source was not object',
    });
  });
});
