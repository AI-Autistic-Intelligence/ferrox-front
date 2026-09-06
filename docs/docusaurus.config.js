// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Ferrox Front',
  tagline: 'Il Bootstrap Killer in puro Rust WebAssembly',
  favicon: 'img/favicon.ico',
  url: 'https://docs.ferrox-front.dev',
  baseUrl: '/',
  organizationName: 'ferrox-team',
  projectName: 'ferrox-front',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
          editUrl: 'https://github.com/ferrox-team/ferrox-front/tree/main/docs/',
        },
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        defaultMode: 'dark', // Ferrox Cyber by default!
        disableSwitch: false,
        respectPrefersColorScheme: false,
      },
      navbar: {
        title: 'Ferrox Front',
        logo: {
          alt: 'Ferrox Logo',
          src: 'img/logo.svg',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Documentazione',
          },
          {
            href: 'https://github.com/ferrox-team/ferrox-front',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Docs',
            items: [
              {
                label: 'Architettura',
                to: '/docs/architecture',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} Ferrox Team. Built with Docusaurus.`,
      },
      prism: {
        theme: require('prism-react-renderer').themes.github,
        darkTheme: require('prism-react-renderer').themes.dracula,
        additionalLanguages: ['rust', 'toml'],
      },
    }),
};

export default config;
