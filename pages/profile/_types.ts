export interface ILanguage {
	locale: string
	name: string
	countryCode: string
}

export interface ISavedSettings {
	activeLanguageKey: string
	pageSize: number
}

export interface ISettings extends Omit<ISavedSettings,  'activeLanguageKey'> {
	activeLanguage: ILanguage
}
