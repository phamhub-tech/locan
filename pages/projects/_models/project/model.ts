import { BaseModel } from "~/_common/models";
import {
	type IProjectShallow,
	type IProjectBase,
	type IProjectBaseJson,
	type IProjectShallowJson
} from "./types";

export class ProjectBase extends BaseModel implements IProjectBase {
	public name: string;
	public loc: number | null;
	constructor(data: IProjectBase) {
		super(data)

		this.name = data.name
		this.loc = data.loc
	}

	protected static buildData(json: IProjectBaseJson): IProjectBase {

		return {
			...BaseModel.buildJson(json),
			name: json.name,
			loc: json.loc,
		}
	}

	override toJson(): IProjectBaseJson {
		return {
			...super.toJson(),
			name: this.name,
			loc: this.loc,
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
