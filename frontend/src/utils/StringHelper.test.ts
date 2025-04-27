import { describe, expect, it } from 'vitest';
import { format } from './StringHelper';

describe('format', () => {
  it('should return as same as source when no args', () => {
    const result = format('Hello World');
    expect(result).toBe('Hello World');
  });

  it('should return formatted string with positional arguments', () => {
    const result = format('Hello {0} {1}', undefined, 'World', '!!!');
    expect(result).toBe('Hello World !!!');
  });

  it('should return formatted string with named arguments', () => {
    const result = format('Hello {name} {punctuation}', {
      name: 'World',
      punctuation: '!!!',
    });
    expect(result).toBe('Hello World !!!');
  });

  it('should return formatted string with mixed positional and named arguments', () => {
    const result = format(
      'Hello {0} {punctuation}',
      {
        punctuation: '!!!',
      },
      'World',
    );
    expect(result).toBe('Hello World !!!');
  });
});
