import { relaunch } from '@tauri-apps/plugin-process'
import type { Theme } from '@tauri-apps/api/window'

import { pageSizeOptions } from '~/_common/constants'

import type { ISavedSettings, ISettings, ILanguage } from '../_types'
import { TApiStatus } from '~/_common/core/api'
import { AppSettings } from '../_models/settings'
import { getApiMessage } from '~/_common/utils'
import { settingsService } from '../_service'
import { check, Update } from '@tauri-apps/plugin-updater'
import { AppInfo } from '../_models/app-info'


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
	appInfo: AppInfo | null;
	languages: ILanguage[];
	activeLanguage: ILanguage;

	pageSize: number;

	settingsApiStatus: TApiStatus;
	settingsApiMsg: string;
	settings: AppSettings | null;

	saveSettingsApiStatus: TApiStatus;
	saveSettingsApiMsg: string;

	updateDownloadApiStatus: TApiStatus,
	updateDownloadApiMsg: string,
	update: Update | null,
	updateSizeTotal: number | null;
	updateSizeDownloaded: number | null;
	updateDownloadProgress: number | null;
	updateDownloadMsg: string;
}

const storeStorageKey = 'settings'
function loadSettings(): ISavedSettings | null {
	const { $storage } = useNuxtApp()
	return $storage.getItem<ISavedSettings>(storeStorageKey, true)
}

const state = (): IState => {
	return {
		appInfo: null,

		..._defaultSettings,
		languages,

		settingsApiStatus: TApiStatus.default,
		settingsApiMsg: '',
		settings: null,

		saveSettingsApiStatus: TApiStatus.default,
		saveSettingsApiMsg: '',

		updateDownloadApiStatus: TApiStatus.default,
		updateDownloadApiMsg: '',
		update: null,
		updateSizeTotal: null,
		updateSizeDownloaded: null,
		updateDownloadProgress: null,
		updateDownloadMsg: '',
	}
}

export const useSettingsStore = defineStore('settings', {
	state,
	getters: {
		storageKey: () => storeStorageKey,
		updateExists: (state) => state.update !== null && state.update.available,
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

		async init() {
			const info = new AppInfo();
			await info.build();
			this.appInfo = info;

			const savedSettings = {
				..._defaultSettings,
				...loadSettings()
			}

			const activeLanguage =
				languages.find((t) => t.locale === savedSettings.activeLanguageKey) ??
				_defaultSettings.activeLanguage

			this.activeLanguage = activeLanguage
			this.getSettings()
			this.checkAndDownloadUpdate()
		},
		setTheme(theme: Theme | null) {
			const colorMode = useColorMode();
			colorMode.preference = theme ?? 'system';
			this.appInfo!.theme = theme;
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
			} catch (e) {
				this.saveSettingsApiStatus = TApiStatus.error
				this.saveSettingsApiMsg = getApiMessage(e);
			}
		},

		async checkAndDownloadUpdate() {
			try {
				this.updateDownloadApiStatus = TApiStatus.loading;
				this.updateDownloadApiMsg = '';

				const update = await check();
				this.update = update;
				if (update === null) {
					this.updateDownloadApiStatus = TApiStatus.success;
					return;
				};

				console.log(
					`found update ${update.available} ${update.version} from ${update.date} with notes ${update.body}`,
				);

				let contentLength = 0;
				await update.download((event) => {
					switch (event.event) {
						case "Started":
							contentLength = event.data.contentLength ?? 0;
							this.updateSizeTotal = contentLength
							this.updateSizeDownloaded = 0;
							break;
						case "Progress":
							this.updateSizeDownloaded! += event.data.chunkLength;
							this.updateDownloadProgress = this.updateSizeDownloaded! / this.updateSizeTotal!;
							break;
						case "Finished":
							console.log("download finished");
							break;
					}
				});

				this.updateDownloadApiStatus = TApiStatus.success;
			} catch (e) {
				this.updateDownloadApiStatus = TApiStatus.error;
				this.updateDownloadApiMsg = getApiMessage(e);
			}
		},

		async updateApp() {
			const update = this.update;
			if (update === null) return;

			await update.install()
			await relaunch()
		},

		resetSettings() {
			settingsService.resetSettings()
		},

		reset() {
			this.$reset()
		},
	},
})
