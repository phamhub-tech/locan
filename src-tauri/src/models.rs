use butane::{model, AutoPk};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[model]
#[derive(Debug, Default, Serialize)]
pub struct Project {
    pub id: AutoPk<i32>,
    #[unique]
    pub uuid: String,

    #[unique]
    pub name: String,
    pub loc: i32,
    pub files: i16,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
