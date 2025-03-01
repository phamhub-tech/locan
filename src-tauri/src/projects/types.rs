use serde::Serialize;

use crate::settings::models::ProjectScanSettings;

use super::models::Project;

#[derive(Serialize)]
pub struct ProjectResponse {
    pub project: Project,
    pub settings: Option<ProjectScanSettings>,
}
