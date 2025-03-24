import { ApiClient } from '@/data-access/api/ApiClient';
import { fetchBrands } from '@/data-access/api/brands/fetchBrands';
import { BrandSearchCondition } from '@/features/brands/BrandSearchCondition';
import { createMemo, createResource, createSignal } from 'solid-js';
import * as Either from '@/utils/Either';
import { Brand } from '@/schemas/Brand';
import { ApiError } from '@/data-access/api/ApiError';

export const createBrandsList = (client: ApiClient) => {
  const [page, setPage] = createSignal(1);
  const [filter, setFilter] = createSignal<BrandSearchCondition>({});
  const createApiResult = ({
    brands = [],
    maxPage = 1,
    error = null,
  }: Partial<{
    brands: Brand[];
    maxPage: number;
    error: ApiError | null;
  }>) => ({
    brands,
    maxPage,
    error,
  });
  const [res, { refetch }] = createResource(() => {
    return fetchBrands(client, filter(), page()).then(res =>
      Either.fold(
        res,
        error => createApiResult({ error }),
        data => createApiResult({ brands: data.items, maxPage: data.maxPage }),
      ),
    );
  });

  const memoized = createMemo(() => {
    const response = res.state === 'ready' ? res() : createApiResult({});
    return {
      brands: response.brands,
      page: page(),
      maxPage: response.maxPage,
      filter: filter(),
      isLoading: res.loading,
      isError: res.error !== undefined || response.error !== null,
      setFilter,
      setPage,
      refetch,
    };
  });

  return memoized;
};
