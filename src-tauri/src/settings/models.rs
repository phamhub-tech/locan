use std::{
    fs::{self, read_to_string},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use notify::{recommended_watcher, Watcher};
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

        manager.watch_for_changes();
        manager
    }

    /// Saves the app settings
    ///
    /// The current settings will be updated by the watcher, not this function
    pub fn save(&self, new_settings: &AppSettings) -> Result<(), Box<dyn std::error::Error>> {
        println!("Saving new settings: {:?}", new_settings);

        // TODO: Handle errors properly
        let content = serde_json::to_string_pretty(new_settings).unwrap();
        fs::write(&self.path, content).unwrap();
        println!("Saved settings.");
        Ok(())
    }

    fn load_from_file<P: AsRef<Path>>(path: P) -> Result<AppSettings, Box<dyn std::error::Error>> {
        let content = read_to_string(path)?;
        let settings = serde_json::from_str::<AppSettings>(&content)?;
        Ok(settings)
    }

    fn watch_for_changes(&self) {
        let path = self.path.clone();
        let settings_clone = self.settings.clone();

        std::thread::spawn(move || {
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = recommended_watcher(tx).unwrap();
            watcher
                .watch(&path, notify::RecursiveMode::NonRecursive)
                .unwrap();
            println!("Setup file watcher for settings.");

            for res in rx.iter() {
                match res {
                    Ok(event) => {
                        let kind = event.kind;
                        if !(kind.is_create() || kind.is_modify()) {
                            continue;
                        }

                        println!("Received modify event: {:?}", kind);
                        if let Ok(new_settings) = Self::load_from_file(&path) {
                            let mut settings = settings_clone.lock().unwrap();
                            println!("Update new settings: {:?}", new_settings);
                            *settings = new_settings;
                        }
                    }
                    Err(e) => eprintln!("Watch error: {:?}", e),
                }
            }
        });
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
