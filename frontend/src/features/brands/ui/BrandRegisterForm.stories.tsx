import { Meta, StoryObj } from 'storybook-solidjs';
import { BrandRegisterForm } from './BrandRegisterForm';
import { translateStub } from '@tests/mocks/TranslateStub';
import { sectors } from '@tests/mocks/sectors';
import { fn } from '@storybook/test';

const meta: Meta<typeof BrandRegisterForm> = {
  component: BrandRegisterForm,
};
export default meta;
type Story = StoryObj<typeof BrandRegisterForm>;

export const Default: Story = {
  args: {
    input: {
      code: '',
      name: '',
      sector: null,
    },
    sectors: sectors,
    translate: translateStub,
    onChange: fn(),
    onSubmit: fn(),
    onCancel: fn(),
  },
};

export const HasValue: Story = {
  args: {
    input: {
      code: '1234',
      name: 'Test Brand',
      sector: sectors[0],
    },
    sectors: sectors,
    translate: translateStub,
    onChange: fn(),
    onSubmit: fn(),
    onCancel: fn(),
  },
};

export const HasError: Story = {
  args: {
    input: {
      code: '1234',
      name: 'Test Brand',
      sector: sectors[0],
    },
    validationError: {
      fields: [
        {
          name: 'sector',
          constraints: [
            {
              rule: 'validation.required',
              args: [],
            },
          ],
        },
        {
          name: 'code',
          constraints: [
            {
              rule: 'validation.required',
              args: [],
            },
            {
              rule: 'validation.length_equals',
              args: ['4'],
            },
          ],
        },
        {
          name: 'name',
          constraints: [
            {
              rule: 'validation.required',
              args: [],
            },
            {
              rule: 'validation.max_length',
              args: ['100'],
            },
          ],
        },
      ],
    },
    sectors: sectors,
    translate: translateStub,
    onChange: fn(),
    onSubmit: fn(),
    onCancel: fn(),
  },
};

export const Disabled: Story = {
  args: {
    input: {
      code: '',
      name: '',
      sector: null,
    },
    sectors: sectors,
    disabled: true,
    translate: translateStub,
    onChange: fn(),
    onSubmit: fn(),
    onCancel: fn(),
  },
};
