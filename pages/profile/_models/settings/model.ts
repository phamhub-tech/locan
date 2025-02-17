import type { IAppSettings, IAppSettingsJson, IScanSettings } from "./types";

export class AppSettings implements IAppSettings {
	public scan: IScanSettings;
	constructor(data: IAppSettings) {
		this.scan = data.scan;
	}

	static fromJson(json: IAppSettingsJson): AppSettings {
		const scan = json.scan;
		return new AppSettings({
			scan: {
				ignoreDirs: scan.ignore_dirs,
				ignoreExtensions: scan.ignore_extensions,
			}
		})
	}

	toJson(): IAppSettingsJson {
		const scan = this.scan;
		return {
			scan: {
				ignore_dirs: scan.ignoreDirs,
				ignore_extensions: scan.ignoreExtensions,
			}
		}
	}
}
