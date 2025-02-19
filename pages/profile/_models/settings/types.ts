export interface IAppSettingsJson {
	scan: {
		ignore_patterns: string[];
	}
}

export interface IScanSettings {
	ignorePatterns: string[];
}
export interface IAppSettings {
	scan: IScanSettings;
}
