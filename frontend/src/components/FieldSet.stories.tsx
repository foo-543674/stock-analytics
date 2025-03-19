import { Meta, StoryObj } from 'storybook-solidjs';
import { FieldSet } from './FieldSet';

const meta: Meta<typeof FieldSet> = {
  component: FieldSet,
};
export default meta;
type Story = StoryObj<typeof FieldSet>;

export const Default: Story = {
  args: {
    label: 'field',
  },
  render: props => {
    return (
      <FieldSet {...props}>
        <input class="input" type="text" placeholder="input" />
      </FieldSet>
    );
  },
};

export const WithMessage: Story = {
  args: {
    label: 'field',
    message: 'message',
  },
  render: props => {
    return (
      <FieldSet {...props}>
        <input class="input" type="text" placeholder="input" />
      </FieldSet>
    );
  },
};
