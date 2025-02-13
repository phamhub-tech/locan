import { TApiStatus } from "~/_common/core/api";
import { getApiMessage } from "~/_common/utils";

import { ProjectShallowModel } from "../_models/project";
import { projectsService, type IProjectAddPayload } from "../_services";

interface IState {
	projectsApiStatus: TApiStatus;
	projectsApiMsg: string;
	projects: ProjectShallowModel[] | null;

	addProjectApiStatus: TApiStatus;
	addProjectApiMsg: string;

	projectOverviewApiStatus: TApiStatus;
	projectOverviewApiMsg: string;
	project: ProjectShallowModel[] | null;
}

const state = (): IState => ({
	projectsApiStatus: TApiStatus.default,
	projectsApiMsg: '',
	projects: null,

	addProjectApiStatus: TApiStatus.default,
	addProjectApiMsg: '',

	projectOverviewApiStatus: TApiStatus.default,
	projectOverviewApiMsg: '',
	project: null,
})

export const useProjectsStore = defineStore('projects', {
	state,
	actions: {
		async getProjects() {
			try {
				this.projectsApiStatus = TApiStatus.loading
				this.projectsApiMsg = ''

				const { data } = await projectsService.getProjects()
				this.projects = data.map(ProjectShallowModel.fromJson)

				this.projectsApiStatus = TApiStatus.success
			} catch (e) {
				this.projectsApiStatus = TApiStatus.error
				this.projectsApiMsg = getApiMessage(e)
			}
		},
		async addProject(payload: IProjectAddPayload) {
			try {
				this.addProjectApiStatus = TApiStatus.loading
				this.addProjectApiMsg = ''

				const { data } = await projectsService.addProject(payload)
				const project = ProjectShallowModel.fromJson(data)
				if (this.projects === null) {
					this.projects = [project]
				} else {
					this.projects.unshift(project)
				}

				this.addProjectApiStatus = TApiStatus.success
			} catch (e) {
				this.addProjectApiStatus = TApiStatus.error
				this.addProjectApiMsg = getApiMessage(e)
			}
		},

		async getProjectOverview(id: string) {
			try {
				this.projectOverviewApiStatus = TApiStatus.loading
				this.projectOverviewApiMsg = ''

				this.projectOverviewApiStatus = TApiStatus.success
			} catch (e) {
				this.projectOverviewApiStatus = TApiStatus.error
				this.projectOverviewApiMsg = getApiMessage(e)
			}
		}
	}
})
