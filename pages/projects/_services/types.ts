import type { IApiResponse } from "~/_common/core/api";

import type { IScanFileJson, IScanResultJson } from "../_models/scan";
import type { IProjectShallowJson } from "../_models/project";
import type { IProjectSettingsJson } from "../_models/project-settings";

export type TProjectResponse = {
	project: IProjectShallowJson,
	settings: IProjectSettingsJson | null,
};

export interface IProjectAddPayload {
	name: string;
	rootDir: string;
}

export interface IProjectScansResponse {
	scans: IScanResultJson[],
	files: IScanFileJson[],
}

export interface IProjectScanResult {
	scan: IScanResultJson,
	files: IScanFileJson[]
}
