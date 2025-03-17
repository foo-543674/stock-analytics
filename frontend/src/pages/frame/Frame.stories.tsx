import { Meta, StoryObj } from 'storybook-solidjs';
import { Frame } from './Frame';
import { fn } from '@storybook/test';
import { translateStub } from '@tests/mocks/TranslateStub';
import { createSignal } from 'solid-js';

const meta: Meta<typeof Frame> = {
  component: Frame,
};
export default meta;
type Story = StoryObj<typeof Frame>;

export const Desktop: Story = {
  args: {
    translate: translateStub,
    isMenuOpened: false,
    onMenuOpenChanged: fn(),
  },
  render: props => {
    return (
      <Frame {...props}>
        <button class="btn btn-primary">Button</button>
      </Frame>
    );
  },
};

export const MobileMenuClosed: Story = {
  args: {
    translate: translateStub,
    isMenuOpened: false,
    onMenuOpenChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
  render: props => {
    return (
      <Frame {...props}>
        <button class="btn btn-primary">Button</button>
      </Frame>
    );
  },
};

export const MobileMenuOpened: Story = {
  args: {
    translate: translateStub,
    isMenuOpened: true,
    onMenuOpenChanged: fn(),
  },
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
  render: props => {
    return (
      <Frame {...props}>
        <button class="btn btn-primary">Button</button>
      </Frame>
    );
  },
};

export const Animation: Story = {
  parameters: {
    viewport: {
      defaultViewport: 'mobile1',
    },
  },
  render: () => {
    const [isMenuOpened, setIsMenuOpened] = createSignal(false);
    return (
      <Frame
        isMenuOpened={isMenuOpened()}
        onMenuOpenChanged={setIsMenuOpened}
        translate={translateStub}
      >
        <button class="btn btn-primary">Button</button>
      </Frame>
    );
  },
};
