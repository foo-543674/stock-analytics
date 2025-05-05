import { Sector } from '@/schemas/brands/Sector';

export type BrandRegisterFormInput = {
  code: string;
  name: string;
  sector: Sector | null;
};
