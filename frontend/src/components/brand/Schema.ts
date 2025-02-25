export type Sector = {
  id: string;
  name: string;
  code: string;
  group: string;
  groupCode: number;
  category: string;
};

export type Brand = {
  id: string;
  name: string;
  code: string;
  sector: Sector;
  version: number;
};
