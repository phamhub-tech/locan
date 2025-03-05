import type { IProjectSettings, IProjectSettingsJson } from "./types";

export class ProjectSettingsModel implements IProjectSettings {
	public mergeSettings: boolean;
	public ignorePatterns?: string[] | undefined;
	public useGitignore?: boolean | undefined;
	constructor(data: IProjectSettings) {
		this.mergeSettings = data.mergeSettings;
		this.ignorePatterns = data.ignorePatterns
		this.useGitignore = data.useGitignore
	}

	static fromJson(json: IProjectSettingsJson): ProjectSettingsModel {
		return new ProjectSettingsModel({
			mergeSettings: json.merge_settings,
			ignorePatterns: json.ignore_patterns,
			useGitignore: json.use_gitignore,
		})
	}

	toJson(): IProjectSettingsJson {
		return {
			merge_settings: this.mergeSettings,
			ignore_patterns: this.ignorePatterns,
			use_gitignore: this.useGitignore
		}
	}
}
