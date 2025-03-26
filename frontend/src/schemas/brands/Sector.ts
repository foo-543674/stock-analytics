import { z } from 'zod';
import { parseApiResponse } from '../common/parseApiResponse';

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

export const parseSector = parseApiResponse(sectorSchema);
export const parseSectorList = parseApiResponse(z.array(sectorSchema));
