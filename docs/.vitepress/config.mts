import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Promptt',
  description: 'A lightweight, interactive CLI prompts library for Rust.',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Getting Started', link: '/getting-started' },
      { text: 'Prompt Types', link: '/prompt-types' },
      { text: 'Examples', link: '/examples' },
    ],
    sidebar: [
      { text: 'Getting Started', link: '/getting-started' },
      { text: 'Prompt Types', link: '/prompt-types' },
      { text: 'Examples', link: '/examples' },
    ],
    socialLinks: [
      { icon: 'github', link: 'https://github.com/betterhyq/promptt' },
    ],
  },
})
