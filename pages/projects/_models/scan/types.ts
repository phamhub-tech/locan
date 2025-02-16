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

export interface IScanFileJson {
	id: number
	extension: string
	file_type: string
	loc: number
	files: number
	created_at: string
}
export interface IScanFile {
	id: number
	extension: string
	fileType: string
	loc: number
	files: number
	createdAt: Date
}
