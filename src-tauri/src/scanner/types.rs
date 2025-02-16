use serde::Serialize;

use crate::scanner::{ScanFile, ScanResult};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Rust,
    Vue,
    Unknown,
}
impl FileType {
    fn from_extension(extension: &str) -> Self {
        match extension {
            "rs" => FileType::Rust,
            "vue" => FileType::Vue,
            _ => FileType::Unknown,
        }
    }
}
