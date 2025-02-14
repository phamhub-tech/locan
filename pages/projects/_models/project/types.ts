import type { IBaseModel, IBaseModelJson } from "~/_common/models";

export interface IProjectBaseJson extends IBaseModelJson {
	name: string
	root_dir: string
	loc: number | null
	files: number | null
	lastScan: string | null
}
export interface IProjectBase extends IBaseModel {
	name: string
	rootDir: string
	loc: number | null
	files: number | null
	lastScan: Date | null
}

export interface IProjectShallowJson extends IProjectBaseJson { }
export interface IProjectShallow extends IProjectBase { }

export interface IProjectJson extends IProjectBaseJson { }
export interface IProject extends IProjectBase { }

export interface IProjectScanJson {
	loc: number
	blanks: number
}
export interface IProjectScan {
	loc: number
	blanks: number
}

