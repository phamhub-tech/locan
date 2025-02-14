import { api } from "~/_common/core/api";

import type { IProjectScanJson, IProjectShallowJson } from "../_models/project";

import type { IProjectAddPayload } from "./types";

export const projectsService = {
	getProjects() {
		return api.invoke<IProjectShallowJson[]>('get_projects')
	},
	addProject(payload: IProjectAddPayload) {
		return api.invoke<IProjectShallowJson>('add_project', payload)
	},
	getProject(uuid: string) {
		return api.invoke<IProjectShallowJson>('get_project', { uuid })
	},
	scanProject(rootDir: string) {
		return api.invoke<IProjectScanJson>('scan_project', { rootDir })
	},
}
