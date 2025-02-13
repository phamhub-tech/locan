import { AuthToken } from "~/_common/utils"

export default definePayloadPlugin((_) => {
	definePayloadReducer('AuthToken', (value) => value instanceof AuthToken && value.raw)
	definePayloadReviver('AuthToken', (value) => new AuthToken(value))
})
