export interface IProfileJson {
	id: number;
	email: string;
	first_name: string;
	last_name: string;
	date_joined: string;
	avatar: string | null;
}
export interface IProfile {
	id: number;
	email: string;
	firstName: string;
	lastName: string;
	joined: Date;
	avatar: string | null;
}
