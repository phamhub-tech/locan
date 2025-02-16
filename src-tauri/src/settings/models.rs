use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::get_app_data_dir;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub scan: ScanSettings,
}
impl AppSettings {
    /// Loads the app settings from the json file
    pub fn load(handle: &AppHandle) -> Self {
        let json_path = get_app_data_dir(handle).join("settings.json");
        if let Ok(content) = read_to_string(json_path) {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                println!("Loaded settings from file");
                return settings;
            };
        }

        println!("Using default settings");
        Self::default()
    }

    /// Saves the app settings
    fn save() {}
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            scan: ScanSettings::default(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScanSettings {
    pub ignore_dirs: Vec<String>,
    pub ignore_extensions: Vec<String>,
}
impl Default for ScanSettings {
    fn default() -> Self {
        ScanSettings {
            ignore_dirs: vec![".git".to_string(), "node_modules".to_string()],
            ignore_extensions: vec!["lock".to_string()],
        }
    }
}
