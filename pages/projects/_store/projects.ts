import { TApiStatus } from "~/_common/core/api";
import { delay, getApiMessage } from "~/_common/utils";

import { ProjectShallowModel, type IProjectScan } from "../_models/project";
import { projectsService, type IProjectAddPayload, type IProjectScanResult } from "../_services";
import { ScanFileModel, ScanResultModel } from "../_models/scan";

interface IState {
	projectsApiStatus: TApiStatus;
	projectsApiMsg: string;
	projects: ProjectShallowModel[] | null;

	addProjectApiStatus: TApiStatus;
	addProjectApiMsg: string;

	projectApiStatus: TApiStatus;
	projectApiMsg: string;
	project: ProjectShallowModel | null;

	projectScansApiStatus: TApiStatus;
	projectScansApiMsg: string;
	projectScans: ScanResultModel[] | null;
	projectScanFiles: ScanFileModel[] | null;

	projectScanResultsApiStatus: TApiStatus;
	projectScanResultsApiMsg: string;
	projectScanResults: IProjectScanResult | null;
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

	projectScansApiStatus: TApiStatus.default,
	projectScansApiMsg: '',
	projectScans: null,
	projectScanFiles: null,

	projectScanResultsApiStatus: TApiStatus.default,
	projectScanResultsApiMsg: '',
	projectScanResults: null,
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
		async getProjectScans(uuid: string) {
			try {
				this.projectScansApiStatus = TApiStatus.loading
				this.projectScansApiMsg = ''

				const { data: { scans, files } } = await projectsService.getProjectScans(uuid)
				this.projectScans = scans.map(ScanResultModel.fromJson)
				this.projectScanFiles = files.map(ScanFileModel.fromJson)

				this.projectScansApiStatus = TApiStatus.success
			} catch (e) {
				this.projectScansApiStatus = TApiStatus.error
				this.projectScansApiMsg = getApiMessage(e)
			}
		},
		async scanProject(uuid: string) {
			try {
				this.projectScanResultsApiStatus = TApiStatus.loading
				this.projectScanResultsApiMsg = ''

				const [{ data }, _] = await Promise.all([
					projectsService.scanProject(uuid),
					delay(1000),
				]);
				const scan = ScanResultModel.fromJson(data.scan)

				this.projectScanResults = data
				if (this.project?.uuid === uuid) {
					const project = this.project;

					project.loc = scan.loc
					project.files = scan.files
					project.scans = (project.scans ?? 0) + 1
					project.lastScan = scan.scannedAt

					this.projectScanFiles = data
						.files
						.map(ScanFileModel.fromJson)
						.sort(({ loc: locA }, { loc: locB }) => locB - locA)
				}
				if (this.projects !== null) {
					const project = this.projects.find((p) => p.uuid == uuid);
					if (project) {
						project.loc = scan.loc
						project.files = scan.files
						project.scans = (project.scans ?? 0) + 1
						project.lastScan = scan.scannedAt
					}
				}


				this.projectScanResultsApiStatus = TApiStatus.success
			} catch (e) {
				this.projectScanResultsApiStatus = TApiStatus.error
				this.projectScanResultsApiMsg = getApiMessage(e)
			}
		},
	}
})
