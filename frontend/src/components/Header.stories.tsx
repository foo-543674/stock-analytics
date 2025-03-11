import type { Meta, StoryObj } from 'storybook-solidjs';
import { Header } from './Header';
import { translateStub } from '@tests/mocks/TranslateStub';
import { createSignal } from 'solid-js';
import { fn, userEvent, within } from '@storybook/test';

const meta: Meta<typeof Header> = {
  component: Header,
};

export default meta;
type Story = StoryObj<typeof Header>;

export const Default: Story = {
  args: {
    translate: translateStub,
  },
  parameters: {
    viewport: {
      defaultViewport: 'desktop',
    },
  },
};

export const MobileMenuOpened: Story = {
  args: {
    isMenuOpened: true,
    translate: translateStub,
    onMenuOpenChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
};

export const MobileMenuClosed: Story = {
  args: {
    isMenuOpened: false,
    translate: translateStub,
    onMenuOpenChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
};

export const OpenAnimation: Story = {
  render: () => {
    const [opened, setOpened] = createSignal(false);

    return (
      <Header
        isMenuOpened={opened()}
        onMenuOpenChanged={setOpened}
        translate={translateStub}
      />
    );
  },
  play: async ({ canvasElement, step }) => {
    const canvas = within(canvasElement);
    await step('open menu', async () => {
      await userEvent.click(canvas.getByTestId('openBtn'));
    });
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
};

export const CloseAnimation: Story = {
  render: () => {
    const [opened, setOpened] = createSignal(true);

    return (
      <Header
        isMenuOpened={opened()}
        onMenuOpenChanged={setOpened}
        translate={translateStub}
      />
    );
  },
  play: async ({ canvasElement, step }) => {
    const canvas = within(canvasElement);
    await step('close menu', async () => {
      await userEvent.click(canvas.getByTestId('closeBtn'));
    });
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
};
