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
    fieldName: 'fieldName',
    constraints: [
      {
        rule: 'validation.required',
        args: [],
      },
      {
        rule: 'validation.duplicate',
        args: ['foo'],
      },
      {
        rule: 'validation.max_length',
        args: ['10'],
      },
      {
        rule: 'validation.length_equals',
        args: ['5'],
      },
    ],
    translate: translateStub,
  },
};
