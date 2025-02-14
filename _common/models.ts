export interface IBaseModelJson {
	uuid: string;
	created_at: string
	updated_at: string
}
export interface IBaseModel {
	uuid: string;
	createdAt: Date
	updatedAt: Date
}

export class BaseModel implements IBaseModel {
	public uuid: string;
	public createdAt: Date;
	public updatedAt: Date;

	constructor(data: IBaseModel) {
		this.uuid = data.uuid
		this.createdAt = data.createdAt
		this.updatedAt = data.updatedAt
	}

	protected static buildJson(json: IBaseModelJson): IBaseModel {
		return {
			uuid: json.uuid,
			createdAt: new Date(json.created_at),
			updatedAt: new Date(json.updated_at),
		}
	}

	toJson(): IBaseModelJson {
		return {
			uuid: this.uuid,
			created_at: this.createdAt.toISOString(),
			updated_at: this.updatedAt.toISOString(),
		}
	}
}
