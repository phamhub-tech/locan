import { TApiStatus } from "~/_common/core/api";
import { delay, getApiMessage } from "~/_common/utils";

import { ProjectShallowModel, type IProjectScan } from "../_models/project";
import { projectsService, type IProjectAddPayload } from "../_services";

interface IState {
	projectsApiStatus: TApiStatus;
	projectsApiMsg: string;
	projects: ProjectShallowModel[] | null;

	addProjectApiStatus: TApiStatus;
	addProjectApiMsg: string;

	projectApiStatus: TApiStatus;
	projectApiMsg: string;
	project: ProjectShallowModel | null;

	scanProjectApiStatus: TApiStatus;
	scanProjectApiMsg: string;
	scanResults: IProjectScan | null;
}

const state = (): IState => ({
	projectsApiStatus: TApiStatus.default,
	projectsApiMsg: '',
	projects: null,

	addProjectApiStatus: TApiStatus.default,
	addProjectApiMsg: '',

	projectApiStatus: TApiStatus.default,
	projectApiMsg: '',
	project: null,

	scanProjectApiStatus: TApiStatus.default,
	scanProjectApiMsg: '',
	scanResults: null,
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
		async getProject(uuid: string) {
			try {
				this.projectApiStatus = TApiStatus.loading
				this.projectApiMsg = ''

				const { data } = await projectsService.getProject(uuid)
				this.project = ProjectShallowModel.fromJson(data)

				this.projectApiStatus = TApiStatus.success
			} catch (e) {
				this.projectApiStatus = TApiStatus.error
				this.projectApiMsg = getApiMessage(e)
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
		async scanProject(rootDir: string) {
			try {
				this.scanProjectApiStatus = TApiStatus.loading
				this.scanProjectApiMsg = ''

				await delay(3000);
				const { data } = await projectsService.scanProject(rootDir)
				this.scanResults = data
				if (this.project?.rootDir === rootDir) {
					this.project.loc = data.loc
				}
				if (this.projects !== null) {
					const project = this.projects.find((p) => p.rootDir == rootDir);
					if (project) project.loc = data.loc
				}

				this.scanProjectApiStatus = TApiStatus.success
			} catch (e) {
				this.scanProjectApiStatus = TApiStatus.error
				this.scanProjectApiMsg = getApiMessage(e)
			}
		},

		async getProjectOverview(id: string) {
			try {
				this.projectApiStatus = TApiStatus.loading
				this.projectApiMsg = ''

				this.projectApiStatus = TApiStatus.success
			} catch (e) {
				this.projectApiStatus = TApiStatus.error
				this.projectApiMsg = getApiMessage(e)
			}
		}
	}
})
