import { Meta, StoryObj } from 'storybook-solidjs';
import { translateStub } from '@tests/mocks/TranslateStub';
import { fn } from '@storybook/test';
import { SearchForm } from './SearchForm';
import { sectors } from '@tests/mocks/sectors';

const meta: Meta<typeof SearchForm> = {
  component: SearchForm,
};
export default meta;
type Story = StoryObj<typeof SearchForm>;

export const Default: Story = {
  args: {
    translate: translateStub,
    onSubmit: fn(),
    onInputChanged: fn(),
  },
};

export const Disabled: Story = {
  args: {
    sectors,
    disabled: true,
    translate: translateStub,
    onSubmit: fn(),
    onInputChanged: fn(),
  },
};

export const WithConditionsInput: Story = {
  args: {
    sectors,
    selectedSectorId: '01JPQXE4EXD965A1MMFGP4BN53',
    code: '2050',
    brandName: 'Amazon Inc.',
    translate: translateStub,
    onSubmit: fn(),
    onInputChanged: fn(),
  },
};
