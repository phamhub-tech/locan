import type { TSortParams } from "~/_common/core/api";

import type { ISortField } from "./interface";

export function handleSortParams<Params extends TSortParams<string>>(params: Params, { name, value }: ISortField<Params>) {
	const p: Params = Object.apply({}, params as any)
	const key = name;
	if (value === null) delete p[key];
	else p[key] = value as any;

	return p
}
