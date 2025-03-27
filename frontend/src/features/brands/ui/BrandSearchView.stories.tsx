import { Meta, StoryObj } from 'storybook-solidjs';
import { BrandSearchView } from './BrandSearchView';
import { brands } from '@tests/mocks/brands';
import { translateStub } from '@tests/mocks/TranslateStub';
import { fn } from '@storybook/test';
import { sectors } from '@tests/mocks/sectors';

const meta: Meta<typeof BrandSearchView> = {
  component: BrandSearchView,
};
export default meta;
type Story = StoryObj<typeof BrandSearchView>;

export const Default: Story = {
  args: {
    brands,
    sectors,
    translate: translateStub,
    page: 1,
    maxPage: 5,
    onSearchSubmit: fn(),
    onConditionChanged: fn(),
    onPageChanged: fn(),
    onBrandClick: fn(),
  },
};

export const Loading: Story = {
  args: {
    isLoading: true,
    translate: translateStub,
    page: 0,
    maxPage: 0,
    onSearchSubmit: fn(),
    onConditionChanged: fn(),
    onPageChanged: fn(),
    onBrandClick: fn(),
  },
};
