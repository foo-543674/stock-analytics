import { PaginatedList } from '@/schemas/common/PaginatedList';

export const pageStub = <T>(
  items: T[] = [],
  page = 1,
  maxPage = 10,
): PaginatedList<T> => {
  return {
    items,
    page,
    maxPage,
  };
};
