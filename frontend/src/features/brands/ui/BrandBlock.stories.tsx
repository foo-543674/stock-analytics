import { Meta, StoryObj } from 'storybook-solidjs';
import { BrandBlock } from './BrandBlock';
import { brands } from '@tests/mocks/brands';
import { expect, fn, userEvent, waitFor, within } from '@storybook/test';

const meta: Meta<typeof BrandBlock> = {
  component: BrandBlock,
};
export default meta;
type Story = StoryObj<typeof BrandBlock>;

export const Default: Story = {
  args: {
    brand: brands[0],
    onClick: fn(),
  },
  play: async ({ args, canvasElement }) => {
    const canvas = within(canvasElement);
    const block = await canvas.findByTestId(`brand-block-${args.brand.id}`);
    await userEvent.click(block);
    await waitFor(() => expect(args.onClick).toHaveBeenCalledWith(args.brand));
  },
};
