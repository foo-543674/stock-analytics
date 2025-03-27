/// <reference types="vitest" />
import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import tsconfigPaths from 'vite-tsconfig-paths';
import tailwindcss from '@tailwindcss/vite';
import { viteStaticCopy } from 'vite-plugin-static-copy';
import path, { resolve } from 'path';
import serveStatic from 'vite-plugin-serve-static';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    solid(),
    tailwindcss(),
    tsconfigPaths(),
    viteStaticCopy({ targets: [{ src: 'src/assets', dest: '.' }] }),
    serveStatic([
      {
        pattern: /^\/assets\/(.*)\?/,
        resolve: groups => path.join('./src/assets/', groups[1]),
      },
    ]),
  ],
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
  resolve: {
    alias: {
      '@tests': resolve(__dirname, './tests'),
      '@': resolve(__dirname, './src'),
    },
  },
  publicDir: './src/assets',
});
