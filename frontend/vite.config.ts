/// <reference types="vitest" />
import { defineConfig, loadEnv } from 'vite';
import solid from 'vite-plugin-solid';
import tsconfigPaths from 'vite-tsconfig-paths';
import tailwindcss from '@tailwindcss/vite';
import { viteStaticCopy } from 'vite-plugin-static-copy';
import path, { resolve } from 'path';
import serveStatic from 'vite-plugin-serve-static';

// https://vite.dev/config/
export default ({ mode }: { mode: string }) => {
  const envDir = resolve(__dirname, '../');
  const env = { ...process.env, ...loadEnv(mode, envDir) };

  return defineConfig({
    envDir: envDir,
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
      port: env.VITE_PORT ? Number(env.VITE_PORT) : 5173,
      strictPort: true,
      host: env.VITE_HOST || 'localhost',
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
    build: {
      minify: mode === 'production',
    },
    publicDir: './src/assets',
  });
};
