import { TApiStatus } from "~/_common/core/api";
import { delay, getApiMessage } from "~/_common/utils";

import { ProjectShallowModel, type IProjectScan } from "../_models/project";
import {
  projectsService,
  type IProjectAddPayload,
  type IProjectScanResult,
} from "../_services";
import { ScanFileModel, ScanResultModel } from "../_models/scan";
import {
  ProjectSettingsModel,
  type IProjectSettingsJson,
} from "../_models/project-settings";
import { useSettingsStore } from "~/pages/profile/_store";

interface IState {
  projectsApiStatus: TApiStatus;
  projectsApiMsg: string;
  projects: ProjectShallowModel[] | null;

  addProjectApiStatus: TApiStatus;
  addProjectApiMsg: string;

  projectApiStatus: TApiStatus;
  projectApiMsg: string;
  project: ProjectShallowModel | null;
  projectSettings: ProjectSettingsModel | null;

  projectScansApiStatus: TApiStatus;
  projectScansApiMsg: string;
  projectScans: ScanResultModel[] | null;
  projectScanFiles: ScanFileModel[] | null;

  projectScanResultsApiStatus: TApiStatus;
  projectScanResultsApiMsg: string;

  saveProjectSettingsApiStatus: TApiStatus;
  saveProjectSettingsApiMsg: string;
}

const state = (): IState => ({
  projectsApiStatus: TApiStatus.default,
  projectsApiMsg: "",
  projects: null,

  addProjectApiStatus: TApiStatus.default,
  addProjectApiMsg: "",

  projectApiStatus: TApiStatus.default,
  projectApiMsg: "",
  project: null,
  projectSettings: null,

  projectScansApiStatus: TApiStatus.default,
  projectScansApiMsg: "",
  projectScans: null,
  projectScanFiles: null,

  projectScanResultsApiStatus: TApiStatus.default,
  projectScanResultsApiMsg: "",

  saveProjectSettingsApiStatus: TApiStatus.default,
  saveProjectSettingsApiMsg: "",
});

export const useProjectsStore = defineStore("projects", {
  state,
  actions: {
    async getProjects() {
      try {
        this.projectsApiStatus = TApiStatus.loading;
        this.projectsApiMsg = "";

        const { data } = await projectsService.getProjects();
        this.projects = data.map(ProjectShallowModel.fromJson);

        this.projectsApiStatus = TApiStatus.success;
      } catch (e) {
        this.projectsApiStatus = TApiStatus.error;
        this.projectsApiMsg = getApiMessage(e);
      }
    },
    async getProject(uuid: string) {
      try {
        this.projectApiStatus = TApiStatus.loading;
        this.projectApiMsg = "";

        const {
          data: { project, settings: settingsJson },
        } = await projectsService.getProject(uuid);

        let settings: ProjectSettingsModel;
        if (settingsJson == null) {
          // If no project settings exist, pick select settings from the global settings
          const store = useSettingsStore();
					await store.getSettings();
					const globalSettings = store.settings?.scan;

          settings = ProjectSettingsModel.fromJson({
            merge_settings: true,
            use_gitignore: globalSettings?.useGitignore,
          });
        } else {
          settings = ProjectSettingsModel.fromJson(settingsJson);
        }

        this.project = ProjectShallowModel.fromJson(project);
        this.projectSettings = settings;

        this.projectApiStatus = TApiStatus.success;
      } catch (e) {
        this.projectApiStatus = TApiStatus.error;
        this.projectApiMsg = getApiMessage(e);
      }
    },
    async addProject(payload: IProjectAddPayload) {
      try {
        this.addProjectApiStatus = TApiStatus.loading;
        this.addProjectApiMsg = "";

        const { data } = await projectsService.addProject(payload);
        const project = ProjectShallowModel.fromJson(data);
        if (this.projects === null) {
          this.projects = [project];
        } else {
          this.projects.unshift(project);
        }

        this.addProjectApiStatus = TApiStatus.success;
      } catch (e) {
        this.addProjectApiStatus = TApiStatus.error;
        this.addProjectApiMsg = getApiMessage(e);
      }
    },
    async getProjectScans(uuid: string) {
      try {
        this.projectScansApiStatus = TApiStatus.loading;
        this.projectScansApiMsg = "";

        const {
          data: { scans, files },
        } = await projectsService.getProjectScans(uuid);
        this.projectScans = scans.map(ScanResultModel.fromJson);
        this.projectScanFiles = files.map(ScanFileModel.fromJson);

        this.projectScansApiStatus = TApiStatus.success;
      } catch (e) {
        this.projectScansApiStatus = TApiStatus.error;
        this.projectScansApiMsg = getApiMessage(e);
      }
    },
    async scanProject(uuid: string) {
      try {
        this.projectScanResultsApiStatus = TApiStatus.loading;
        this.projectScanResultsApiMsg = "";

        const [{ data }, _] = await Promise.all([
          projectsService.scanProject(uuid),
          delay(1000),
        ]);
        const scan = ScanResultModel.fromJson(data.scan);

        if (this.projectScans === null) {
          this.projectScans = [scan];
        } else {
          this.projectScans.push(scan);
        }

        if (this.project?.uuid === uuid) {
          const project = this.project;

          project.loc = scan.loc;
          project.files = scan.files;
          project.scans = (project.scans ?? 0) + 1;
          project.lastScan = scan.scannedAt;

          this.projectScanFiles = data.files
            .map(ScanFileModel.fromJson)
            .sort(({ loc: locA }, { loc: locB }) => locB - locA);
        }
        if (this.projects !== null) {
          const project = this.projects.find((p) => p.uuid == uuid);
          if (project) {
            project.loc = scan.loc;
            project.files = scan.files;
            project.scans = (project.scans ?? 0) + 1;
            project.lastScan = scan.scannedAt;
          }
        }

        this.projectScanResultsApiStatus = TApiStatus.success;
      } catch (e) {
        this.projectScanResultsApiStatus = TApiStatus.error;
        this.projectScanResultsApiMsg = getApiMessage(e);
      }
    },
    async saveSettings(rootDir: string, settings: IProjectSettingsJson) {
      try {
        this.saveProjectSettingsApiStatus = TApiStatus.loading;
        this.saveProjectSettingsApiMsg = "";

        await projectsService.saveSettings(rootDir, settings);
        this.projectSettings = ProjectSettingsModel.fromJson(settings);

        this.saveProjectSettingsApiStatus = TApiStatus.success;
      } catch (e) {
        this.saveProjectSettingsApiStatus = TApiStatus.error;
        this.saveProjectSettingsApiMsg = getApiMessage(e);
      }
    },
  },
});
