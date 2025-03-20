import { camelizeKeys, isObject } from '@/utils/ObjectHelper';
import { z } from 'zod';

export const paginatedListSchema = <T extends z.ZodType>(schema: T) =>
  z.object({
    page: z.number(),
    maxPage: z.number(),
    items: z.array(schema),
  });

export type PaginatedList<T extends z.ZodType> = z.infer<
  ReturnType<typeof paginatedListSchema<T>>
>;

export const parsePaginatedList = <T extends z.ZodType>(
  schema: T,
  source: unknown,
): PaginatedList<T> => {
  if (!isObject(source)) {
    throw new Error(`Source is not parsable. source:${source}`);
  }

  return paginatedListSchema(schema).parse(camelizeKeys(source));
};
