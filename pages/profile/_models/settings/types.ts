export interface IAppSettingsJson {
	scan: {
		ignore_patterns: string[];
		use_gitignore: boolean;
	}
}

export interface IScanSettings {
	ignorePatterns: string[];
	useGitignore: boolean;
}
export interface IAppSettings {
	scan: IScanSettings;
}
