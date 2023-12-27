import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
    base: '',
    build: {
        target: 'es2022',
        emptyOutDir: true,
    },
    optimizeDeps: {
        esbuildOptions: {
            target: 'es2022',
        },
    },
    plugins: [wasm()],
});
