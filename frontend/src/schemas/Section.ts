import { z } from 'zod';
import { camelizeKeys, isObject } from '@/utils/ObjectHelper';

export const sectionSchema = z
  .object({
    id: z.string().ulid(),
    name: z.string(),
    code: z.string(),
    group: z.string(),
    groupCode: z.number().positive(),
    category: z.string(),
  })
  .strip();

export type Section = z.infer<typeof sectionSchema>;

export const parseSection = (source: unknown): Section => {
  if (!isObject(source)) {
    throw new Error(`Source is not parsable. source:${source}`);
  }

  return sectionSchema.parse(camelizeKeys(source));
};
