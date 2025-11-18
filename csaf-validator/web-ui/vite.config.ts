import { defineConfig } from 'npm:vite@^6.0.5';
import react from 'npm:@vitejs/plugin-react@^4.3.4';
import tailwindcss from 'npm:@tailwindcss/vite@^4.1.16';

export default defineConfig({
  plugins: [react(), tailwindcss()],
  build: {
    outDir: '../src/web/static',
    emptyOutDir: true,
    rollupOptions: {
      external: ['/assets/csaf_rs.js'],
      output: {
        entryFileNames: 'assets/[name].js',
        chunkFileNames: 'assets/[name].js',
        assetFileNames: 'assets/[name].[ext]',
        paths: {
          '/assets/csaf_rs.js': '/assets/csaf_rs.js'
        }
      }
    }
  },
  server: {
    port: 3000,
  },
});
