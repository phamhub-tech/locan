export interface IScanResultJson {
	id: number;
	loc: number,
	files: number,
	scanned_at: string,
}

export interface IScanResult {
	id: number;
	loc: number,
	files: number,
	scannedAt: Date,
}
