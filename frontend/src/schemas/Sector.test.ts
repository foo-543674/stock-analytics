import { describe, expect, it } from 'vitest';
import { parseSector } from './Sector';

describe('Sector', () => {
  it('should convert object as Sector', () => {
    const source = {
      id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
      name: 'Sector 1',
      code: 'S1',
      group: 'Group 1',
      group_code: 1,
      category: 'Category 1',
    };
    const result = parseSector(source);
    expect(result).toEqual({
      id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
      name: 'Sector 1',
      code: 'S1',
      group: 'Group 1',
      groupCode: 1,
      category: 'Category 1',
    });
  });
});
