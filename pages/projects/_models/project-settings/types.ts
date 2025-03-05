import type { IScanSettings, IScanSettingsJson } from "~/pages/profile/_models/settings";

export interface IProjectSettingsJson extends Partial<IScanSettingsJson> { }
export interface IProjectSettings extends Partial<IScanSettings> { }

