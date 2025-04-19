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
          keys: ['validation.required'],
        },
        {
          name: 'code',
          keys: ['validation.required', 'validation.max_length'],
        },
        {
          name: 'name',
          keys: ['validation.required'],
        },
      ],
    },
    sectors: sectors,
    translate: translateStub,
    onChange: fn(),
    onSubmit: fn(),
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
  },
};
