/// <reference types="vitest" />
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tsconfigPaths from 'vite-tsconfig-paths';

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tsconfigPaths()],
  test: {
    include: ['src/**/*.{test,spec}.{ts,tsx}'],
  },
  server: {
    watch: {
      usePolling: true,
      interval: 1000,
    },
    host: '127.0.0.1',
  },
});
