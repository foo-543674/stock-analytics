import { Meta, StoryObj } from 'storybook-solidjs';
import { ValidationHintList } from './ValidationHintList';
import { translateStub } from '@tests/mocks/TranslateStub';

const meta: Meta<typeof ValidationHintList> = {
  component: ValidationHintList,
};
export default meta;
type Story = StoryObj<typeof ValidationHintList>;

export const Default: Story = {
  args: {
    keys: ['validation.required', 'validation.max_length'],
    translate: translateStub,
  },
};
