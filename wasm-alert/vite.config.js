import { defineConfig } from 'vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

import { createHtmlPlugin } from 'vite-plugin-html';

// Vite doesn't have a direct equivalent of Webpack's ProvidePlugin. You can use a polyfill instead.
import { TextEncoder, TextDecoder } from 'text-encoding';

global.TextEncoder = TextEncoder;
global.TextDecoder = TextDecoder;

export default defineConfig({
  build: {
    outDir: 'dist',
    rollupOptions: {
      input: './index.js',
      output: {
        entryFileNames: 'index.js',
      },
    },
  },
  plugins: [
    createHtmlPlugin(),
    wasm(),
    topLevelAwait()
  ],
  define: {
    'process.env.NODE_ENV': '"development"',
  },
});

