import type { IScanResultJson } from "../_models/scan";

export interface IProjectAddPayload {
	name: string;
	rootDir: string;
}

export interface IProjectScansResponse {
	scans: IScanResultJson[],
	files: [],
}

export interface IProjectScanResult {
	scan: IScanResultJson,
	files: []
}
