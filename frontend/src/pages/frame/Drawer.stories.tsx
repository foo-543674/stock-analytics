import { Meta, StoryObj } from 'storybook-solidjs';
import { Drawer } from './Drawer';
import { translateStub } from '@tests/mocks/TranslateStub';
import { fn } from '@storybook/test';

const meta: Meta<typeof Drawer> = {
  component: Drawer,
};
export default meta;
type Story = StoryObj<typeof Drawer>;

export const Desktop: Story = {
  args: {
    translate: translateStub,
    isMenuOpened: false,
    onMenuOpenChanged: fn(),
  },
  render: props => {
    return (
      <Drawer {...props}>
        <button class="btn btn-primary">Button</button>
      </Drawer>
    );
  },
};

export const Mobile: Story = {
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
      <Drawer {...props}>
        <button class="btn btn-primary">Button</button>
      </Drawer>
    );
  },
};
