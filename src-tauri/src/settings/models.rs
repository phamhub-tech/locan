use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

use crate::utils::get_app_data_dir;

pub struct AppSettingsManager {
    pub settings: Arc<Mutex<AppSettings>>,
    path: PathBuf,
}
impl AppSettingsManager {
    /// Loads the app settings from the json file
    pub fn new(handle: &AppHandle) -> Self {
        let json_path = get_app_data_dir(handle).join("settings.json");
        let settings = Self::load_from_file(&json_path).unwrap_or_else(|_| AppSettings::default());
        let manager = Self {
            settings: Arc::new(Mutex::new(settings)),
            path: json_path,
        };

        manager
    }

    /// Saves the app settings
    pub fn save(&self, new_settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
        println!("Saving new settings: {:?}", new_settings);

        // TODO: Handle errors properly
        let mut settings = self.settings.lock().unwrap();
        *settings = new_settings.clone();

        let content = serde_json::to_string_pretty(&*settings).unwrap();
        fs::write(&self.path, content).unwrap();

        println!("Saved settings");
        Ok(())
    }

    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<AppSettings, Box<dyn std::error::Error>> {
        let content = read_to_string(path)?;
        let settings = serde_json::from_str::<AppSettings>(&content)?;
        Ok(settings)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    #[serde(default)]
    pub scan: ScanSettings,
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
