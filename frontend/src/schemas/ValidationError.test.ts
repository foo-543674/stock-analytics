import { describe, it, expect } from 'vitest';
import { isValidationError } from './ValidationError';

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
    const result = isValidationError(source);
    expect(result).toEqual({
      fields: [
        {
          name: 'name',
          keys: ['validation.required'],
        },
      ],
    });
  });
});
