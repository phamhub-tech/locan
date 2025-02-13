export interface IBaseModelJson {
	id: number;
	created_at: string
	updated_at: string
}
export interface IBaseModel {
	id: number;
	createdAt: Date
	updatedAt: Date
}

export class BaseModel implements IBaseModel {
	public id: number;
	public createdAt: Date;
	public updatedAt: Date;

	constructor(data: IBaseModel) {
		this.id = data.id
		this.createdAt = data.createdAt
		this.updatedAt = data.updatedAt
	}

	protected static buildJson(json: IBaseModelJson): IBaseModel {
		return {
			id: json.id,
			createdAt: new Date(json.created_at),
			updatedAt: new Date(json.updated_at),
		}
	}

	toJson(): IBaseModelJson {
		return {
			id: this.id,
			created_at: this.createdAt.toISOString(),
			updated_at: this.updatedAt.toISOString(),
		}
	}
}
