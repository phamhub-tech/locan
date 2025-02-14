use butane::{model, AutoPk};
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
    pub loc: Option<i32>,
    pub files: Option<i16>,
		pub root_dir: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
