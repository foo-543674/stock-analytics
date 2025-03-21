export type OmittedSymbol = '...';
export type PaginationItem = number | OmittedSymbol;

const omittedSymbol: OmittedSymbol = '...';

export const generatePages = (
  currentpage: number,
  maxPage: number,
  visibleRange: number,
): PaginationItem[] => {
  const start = Math.max(currentpage - visibleRange, 1);
  const end = Math.min(currentpage + visibleRange, maxPage);

  const numbers = Array.from({ length: end - start + 1 }, (_, i) => start + i);

  return [
    ...(numbers[0] > 1 ? [omittedSymbol] : []),
    ...numbers,
    ...(numbers[numbers.length - 1] < maxPage ? [omittedSymbol] : []),
  ];
};
