const host = process.env.TAURI_DEV_HOST

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: '2024-11-01',
	css: ['~/assets/css/main.scss'],
	devtools: { enabled: true },
	devServer: { host: host || 'localhost', port: 5000 },
	experimental: { typedPages: true },
	ssr: false,
	app: {
		head: {
			titleTemplate: '%s | Cluzar'
		}
	},
	hooks: {
		'pages:extend': function(pages) {
			function removePagesMatching(pattern: RegExp, pages: any[] = []) {
				const pagesToRemove: any[] = [];

				for (const page of pages) {
					if (page.file?.match(pattern)) {
						pagesToRemove.push(page);
					} else {
						removePagesMatching(pattern, page.children);
					}
				}

				for (const page of pagesToRemove) {
					pages.splice(pages.indexOf(page), 1);
				}
			}

			removePagesMatching(/\/pages\/(.*\/)*_/, pages);
		}
	},
	vite: {
		clearScreen: false,
		envPrefix: ['VITE_', 'TAURI_'],
		server: {
			strictPort: true,
			hmr: host
				? {
					protocol: "ws",
					host,
					port: 1421,
				}
				: undefined,
			watch: {
				// 3. tell vite to ignore watching `src-tauri`
				ignored: ["**/src-tauri/**"],
			},
		},
	},

	modules: [
		'@nuxtjs/tailwindcss',
		'shadcn-nuxt',
		'@nuxtjs/color-mode',
		'@nuxtjs/i18n',
		'@nuxt/fonts',
		'@vueuse/nuxt',
		'@pinia/nuxt',
	],
	colorMode: {
		classSuffix: '',
		storageKey: 'theme',
		preference: 'light',
	},
	fonts: {
		//families: [
		//	{ name: 'Manrope', provider: 'google' },
		//	{ name: 'Jet Brains Mono', provider: 'google' },
		//]
	},
	i18n: {
		lazy: true,
		langDir: './locales',
		defaultLocale: 'en',
		locales: [{ code: 'en', language: 'en-GB', file: 'en.json' }],
	},
	postcss: {
		plugins: {
			tailwindcss: {},
			autoprefixer: {},
		},
	},
	shadcn: {
		prefix: '',
		componentDir: '_common/components/ui',
	},
	tailwindcss: {
		cssPath: './assets/css/main.scss'
	}
})
