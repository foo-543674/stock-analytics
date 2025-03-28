import { describe, expect, it } from 'vitest';
import { parseBrand } from './Brand';

describe('Brand', () => {
  it('should convert object as Brand', () => {
    const source = {
      id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
      name: 'Brand 1',
      code: 'B1',
      sector: {
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'Sector 1',
        code: 'S1',
        group: 'Group 1',
        group_code: 1,
        category: 'Category 1',
      },
    };
    const result = parseBrand(source);
    expect(result.isOk()).toBe(true);
    expect(result._unsafeUnwrap()).toEqual({
      id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
      name: 'Brand 1',
      code: 'B1',
      sector: {
        id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
        name: 'Sector 1',
        code: 'S1',
        group: 'Group 1',
        groupCode: 1,
        category: 'Category 1',
      },
    });
  });
});
