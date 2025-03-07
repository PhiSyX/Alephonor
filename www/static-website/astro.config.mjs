import { defineConfig } from "astro/config";
import mdx from "@astrojs/mdx";
import sitemap from "@astrojs/sitemap";

// https://astro.build/config
export default defineConfig({
	site: import.meta.env.VITE_APP_SITE,
	integrations: [mdx(), sitemap()],
});
