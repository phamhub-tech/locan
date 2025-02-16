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
}
