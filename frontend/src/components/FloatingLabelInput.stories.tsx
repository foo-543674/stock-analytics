import { Meta, StoryObj } from 'storybook-solidjs';
import { FloatingLabelInput } from './FloatingLabelInput';
import { fn } from '@storybook/test';

const meta: Meta<typeof FloatingLabelInput> = {
  component: FloatingLabelInput,
};
export default meta;
type Story = StoryObj<typeof FloatingLabelInput>;

export const Default: Story = {
  args: {
    placeholder: 'placeholder',
    label: 'Label',
    onInput: fn(),
    onChange: fn(),
    onBlur: fn(),
  },
};

export const HasValue: Story = {
  args: {
    placeholder: 'placeholder',
    label: 'Label',
    value: 'Value',
    onInput: fn(),
    onChange: fn(),
    onBlur: fn(),
  },
};

export const Small: Story = {
  render: () => <FloatingLabelInput small placeholder="small" />,
};

export const Medium: Story = {
  render: () => <FloatingLabelInput medium placeholder="medium" />,
};

export const Large: Story = {
  render: () => <FloatingLabelInput large placeholder="large" />,
};

export const Error: Story = {
  args: {
    placeholder: 'placeholder',
    label: 'Label',
    value: 'Value',
    error: true,
    onInput: fn(),
    onChange: fn(),
    onBlur: fn(),
  },
};

export const Disabled: Story = {
  args: {
    placeholder: 'placeholder',
    label: 'Label',
    value: 'Value',
    disabled: true,
    onInput: fn(),
    onChange: fn(),
    onBlur: fn(),
  },
};
