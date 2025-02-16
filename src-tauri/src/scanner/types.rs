use serde::Serialize;

use crate::scanner::{ScanResult, ScanFileType};

#[derive(Debug, Default, Serialize)]
pub struct LocAnalysis {
    pub loc: i32,
    pub blanks: i32,
    pub files: i32,
}

#[derive(Debug, Serialize)]
pub struct ScanResponse {
    pub scan: ScanResult,
    pub files: Vec<ScanFileType>,
}

pub enum FileType {}



