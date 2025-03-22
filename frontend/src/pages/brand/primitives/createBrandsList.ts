import { ApiClient } from '@/api/ApiClient';
import { fetchBrands } from '@/api/brands/fetchBrands';
import { BrandSearchCondition } from '@/features/brands/BrandSearchCondition';
import { fold } from '@/utils/Either';
import { createSignal } from 'solid-js';
import { createResource } from 'solid-js/types/server/rendering.js';

export const createBrandsList = async (client: ApiClient) => {
  const [page, setPage] = createSignal(1);
  const [filter, setFilter] = createSignal<BrandSearchCondition>({});
  const queryState = () => ({ page: page(), filter: filter() });

  const [brandsPage] = createResource(queryState, state =>
    fetchBrands(client, state.filter, state.page),
  );
  if (!brandsPage.loading && !brandsPage.error) {
    const foo = brandsPage();
    if (!foo) {
      return [];
    }
    return fold(
      foo,
      e => [],
      v => v,
    );
  }
  const foo = brandsPage();
  return [brandsPage];
};
