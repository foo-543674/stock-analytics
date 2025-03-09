/// <reference types="vitest" />
import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import tsconfigPaths from 'vite-tsconfig-paths';
import tailwindcss from '@tailwindcss/vite';

// https://vite.dev/config/
export default defineConfig({
  plugins: [solid(), tailwindcss(), tsconfigPaths()],
  test: {
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
  },
  server: {
    watch: {
      usePolling: true,
      interval: 1000,
    },
    host: '0.0.0.0',
  },
  css: {
    modules: {
      localsConvention: 'dashes',
    },
  },
});
