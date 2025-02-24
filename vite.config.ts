import { defineConfig } from 'vite';
import svelte from '@sveltejs/vite-plugin-svelte';
import wasm from 'vite-plugin-wasm';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [
    // svelte(),
    wasm(),
    sveltekit(),
    tailwindcss()
  ]
});
