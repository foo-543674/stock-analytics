import { describe, expect, it } from 'vitest';
import { parsePaginatedList } from './PaginatedList';
import { z } from 'zod';

describe('PageinatedList', () => {
  it('should convert object as PaginatedList', () => {
    const source = {
      page: 1,
      max_page: 2,
      items: ['foo', 'bar', 'baz'],
    };
    const result = parsePaginatedList(z.string())(source);
    expect(result.isOk()).toBe(true);
    expect(result._unsafeUnwrap()).toEqual({
      page: 1,
      maxPage: 2,
      items: ['foo', 'bar', 'baz'],
    });
  });
});
