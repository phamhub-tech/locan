import { IAuthRole, ITokenType, type ITokenPayload } from "./types"

export * from './types';

export class AuthToken {
	private _payload: ITokenPayload
	private _raw: string

	role: IAuthRole
	id: number

	constructor(raw: string) {
		this._raw = raw;

		const tokenSplit = raw.split('.')
		if (tokenSplit.length !== 3) throw Error(`You have entered an invalid token: ${raw}`)

		const [, payloadRaw, ,] = tokenSplit
		const payload = JSON.parse(atob(payloadRaw)) as ITokenPayload

		const roleRaw = payload.role
		const role = IAuthRole.admin

		this._payload = payload
		this.role = role
		this.id = payload.user_id
	}

	get raw(): string {
		return this._raw;
	}

	/**
	 * Checks whether this token hasn't expired
	 *
	 * @param [offset=0] The offset in milliseconds from the actual expiry date to check for expiry
	 */
	isValid(offset = 0): boolean {
		const expiry = this._payload.exp * 1000
		const expiryDate = new Date(expiry - offset)
		const now = new Date()

		return now < expiryDate
	}

	isValidForRole(role: IAuthRole): boolean {
		return this.isValid() && role === this.role
	}

	hasAnyRoles(...roles: IAuthRole[]): boolean {
		return roles.some((role) => role === this.role)
	}
}

export function retrieveToken(type = ITokenType.access): AuthToken | null {
	const { $storage } = useNuxtApp()
	let token = $storage.getItem(type)
	if (token === null) token = $storage.getItem(type, true)

	return token !== null ? new AuthToken(token) : null
}

export function saveTokens(tokens: { access: string; refresh: string }, persist: boolean) {
	const { $storage } = useNuxtApp()

	$storage.setItem(ITokenType.access, tokens.access, persist)
	$storage.setItem(ITokenType.refresh, tokens.refresh, persist)

	return new AuthToken(tokens.access)
}
