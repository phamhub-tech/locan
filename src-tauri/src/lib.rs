use std::sync::Mutex;

use tauri::Manager;

mod api;
mod butane_migrations;
mod config;
mod db;
mod projects;
mod scanner;
mod settings;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let handle = app.handle();
            let conn = db::establish_connection(&handle);
            app.manage(conn);

            let app_settings = settings::models::AppSettings::load(&handle);
            app.manage(Mutex::new(app_settings));

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            projects::add_project,
            projects::get_projects,
            projects::get_project,
            scanner::get_project_scans,
            scanner::scan_project,
            settings::api::get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
