import { z } from 'zod';
import { parseApiResponse } from './parseApiResponse';

export const paginatedListSchema = <T extends z.ZodType>(schema: T) =>
  z.object({
    page: z.number(),
    maxPage: z.number(),
    items: z.array(schema),
  });

export type PaginatedListType<T extends z.ZodType> = z.infer<
  ReturnType<typeof paginatedListSchema<T>>
>;

export type PaginatedList<T> = {
  page: number;
  maxPage: number;
  items: T[];
};

export const parsePaginatedList = <T extends z.ZodType>(schema: T) =>
  parseApiResponse(paginatedListSchema(schema));
