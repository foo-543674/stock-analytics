import { describe, it, expect } from 'vitest';
import { parseValidationError } from './ValidationError';

describe('ValidationError', () => {
  it('should convert object as ValidationError', () => {
    const source = {
      fields: [
        {
          name: 'name',
          constraints: [
            {
              rule: 'validation.required',
              args: [],
            },
            {
              rule: 'validation.duplicate',
              args: [],
            },
            {
              rule: 'validation.length_equals',
              args: ['10'],
            },
          ],
        },
      ],
    };
    const result = parseValidationError(source);
    expect(result.isOk()).toBe(true);
    expect(result._unsafeUnwrap()).toEqual({
      fields: [
        {
          name: 'name',
          constraints: [
            {
              rule: 'validation.required',
              args: [],
            },
            {
              rule: 'validation.duplicate',
              args: [],
            },
            {
              rule: 'validation.length_equals',
              args: ['10'],
            },
          ],
        },
      ],
    });
  });
});
