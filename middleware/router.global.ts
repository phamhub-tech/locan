import type { RouteMeta } from "vue-router"

import { getRouteName, retrieveToken } from "~/_common/utils"

interface IPageMeta extends RouteMeta {
	/**
	 * Whether this route requires an authenticated user or not
	 *
	 * If `true`, this route is public and accessible to everyone.
	 */
	noAuth?: boolean
}

export default defineNuxtRouteMiddleware((to) => {
	if (import.meta.server) return;

	const meta: IPageMeta = to.meta

	const requiresAuth = !meta.noAuth
	if (!requiresAuth) return

	const authToken = retrieveToken()
	if (!authToken?.isValid()) return navigateTo({ name: getRouteName('login'), query: { to: to.path } })
})
