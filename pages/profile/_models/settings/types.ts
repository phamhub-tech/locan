export interface IAppSettingsJson {
	scan: {
		ignore_dirs: string[];
		ignore_extensions: string[];
	}
}

export interface IScanSettings {
	ignoreDirs: string[];
	ignoreExtensions: string[];
}
export interface IAppSettings {
	scan: IScanSettings;
}
