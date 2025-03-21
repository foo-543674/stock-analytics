import { Meta, StoryObj } from 'storybook-solidjs';
import { BrandList } from './BrandList';
import { brands } from '@tests/mocks/brands';
import { expect, fn, userEvent, within } from '@storybook/test';
import { waitFor } from '@solidjs/testing-library';

const meta: Meta<typeof BrandList> = {
  component: BrandList,
};
export default meta;
type Story = StoryObj<typeof BrandList>;

export const Default: Story = {
  args: {
    brands,
    onClick: fn(),
  },
  play: async ({ args, canvasElement }) => {
    const canvas = within(canvasElement);
    args.brands?.forEach(async brand => {
      const block = await canvas.findByTestId(`brand-block-${brand.id}`);
      await userEvent.click(block);
      await waitFor(() => expect(args.onClick).toHaveBeenCalledWith(brand));
    });
  },
};

export const Loading: Story = {
  args: {
    isLoading: true,
  },
};
