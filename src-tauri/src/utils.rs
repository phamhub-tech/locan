use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub fn get_app_data_dir(handle: &AppHandle) -> PathBuf {
    handle
        .path()
        .app_data_dir()
        .expect("Could not get data directory")
}
