use butane::model;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[model]
#[derive(Debug, Default, Serialize)]
pub struct Project {
    #[pk]
    #[unique]
    pub uuid: String,

    #[unique]
    pub name: String,
    pub root_dir: String,
    pub loc: Option<i32>,
    pub files: Option<i16>,
    pub scans: Option<i16>,

    pub last_scan: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
