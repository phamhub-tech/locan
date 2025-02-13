import type { TSortType } from "~/_common/core/api"

export type TDataTableAlignment = 'left' | 'center' | 'right'
export interface IHeader<T> {
	label: string;
	value: (keyof T | Omit<string, keyof T | number | symbol>) | ((item: T) => string | number | null);
	sort?: TSortType | null;
	classes?: string;
	itemClasses?: string | ((item: T) => string);
	isDate?: boolean;
	align?: TDataTableAlignment;

	/**
	 * Used to uniquely identify this header
	 */
	key?: string;

	/**
	 * Used to show whether the value for this header should
	 * be computed
	 */
	ignoreValue?: boolean;
}

export interface ISortField<T = any> {
	name: keyof T;
	value: TSortType | undefined;
}
