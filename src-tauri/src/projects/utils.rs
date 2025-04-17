use std::path::Path;
use std::fs;

pub fn detect_framework(root_dir: &str) -> Option<String> {
    let path = Path::new(root_dir);

    // Next.js detection
    let next_config = path.join("next.config.js");
    let next_config_ts = path.join("next.config.ts");
    if next_config.exists() || next_config_ts.exists() {
        return Some("nextjs".to_string());
    }

    // Nuxt.js detection
    let nuxt_config = path.join("nuxt.config.ts");
    if nuxt_config.exists() {
        return Some("nuxtjs".to_string());
    }

    // Flutter detection
    let pubspec = path.join("pubspec.yaml");
    if pubspec.exists() {
        if let Ok(content) = fs::read_to_string(&pubspec) {
            if content.contains("flutter:") {
                return Some("flutter".to_string());
            }
        }
    }

    None
}