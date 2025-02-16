export interface ILocScanChartData {
  label: string;
  value: number;
}

export interface ILocScanFile {
  icon: string;
  file: TFileType;
  loc: number;
}

export enum TDuration {
  thisMonth = "thisMonth",
  thisYear = "thisYear",
  allTime = "allTime",
}

export enum TFileType {
  rust = "rust",
  vue = "vue",
  ts = "typescript",
  json = "json",
  toml = "toml",
}
