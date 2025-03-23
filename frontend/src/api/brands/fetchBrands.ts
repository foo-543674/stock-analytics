import { brandSchema } from '@/schemas/Brand';
import { ApiClient, ApiResponse } from '../ApiClient';
import { PaginatedList, parsePaginatedList } from '@/schemas/PaginatedList';
import { BrandSearchCondition } from '@/features/brands/BrandSearchCondition';
import { filterUndefinedProperties } from '@/utils/ObjectHelper';

const BRAND_LIST_COUNT = 10;

export const fetchBrands = async (
  client: ApiClient,
  condition: BrandSearchCondition,
  page?: number,
): Promise<ApiResponse<PaginatedList<typeof brandSchema>>> => {
  const query = filterUndefinedProperties({
    page: page?.toString() ?? '1',
    count: `${BRAND_LIST_COUNT}`,
    sector: condition.sectorId,
    name: condition.brandName,
    code: condition.code,
  });

  return await client.get('/brands', query, parsePaginatedList(brandSchema));
};
