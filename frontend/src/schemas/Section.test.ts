import { describe, expect, it } from 'vitest';
import { parseSection } from './Section';

describe('Section', () => {
  it('should convert object as Section', () => {
    const source = {
      id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
      name: 'Section 1',
      code: 'S1',
      group: 'Group 1',
      group_code: 1,
      category: 'Category 1',
    };
    const result = parseSection(source);
    expect(result).toEqual({
      id: '01F8ZQ2N9BZ5QZK5FVZQ4QZK5F',
      name: 'Section 1',
      code: 'S1',
      group: 'Group 1',
      groupCode: 1,
      category: 'Category 1',
    });
  });
});
