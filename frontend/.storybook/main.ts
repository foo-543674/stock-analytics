// NOTE: storybook-solidjs-vite latest version has a bug, so we use html-vite instead
// import type { StorybookConfig } from 'storybook-solidjs-vite';
import { type StorybookConfig } from '@storybook/html-vite';

const config: StorybookConfig = {
  stories: ['../src/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
  addons: [
    {
      name: '@storybook/addon-essentials',
      options: {
        docs: false,
      },
    },
    '@chromatic-com/storybook',
    '@storybook/addon-interactions',
  ],
  framework: {
    name: '@storybook/html-vite',
    options: {},
  },
  core: {
    builder: '@storybook/builder-vite',
  },
};
export default config;
