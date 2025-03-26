import { z } from 'zod';
import { sectorSchema } from './Sector';
import { PaginatedList, parsePaginatedList } from '../common/PaginatedList';
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

export type BrandsPage = PaginatedList<typeof brandSchema>;

export const parseBrandPage = parsePaginatedList(brandSchema);
