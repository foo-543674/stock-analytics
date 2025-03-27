import { describe, it, expect } from 'vitest';
import { generatePages } from './PaginationHelper';

describe('PaginationHelper', () => {
  it('should generate page numbers from 1 to maxPage', () => {
    const result = generatePages(3, 5, 2);
    expect(result).toEqual([1, 2, 3, 4, 5]);
  });

  it('should be omitted between 1 and currentpage-visibleRange', () => {
    const result = generatePages(12, 12, 2);
    expect(result).toEqual(['...', 10, 11, 12]);
  });

  it('should be omitted between currentpage+visibleRange and maxPage', () => {
    const result = generatePages(1, 12, 2);
    expect(result).toEqual([1, 2, 3, '...']);
  });

  it('should be omitted between 1 and currentpage-visibleRange and currentpage+visibleRange and maxPage', () => {
    const result = generatePages(4, 7, 2);
    expect(result).toEqual(['...', 2, 3, 4, 5, 6, '...']);
  });
});
