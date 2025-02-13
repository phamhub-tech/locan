export enum TApiStatus {
	default,
	loading,
	success,
	error
}

export type TSortType = 'asc' | 'desc' | undefined;
export type TSortParams<T extends string> = {
	[_ in T]?: TSortType
}
export interface IApiRequestParams<Sort = TSortParams<any>, Search = string> {
	pageSize?: number
	page?: number

	sortParams?: Sort;
	filterParams?: any
	search?: { query: string; fields: Search[] }
}
export interface IApiResponse<T = null> {
	error: boolean
	message: string
	data: T
}

export type TApiErrorData = Record<string, string[]> | null;
export interface IApiError {
	error: true
	message: string
	data?: TApiErrorData
}
