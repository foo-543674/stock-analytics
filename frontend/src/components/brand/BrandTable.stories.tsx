import type { StoryObj, Meta } from '@storybook/react';

import { BrandTable } from '@/components/brand/BrandTable';

const meta: Meta<typeof BrandTable> = {
  component: BrandTable,
};

export default meta;
type Story = StoryObj<typeof BrandTable>;

export const Default: Story = {
  args: {
    items: [
      {
        id: '1',
        name: 'Brand Name',
        code: 'BRAND_CODE',
        sector: {
          id: '1',
          name: 'Sector Name',
          code: 'SECTOR_CODE',
          group: 'Group Name',
          groupCode: 1,
          category: 'Category Name',
        },
        version: 1,
      },
      {
        id: '2',
        name: 'Brand Name',
        code: 'BRAND_CODE',
        sector: {
          id: '1',
          name: 'Sector Name',
          code: 'SECTOR_CODE',
          group: 'Group Name',
          groupCode: 1,
          category: 'Category Name',
        },
        version: 1,
      },
    ],
    loading: false,
  },
};

export const Loading: Story = {
  args: {
    items: [],
    loading: true,
  },
};
