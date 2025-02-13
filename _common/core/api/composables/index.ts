import { computed, type ComputedRef, type Ref } from 'vue'

import { TApiStatus } from '../types'

export interface IApiHandle {
	status: Ref<TApiStatus>

	isLoading: ComputedRef<boolean>
	isSuccess: ComputedRef<boolean>
	isError: ComputedRef<boolean>
}

export class ApiHandle implements IApiHandle {
	status: Ref<TApiStatus>

	constructor(apiStatus: Ref<TApiStatus>) {
		this.status = apiStatus
	}

	isDefault = computed<boolean>(() => this.status.value === TApiStatus.default)
	isLoading = computed<boolean>(() => this.status.value === TApiStatus.loading)
	isSuccess = computed<boolean>(() => this.status.value === TApiStatus.success)
	isError = computed<boolean>(() => this.status.value === TApiStatus.error)
}

export function useApiHandle(apiStatus: Ref<TApiStatus>) {
	return new ApiHandle(apiStatus)
}
