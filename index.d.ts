declare module '#app' {
	interface PageMeta {
		noAuth?: boolean
	}
}

declare global {
	interface Window {
		__TAURI__: any
	}
}

export {}
