use butane::{model, AutoPk, ForeignKey};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::projects::models::Project;

#[model]
#[derive(Debug, Serialize)]
pub struct ScanResult {
    pub id: AutoPk<i32>,
    pub project: ForeignKey<Project>,

    pub loc: i32,
    pub files: i16,

    pub scanned_at: DateTime<Utc>,
}

#[model]
#[derive(Debug, Serialize)]
pub struct ScanFileType {
    pub id: AutoPk<i32>,
    pub scan: ForeignKey<ScanResult>,
    pub file_type: String,

    pub loc: i32,
    pub files: i16,

    pub created_at: DateTime<Utc>,
}
