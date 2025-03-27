import { Meta, StoryObj } from 'storybook-solidjs';
import { Loading } from './Loading';

const meta: Meta<typeof Loading> = {
  component: Loading,
};
export default meta;
type Story = StoryObj<typeof Loading>;

export const Default: Story = {
  args: {},
};
