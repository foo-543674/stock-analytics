import { z } from 'zod';
import { camelizeKeys, isObject } from '@/utils/ObjectHelper';

export const sectorSchema = z
  .object({
    id: z.string().ulid(),
    name: z.string(),
    code: z.string(),
    group: z.string(),
    groupCode: z.number().positive(),
    category: z.string(),
  })
  .strip();

export type Sector = z.infer<typeof sectorSchema>;

export const parseSector = (source: unknown): Sector => {
  if (!isObject(source)) {
    throw new Error(`Source is not parsable. source:${source}`);
  }

  return sectorSchema.parse(camelizeKeys(source));
};
