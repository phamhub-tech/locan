pub mod api;
pub mod butane_migrations;
pub mod db;
pub mod models;

use api::ApiError;
use butane::colname;
use butane::prelude::*;
use butane::DataResult;
use chrono::Utc;
use db::DBConnection;
use models::Project;
use tauri::{Manager, State};
use uuid::Uuid;

use crate::api::ApiResponse;

#[tauri::command]
fn get_projects(db: State<DBConnection>) -> Result<ApiResponse<Vec<Project>>, ApiError> {
    let conn_guard = db.conn.lock().map_err(|e| api_error!(e.to_string()))?;
    let conn = &*conn_guard;
    let projects = Project::query()
        .order_desc(colname!(Project, updated_at))
        .load(conn)
        .map_err(|e| api_error!(e.to_string()))?;

    return Ok(api_response!(projects));
}

#[tauri::command]
fn add_project(db: State<DBConnection>, name: String, root_dir: String) -> Result<ApiResponse<Project>, ApiError> {
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle();
            let conn = db::establish_connection(&handle);
            app.manage(conn);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_project, get_projects])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
