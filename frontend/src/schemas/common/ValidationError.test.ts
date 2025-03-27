import { describe, it, expect } from 'vitest';
import { parseValidationError } from './ValidationError';

describe('ValidationError', () => {
  it('should convert object as ValidationError', () => {
    const source = {
      fields: [
        {
          name: 'name',
          keys: ['validation.required'],
        },
      ],
    };
    const result = parseValidationError(source);
    expect(result.isOk()).toBe(true);
    expect(result._unsafeUnwrap()).toEqual({
      fields: [
        {
          name: 'name',
          keys: ['validation.required'],
        },
      ],
    });
  });
});
