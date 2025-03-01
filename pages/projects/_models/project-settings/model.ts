import { ScanSettingsModel } from "~/pages/profile/_models/settings";
import type { IProjectSettings, IProjectSettingsJson } from "./types";

export class ProjectSettingsModel extends ScanSettingsModel implements IProjectSettings {

	static override fromJson(json: IProjectSettingsJson): ProjectSettingsModel {
		return new ProjectSettingsModel({
			...super.buildData(json)
		})
	}

	override toJson(): IProjectSettingsJson {
		return {
			...super.toJson(),
		}
	}
}
