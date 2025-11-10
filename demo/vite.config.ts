import { defineConfig } from 'npm:vite@^6.0.5';
import react from 'npm:@vitejs/plugin-react@^4.3.4';
import tailwindcss from 'npm:@tailwindcss/vite@^4.1.16';

export default defineConfig({
  plugins: [react(), tailwindcss()],
  build: {
    outDir: '../csaf-validator/src/web/static',
    emptyOutDir: true,
    rollupOptions: {
      external: ['/static/csaf_rs.js'],
      output: {
        entryFileNames: 'static/[name].js',
        chunkFileNames: 'static/[name].js',
        assetFileNames: 'static/[name].[ext]',
        paths: {
          '/static/csaf_rs.js': '/static/csaf_rs.js'
        }
      }
    }
  },
  server: {
    port: 3000,
  },
});
