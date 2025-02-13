import { api } from "~/_common/core/api";

import type { IProjectShallowJson } from "../_models/project";

import type { IProjectAddPayload } from "./types";

export const projectsService = {
	getProjects() {
		return api.invoke<IProjectShallowJson[]>('get_projects')
	},
	addProject(payload: IProjectAddPayload) {
		return api.invoke<IProjectShallowJson>('add_project', payload)
	}
}
