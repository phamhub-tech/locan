import { api } from "~/_common/core/api";

import type { IProjectShallowJson } from "../_models/project";

import type { IProjectAddPayload, IProjectScanResult, IProjectScansResponse, TProjectResponse } from "./types";
import type { IProjectSettingsJson } from "../_models/project-settings";

export const projectsService = {
	getProjects() {
		return api.invoke<IProjectShallowJson[]>('get_projects')
	},
	addProject(payload: IProjectAddPayload) {
		return api.invoke<IProjectShallowJson>('add_project', payload)
	},
	getProject(uuid: string) {
		return api.invoke<TProjectResponse>('get_project', { uuid })
	},
	getProjectScans(uuid: string) {
		return api.invoke<IProjectScansResponse>('get_project_scans', { uuid });
	},
	scanProject(uuid: string) {
		return api.invoke<IProjectScanResult>('scan_project', { uuid })
	},
	saveSettings(rootDir: string, settings: IProjectSettingsJson) {
		return api.invoke<string>("save_project_settings", { rootDir, newSettings: settings })
	},
}
