use tauri::Manager;

mod api;
mod butane_migrations;
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

            let settings_manager = settings::models::AppSettingsManager::new(&handle);
            app.manage(settings_manager);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            projects::api::add_project,
            projects::api::get_projects,
            projects::api::get_project,
            projects::api::save_project_settings,
            scanner::api::get_project_scans,
            scanner::api::scan_project,
            settings::api::get_settings,
            settings::api::save_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
