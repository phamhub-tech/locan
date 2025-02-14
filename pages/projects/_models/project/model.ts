import { BaseModel } from "~/_common/models";
import {
	type IProjectShallow,
	type IProjectBase,
	type IProjectBaseJson,
	type IProjectShallowJson,
	type IProject
} from "./types";

export class ProjectBase extends BaseModel implements IProjectBase {
	public name: string;
	public rootDir: string;
	public loc: number | null;
	public files: number | null;
	public lastScan: Date | null;
	constructor(data: IProjectBase) {
		super(data)

		this.name = data.name
		this.rootDir = data.rootDir
		this.loc = data.loc
		this.files = data.files
		this.lastScan = data.lastScan
	}

	protected static buildData(json: IProjectBaseJson): IProjectBase {
		return {
			...BaseModel.buildJson(json),
			name: json.name,
			rootDir: json.root_dir,
			loc: json.loc,
			files: json.files,
			lastScan: json.lastScan ? new Date(json.lastScan) : null
		}
	}

	override toJson(): IProjectBaseJson {
		return {
			...super.toJson(),
			name: this.name,
			root_dir: this.rootDir,
			loc: this.loc,
			files: this.files,
			lastScan: this.lastScan?.toISOString() ?? null,
		}
	}
}

export class ProjectShallowModel extends ProjectBase implements IProjectShallow {
	constructor(data: IProjectShallow) {
		super(data)
	}

	static fromJson(json: IProjectShallowJson): ProjectShallowModel {
		return new ProjectShallowModel(
			ProjectBase.buildData(json)
		)
	}
}

export class ProjectModel extends ProjectBase implements IProject {
	constructor(data: IProject) {
		super(data)
	}

	static fromJson(json: IProjectShallowJson): ProjectModel {
		return new ProjectModel(
			ProjectBase.buildData(json)
		)
	}
}
