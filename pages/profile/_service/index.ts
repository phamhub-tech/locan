import { api } from "~/_common/core/api"

import type { IAppSettingsJson } from "../_models/settings"

export const profileService = {
	getProfile() {
		//api.get(basePath, {
		//	onSuccess: resolve,
		//	onError: reject,
		//})
	}
}

export const settingsService = {
	getSettings() {
		return api.invoke<IAppSettingsJson>("get_settings");
	},
	saveSettings(settings: IAppSettingsJson) {
		return api.invoke<string>("save_settings", { newSettings: settings })
	},
	resetSettings() {
		return api.invoke<string>("reset_settings")
	}
}
