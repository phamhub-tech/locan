import type { TProfileApiResponse } from "./interface"

const basePath = '/users/profile'

export const profileService = {
	getProfile() {
		return new Promise<TProfileApiResponse>((resolve, reject) => {
			const { $api: api } = useNuxtApp()
			api.get(basePath, {
				onSuccess: resolve,
				onError: reject,
			})
		})
	}
}
