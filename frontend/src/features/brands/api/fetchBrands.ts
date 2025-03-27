import { BrandsPage, parseBrandPage } from '@/schemas/brands/Brand';
import { BrandSearchCondition } from '@/features/brands/types/BrandSearchCondition';
import { filterUndefinedProperties } from '@/utils/ObjectHelper';
import { ApiClient } from '@/data-access/ApiClient';
import { HttpResult } from '@/data-access/http';

const BRAND_LIST_COUNT = 10;

export const fetchBrands = (
  client: ApiClient,
  condition: BrandSearchCondition,
  page?: number,
): HttpResult<BrandsPage> => {
  const query = filterUndefinedProperties({
    page: page?.toString() ?? '1',
    count: `${BRAND_LIST_COUNT}`,
    sector: condition.sectorId,
    name: condition.brandName,
    code: condition.code,
  });

  return client.get('/brands', query, parseBrandPage);
};
