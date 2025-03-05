import type { IProjectSettings, IProjectSettingsJson } from "./types";

export class ProjectSettingsModel implements IProjectSettings {
	public ignorePatterns?: string[] | undefined;
	public useGitignore?: boolean | undefined;
	constructor(data: IProjectSettings) {
		this.ignorePatterns = data.ignorePatterns
		this.useGitignore = data.useGitignore
	}

	static fromJson(json: IProjectSettingsJson): ProjectSettingsModel {
		return new ProjectSettingsModel({
			ignorePatterns: json.ignore_patterns,
			useGitignore: json.use_gitignore,
		})
	}

	toJson(): IProjectSettingsJson {
		return {
			ignore_patterns: this.ignorePatterns,
			use_gitignore: this.useGitignore
		}
	}
}
