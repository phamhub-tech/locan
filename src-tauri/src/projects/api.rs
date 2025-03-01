use butane::colname;
use butane::prelude::*;
use butane::Error;
use chrono::Utc;
use tauri::State;
use uuid::Uuid;

use super::models::Project;
use super::types::ProjectResponse;

use crate::api::{ApiError, ApiResponse};
use crate::db::DBConnection;
use crate::settings::models::ProjectScanSettings;
use crate::{api_error, api_response};

#[tauri::command]
pub fn get_projects(db: State<DBConnection>) -> Result<ApiResponse<Vec<Project>>, ApiError> {
    let conn_guard = db.conn.lock().map_err(|e| api_error!(e.to_string()))?;
    let conn = &*conn_guard;
    let projects = Project::query()
        .order_desc(colname!(Project, updated_at))
        .load(conn)
        .map_err(|e| api_error!(e.to_string()))?;

    return Ok(api_response!(projects));
}

#[tauri::command]
pub fn get_project(
    db: State<DBConnection>,
    uuid: String,
) -> Result<ApiResponse<ProjectResponse>, ApiError> {
    let conn_guard = db.conn.lock().map_err(|e| api_error!(e.to_string()))?;
    let conn = &*conn_guard;
    let project = Project::get(conn, uuid).map_err(|e| match e {
        Error::NoSuchObject => api_error!(String::from("Not found")),
        _ => api_error!(e.to_string()),
    })?;

    let settings = ProjectScanSettings::load_from_project_path(&project.uuid).ok();
    Ok(api_response!(ProjectResponse { project, settings }))
}

#[tauri::command]
pub fn add_project(
    db: State<DBConnection>,
    name: String,
    root_dir: String,
) -> Result<ApiResponse<Project>, ApiError> {
    let conn_guard = db.conn.lock().map_err(|e| api_error!(e.to_string()))?;
    let conn = &*conn_guard;

    let mut project = Project {
        uuid: Uuid::new_v4().to_string(),
        name,
        root_dir,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        ..Default::default()
    };
    project.save(conn).map_err(|e| api_error!(e.to_string()))?;
    Ok(api_response!(project))
}
