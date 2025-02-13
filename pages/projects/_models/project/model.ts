import { BaseModel } from "~/_common/models";
import {
	type IProjectShallow,
	type IProjectBase,
	type IProjectBaseJson,
	type IProjectShallowJson
} from "./types";

export class ProjectBase extends BaseModel implements IProjectBase {
	public name: string;
	public rootDir: string;
	public loc: number | null;
	public files: number | null;
	constructor(data: IProjectBase) {
		super(data)

		this.name = data.name
		this.rootDir = data.rootDir
		this.loc = data.loc
		this.files = data.files
	}

	protected static buildData(json: IProjectBaseJson): IProjectBase {
		return {
			...BaseModel.buildJson(json),
			name: json.name,
			rootDir: json.root_dir,
			loc: json.loc,
			files: json.files,
		}
	}

	override toJson(): IProjectBaseJson {
		return {
			...super.toJson(),
			name: this.name,
			root_dir: this.rootDir,
			loc: this.loc,
			files: this.files,
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
