import { pageSizeOptions } from '~/_common/constants'

import type { ISavedSettings, ISettings, ILanguage } from '../_types'
import { TApiStatus } from '~/_common/core/api'
import { AppSettings } from '../_models/settings'
import { getApiMessage } from '~/_common/utils'
import { settingsService } from '../_service'


const languages: ILanguage[] = [
	{
		locale: 'en',
		name: 'english',
		countryCode: 'GB',
	},
]

const _defaultSettings: ISettings = {
	activeLanguage: languages.find((l) => l.locale === 'en')!,
	pageSize: pageSizeOptions[0],
}

interface IState extends ISettings {
	languages: ILanguage[];
	activeLanguage: ILanguage;

	pageSize: number;

	settingsApiStatus: TApiStatus;
	settingsApiMsg: string;
	settings: AppSettings | null;

	saveSettingsApiStatus: TApiStatus;
	saveSettingsApiMsg: string;
}

const storeStorageKey = 'settings'
function loadSettings(): ISavedSettings | null {
	const { $storage } = useNuxtApp()
	return $storage.getItem<ISavedSettings>(storeStorageKey, true)
}

const state = (): IState => {
	return {
		..._defaultSettings,
		languages,

		settingsApiStatus: TApiStatus.default,
		settingsApiMsg: '',
		settings: null,

		saveSettingsApiStatus: TApiStatus.default,
		saveSettingsApiMsg: '',
	}
}

export const useSettingsStore = defineStore('settings', {
	state,
	getters: {
		storageKey: () => storeStorageKey,
	},
	actions: {
		setLanguage(language: ILanguage) {
			this.activeLanguage = language
			this.saveLocalSettings()

			document.documentElement.setAttribute('lang', language.locale)
		},

		saveLocalSettings() {
			const settings: ISavedSettings = {
				activeLanguageKey: this.activeLanguage.locale,
				pageSize: this.pageSize,
			}

			const { $storage } = useNuxtApp()
			$storage.setItem(this.storageKey, settings, true)
		},

		init() {
			const savedSettings = {
				..._defaultSettings,
				...loadSettings()
			}

			const activeLanguage =
				languages.find((t) => t.locale === savedSettings.activeLanguageKey) ??
				_defaultSettings.activeLanguage

			this.activeLanguage = activeLanguage
			this.getSettings()
		},

		async getSettings() {
			if (this.settings !== null) return;

			try {
				this.settingsApiStatus = TApiStatus.loading;
				this.settingsApiMsg = '';

				const { data } = await settingsService.getSettings()
				this.settings = AppSettings.fromJson(data);

				this.settingsApiStatus = TApiStatus.success;
			} catch (e) {
				this.settingsApiStatus = TApiStatus.error;
				this.settingsApiMsg = getApiMessage(e)
			}
		},
		async saveSettings(settings: AppSettings) {
			try {
				this.saveSettingsApiStatus = TApiStatus.loading
				this.saveSettingsApiMsg = '';

				await settingsService.saveSettings(settings.toJson())
				this.settings = settings;

				this.saveSettingsApiStatus = TApiStatus.success
			} catch(e) {
				this.saveSettingsApiStatus = TApiStatus.error
				this.saveSettingsApiMsg = getApiMessage(e);
			}
		},
		resetSettings() {
			settingsService.resetSettings()
		},

		reset() {
			this.$reset()
		},
	},
})
