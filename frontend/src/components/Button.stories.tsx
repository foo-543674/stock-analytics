import { Meta, StoryObj } from 'storybook-solidjs';
import { Button } from './Button';
import { fn } from '@storybook/test';

const meta: Meta<typeof Button> = {
  component: Button,
};
export default meta;
type Story = StoryObj<typeof Button>;

export const ContainedPrimary: Story = {
  args: {
    variant: 'contained',
    color: 'primary',
    children: 'Primary Button',
    onClick: fn(),
  },
};

export const ContainedSecondary: Story = {
  args: {
    variant: 'contained',
    color: 'secondary',
    children: 'Secondary Button',
    onClick: fn(),
  },
};

export const OutlinedPrimary: Story = {
  args: {
    variant: 'outlined',
    color: 'primary',
    children: 'Primary Button',
    onClick: fn(),
  },
};

export const OutlinedSecondary: Story = {
  args: {
    variant: 'outlined',
    color: 'secondary',
    children: 'Secondary Button',
    onClick: fn(),
  },
};

export const TextPrimary: Story = {
  args: {
    variant: 'text',
    color: 'primary',
    children: 'Primary Button',
    onClick: fn(),
  },
};

export const TextSecondary: Story = {
  args: {
    variant: 'text',
    color: 'secondary',
    children: 'Secondary Button',
    onClick: fn(),
  },
};
