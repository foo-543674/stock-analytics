import { fetchBrands } from '@/features/brands/api/fetchBrands';
import { BrandSearchCondition } from '@/features/brands/types/BrandSearchCondition';
import { createMemo, createResource, createSignal } from 'solid-js';
import { Brand } from '@/schemas/brands/Brand';
import { createOptionalParameterFunc } from '@/utils/createOptionalParameterFunc';
import { ApiClient } from '@/data-access/ApiClient';

export const createBrandsList = (client: ApiClient) => {
  const [page, setPage] = createSignal(1);
  const [filter, setFilter] = createSignal<BrandSearchCondition>({});
  const [res, { refetch }] = createResource(async () => {
    return await fetchBrands(client, filter(), page());
  });

  const createResult = createOptionalParameterFunc(
    {
      brands: [] as Brand[],
      maxPage: 1,
      isLoading: false,
      isError: false,
    },
    result => ({
      brands: result.brands,
      page: page(),
      maxPage: result.maxPage,
      filter: filter(),
      isLoading: result.isLoading,
      isError: result.isError,
      setFilter,
      setPage,
      refetch,
    }),
  );
  const memoized = createMemo(() => {
    if (res.state !== 'ready') return createResult({ isLoading: true });
    const response = res();
    if (response.isErr()) {
      //TODO: handle error
      return createResult({ isError: true });
    }
    const fetched = response.value;
    return createResult({
      brands: fetched.items,
      maxPage: fetched.maxPage,
    });
  });

  return memoized;
};
