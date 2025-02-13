import type { IProfile, IProfileJson } from "./interface";

export class ProfileModel implements IProfile {
	public id: number;
	public email: string;
	public firstName: string;
	public lastName: string;
	public joined: Date;
	public avatar: string | null;
	constructor(data: IProfile) {
		this.id = data.id
		this.email = data.email
		this.firstName = data.firstName
		this.lastName = data.lastName
		this.avatar = data.avatar
		this.joined = data.joined
	}

	get username(): string {
		return this.email.split('@')[0]
	}

	static fromJson(json: IProfileJson): ProfileModel {
		return new ProfileModel({
			id: json.id,
			email: json.email,
			firstName: json.first_name,
			lastName: json.last_name,
			avatar: json.avatar,
			joined: new Date(json.date_joined),
		})
	}
}
