import { defineConfig } from 'astro/config'

// Import Astro integrations
import icon from 'astro-icon'
import sitemap from '@astrojs/sitemap'
import tailwind from '@astrojs/tailwind'

// https://astro.build/config
export default defineConfig({
  // To ensure that our provider displays `/partners` without trailing slashes output as files instead of directories
  build: { format: 'file' },
  integrations: [
    icon({ include: { mdi: ['*'] } }),
    sitemap(),
    tailwind(),
  ],
  // The production site will be hosted here
  site: 'https://execut.nl/',
})
