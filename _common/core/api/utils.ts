import type { IApiRequestParams, IApiResponse } from './types'

type TParsedParams = Record<string, string | boolean | any[]>

export function getApiMessage(
	err: any,
	message = 'An error occured. Please try again later.',
): string {
	const msg = (err as IApiResponse).message ?? message

	if (msg === message) console.error(err)
	return msg
}

export const isTauri = !!window.__TAURI__

/**
 * Parses one of search, sort or filter params
 *
 * @param param The params group (search | sort | filter) to be parsed
 */
export function parseParam(param?: Record<string, any>): string | null {
	if (!param || Object.keys(param).length === 0) return null

	const parsed = Object.entries(param)
		.filter(([, value]) => value !== null || value !== undefined || value.toString().trim() !== '')
		.map(([field, value]) => (value == '' || value === null ? '' : `${field}::${value}`))
		.join('|')
	return parsed !== '' ? parsed : null
}

/**
 * Parses the api search params
 *
 * @param param The search param
 */
function parseSearchParam(param?: IApiRequestParams['search']): string | null {
	if (!param) return null

	const q = param.query
	if (q.trim() === '') return null

	const parsed = param.fields.map((field) => `${field}::${q}`).join('|')
	return parsed
}

/**
 * Parses all api parameters
 *
 * @param params The params to parse
 */
export function parseParams<T extends IApiRequestParams>(params?: T) {
	if (!params) return null

	const finalParams: TParsedParams = {}

	const searchParams = parseSearchParam(params?.search)
	if (searchParams) finalParams['search'] = searchParams

	const sortParams = parseParam(params?.sortParams)
	if (sortParams) finalParams['sort'] = sortParams

	const filterParams = parseParam(params?.filterParams)
	if (filterParams) finalParams['filter'] = filterParams


	delete params?.filterParams
	delete params?.sortParams
	delete params?.search
	Object.assign(finalParams, params)

	return finalParams
}
