import type { Meta, StoryObj } from 'storybook-solidjs';
import { Header } from './Header';
import { fn } from '@storybook/test';

const meta: Meta<typeof Header> = {
  component: Header,
};

export default meta;
type Story = StoryObj<typeof Header>;

export const Default: Story = {
  args: {
    onToggleMenu: fn(),
  },
};
