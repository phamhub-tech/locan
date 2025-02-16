import type { IScanFile, IScanFileJson, IScanResult, IScanResultJson } from "./types";

export class ScanResultModel implements IScanResult {
	public id: number;
	public loc: number;
	public files: number;
	public scannedAt: Date;
	constructor(data: IScanResult) {
		this.id = data.id
		this.loc = data.loc
		this.files = data.files
		this.scannedAt = data.scannedAt
	}

	static fromJson(json: IScanResultJson): ScanResultModel {
		return new ScanResultModel({
			id: json.id,
			loc: json.loc,
			files: json.files,
			scannedAt: new Date(json.scanned_at),
		});
	}
}

export class ScanFileModel implements IScanFile {
	public id: number;
	public loc: number;
	public files: number;
	public extension: string;
	public fileType: string;
	public createdAt: Date;
	constructor(data: IScanFile) {
		this.id = data.id
		this.loc = data.loc
		this.files = data.files
		this.extension = data.extension
		this.fileType = data.fileType
		this.createdAt = data.createdAt
	}

	static fromJson(json: IScanFileJson): ScanFileModel {
		return new ScanFileModel({
			id: json.id,
			loc: json.loc,
			files: json.files,
			extension: json.extension,
			fileType: json.file_type,
			createdAt: new Date(json.created_at),
		});
	}
}
