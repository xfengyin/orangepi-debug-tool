import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react({
      babel: {
        plugins: [
          ['@babel/plugin-transform-react-jsx', { runtime: 'automatic' }],
        ],
      },
    }),
  ],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
      '@components': path.resolve(__dirname, './src/components'),
      '@hooks': path.resolve(__dirname, './src/hooks'),
      '@stores': path.resolve(__dirname, './src/stores'),
      '@services': path.resolve(__dirname, './src/services'),
      '@types': path.resolve(__dirname, './src/types'),
      '@utils': path.resolve(__dirname, './src/utils'),
      '@styles': path.resolve(__dirname, './src/styles'),
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    host: true,
    hmr: {
      protocol: 'ws',
      host: 'localhost',
      port: 1420,
    },
  },
  build: {
    target: 'esnext',
    minify: 'terser',
    sourcemap: process.env.NODE_ENV !== 'production',
    rollupOptions: {
      output: {
        manualChunks: {
          'vendor-react': ['react', 'react-dom', 'react-router-dom'],
          'vendor-mui': ['@mui/material', '@mui/icons-material', '@emotion/react', '@emotion/styled'],
          'vendor-charts': ['recharts'],
          'vendor-utils': ['lodash-es', 'date-fns', 'uuid'],
        },
      },
    },
    chunkSizeWarningLimit: 1000,
  },
  optimizeDeps: {
    include: [
      'react',
      'react-dom',
      'react-router-dom',
      '@mui/material',
      '@mui/icons-material',
      'zustand',
      'recharts',
      'lodash-es',
    ],
  },
  envPrefix: ['VITE_', 'TAURI_'],
  clearScreen: false,
});