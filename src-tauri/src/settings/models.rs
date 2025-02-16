use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::get_app_data_dir;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    #[serde(default)]
    pub scan: ScanSettings,
}
impl AppSettings {
    /// Loads the app settings from the json file
    pub fn load(handle: &AppHandle) -> Self {
        let json_path = get_app_data_dir(handle).join("settings.json");
        if let Ok(content) = read_to_string(json_path) {
            println!("Loading settings from file. {content}");
            match serde_json::from_str::<AppSettings>(&content) {
                Ok(settings) => {
                    println!("Loaded settings from file: {:?}", settings);
                    return settings;
                }
                Err(e) => {
                    eprintln!("An error occured: {:?}", e);
                }
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
    #[serde(default = "ignore_dirs_default")]
    pub ignore_dirs: Vec<String>,

    #[serde(default = "ignore_extensions_default")]
    pub ignore_extensions: Vec<String>,
}
impl Default for ScanSettings {
    fn default() -> Self {
        ScanSettings {
            ignore_dirs: ignore_dirs_default(),
            ignore_extensions: ignore_extensions_default(),
        }
    }
}

fn ignore_dirs_default() -> Vec<String> {
    vec![".git".to_string(), "node_modules".to_string()]
}
fn ignore_extensions_default() -> Vec<String> {
    vec!["lock".to_string()]
}
