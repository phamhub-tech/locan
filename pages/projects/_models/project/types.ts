import type { IBaseModel, IBaseModelJson } from "~/_common/models";

export interface IProjectBaseJson extends IBaseModelJson {
	name: string
	loc: number | null
}
export interface IProjectBase extends IBaseModel {
	name: string
	loc: number | null
}

export interface IProjectShallowJson extends IProjectBaseJson { }
export interface IProjectShallow extends IProjectBase { }

