import { defineConfig } from 'astro/config'

// Import Astro integrations
import node from '@astrojs/node'
import sitemap from '@astrojs/sitemap'
import tailwind from '@tailwindcss/vite'
import icon from 'astro-icon'

// https://astro.build/config
export default defineConfig({
  // The production site will be hosted here
  site: 'https://execut.nl/',
  trailingSlash: 'never',
  adapter: node({
    mode: 'standalone',
  }),
  integrations: [
    icon({ include: { mdi: ['*'] } }),
    sitemap(),
  ],
  vite: { plugins: [tailwind()] },
  // To ensure that our provider displays `/partners` without trailing slashes output as files instead of directories
  build: { format: 'file' },
  experimental: {
    session: true,
  },
})
