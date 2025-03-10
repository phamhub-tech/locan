use serde::Serialize;

use super::models::{ScanFile, ScanResult};

#[derive(Debug, Default, Serialize)]
pub struct LocAnalysis {
    pub loc: i32,
    pub blanks: i32,
    pub files: i32,
}

#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub scan: ScanResult,
    pub files: Vec<ScanFile>,
}

#[derive(Debug, Serialize)]
pub struct ScansResponse {
    pub scans: Vec<ScanResult>,
    pub files: Vec<ScanFile>,
}
