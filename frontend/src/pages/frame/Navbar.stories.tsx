import { Meta, StoryObj } from 'storybook-solidjs';
import { Navbar } from './Navbar';
import { translateStub } from '@tests/mocks/TranslateStub';
import { fn } from '@storybook/test';
import { createSignal } from 'solid-js';
import { Theme } from '@/primitives/createTheme';

const meta: Meta<typeof Navbar> = {
  component: Navbar,
};
export default meta;
type Story = StoryObj<typeof Navbar>;

export const Desktop: Story = {
  args: {
    translate: translateStub,
  },
};

export const Dark: Story = {
  args: {
    translate: translateStub,
    theme: 'dark',
  },
};

export const ThemeSwapAnimation: Story = {
  args: {
    translate: translateStub,
  },
  render: props => {
    const [theme, toggleTheme] = createSignal<Theme>('light');
    return (
      <Navbar
        {...props}
        theme={theme()}
        toggleTheme={() => {
          toggleTheme(theme() === 'light' ? 'dark' : 'light');
        }}
      />
    );
  },
};

export const Mobile: Story = {
  args: {
    translate: translateStub,
    onMenuOpenChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
};
