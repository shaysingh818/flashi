import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";

// SITE_BASE lets the site be served from a sub-path, e.g. "/flashi/" on GitHub Pages.
export default defineConfig({
  // @ts-expect-error process is a nodejs global
  base: process.env.SITE_BASE ?? "/",
  plugins: [vue()],
  server: {
    port: 5180,
  },
});
