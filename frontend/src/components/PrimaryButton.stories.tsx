import { Meta, StoryObj } from 'storybook-solidjs';
import { PrimaryButton } from './PrimaryButton';
import { fn } from '@storybook/test';

const meta: Meta<typeof PrimaryButton> = {
  component: PrimaryButton,
};
export default meta;
type Story = StoryObj<typeof PrimaryButton>;

export const Default: Story = {
  args: {
    children: 'Primary Button',
    onClick: fn(),
  },
};
