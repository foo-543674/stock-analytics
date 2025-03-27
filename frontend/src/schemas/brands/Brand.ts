import { z } from 'zod';
import { sectorSchema } from './Sector';
import { PaginatedListType, parsePaginatedList } from '../common/PaginatedList';
import { parseApiResponse } from '../common/parseApiResponse';

export const brandSchema = z
  .object({
    id: z.string().ulid(),
    name: z.string(),
    code: z.string(),
    sector: sectorSchema,
  })
  .strip();

export type Brand = z.infer<typeof brandSchema>;

export const parseBrand = parseApiResponse(brandSchema);

export type BrandsPage = PaginatedListType<typeof brandSchema>;

export const parseBrandPage = parsePaginatedList(brandSchema);
