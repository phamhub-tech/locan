use tauri::Manager;

pub mod api;
pub mod butane_migrations;
pub mod config;
pub mod db;
pub mod projects;
pub mod scanner;

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
        .invoke_handler(tauri::generate_handler![
            projects::add_project,
            projects::get_projects,
            projects::get_project,
            scanner::scan_project,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
