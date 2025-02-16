import type { IScanResult, IScanResultJson } from "./types";

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
