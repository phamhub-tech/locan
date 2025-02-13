export enum ITokenType {
	access = 'token',
	refresh = 'refresh',
}

export enum IAuthRole {
	admin = 'AD',
}

export interface ITokenPayload {
	exp: number;
	iat: number;
	role: 'superadmin' | 'admin';
	user_id: number;
}
