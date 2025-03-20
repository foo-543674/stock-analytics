import { z } from 'zod';
import { sectorSchema } from './Sector';
import { camelizeKeys, isObject } from '@/utils/ObjectHelper';
import { PaginatedList, parsePaginatedList } from './PaginatedList';

export const brandSchema = z
  .object({
    id: z.string().ulid(),
    name: z.string(),
    code: z.string(),
    sector: sectorSchema,
  })
  .strip();

export type Brand = z.infer<typeof brandSchema>;

export const parseBrand = (source: unknown): Brand => {
  if (!isObject(source)) {
    throw new Error(`Source is not parsable. source:${source}`);
  }

  return brandSchema.parse(camelizeKeys(source));
};

export const parseBrandPage = (
  source: unknown,
): PaginatedList<typeof brandSchema> => {
  return parsePaginatedList(brandSchema, source);
};
