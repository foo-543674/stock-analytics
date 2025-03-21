import { Meta, StoryObj } from 'storybook-solidjs';
import { BrandSearchView } from './BrandSearchView';
import { brands } from '@tests/mocks/brands';
import { translateStub } from '@tests/mocks/TranslateStub';

const meta: Meta<typeof BrandSearchView> = {
  component: BrandSearchView,
};
export default meta;
type Story = StoryObj<typeof BrandSearchView>;

export const Default: Story = {
  args: { brands, translate: translateStub, page: 1, maxPage: 5 },
};
