import { Meta, StoryObj } from 'storybook-solidjs';
import { SelectBox } from './SelectBox';
import { fn } from '@storybook/test';

const meta: Meta<typeof SelectBox> = {
  component: SelectBox,
};
export default meta;
type Story = StoryObj<typeof SelectBox>;

export const Default: Story = {
  args: {
    label: 'Label',
    items: [
      { label: 'Item 1', value: 'item1' },
      { label: 'Item 2', value: 'item2' },
      { label: 'Item 3', value: 'item3' },
    ],
    onChange: fn(),
    onInput: fn(),
    onBlur: fn(),
  },
};
