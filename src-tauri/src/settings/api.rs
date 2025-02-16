use std::sync::Mutex;

use tauri::State;

use crate::{
    api::{ApiError, ApiResponse}, api_error, api_response
};

use super::models::AppSettings;

#[tauri::command]
pub fn get_settings(
    app_settings: State<Mutex<AppSettings>>,
) -> Result<ApiResponse<AppSettings>, ApiError> {
    let guard = app_settings.lock().map_err(|e| api_error!(e.to_string()))?;
    let settings = guard.clone();

    Ok(api_response!(settings))
}
