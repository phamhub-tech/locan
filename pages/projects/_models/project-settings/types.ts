import type { IScanSettings, IScanSettingsJson } from "~/pages/profile/_models/settings";

export interface IProjectSettingsJson extends Partial<IScanSettingsJson> {
	merge_settings: boolean
}
export interface IProjectSettings extends Partial<IScanSettings> {
	mergeSettings: boolean; 
}

