import { KVStore } from "~/_common/core/storage"

export default defineNuxtPlugin(() => {
	return {
		provide: {
			storage: new KVStore()
		}
	}
})
