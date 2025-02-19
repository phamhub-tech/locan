use regex::Regex;

/// Returns the computed extension from the given file path
pub fn get_extension_from_path(path: &str) -> Option<String> {
    let compound_extensions = vec!["env.example"]
        .iter()
        .map(|ext| ext.to_string())
        .reduce(|acc, ext| format!("{acc}|{ext}"))
        .expect("Could not build compund_extensions regex");
    let pattern = format!("({compound_extensions})$");

    let re = Regex::new(&pattern).unwrap();
    if let Some(captures) = re.captures(path) {
        if let Some(cap) = captures.get(1) {
            return Some(cap.as_str().to_string());
        }
    } else  {
        if let Some(ext) = path.split(".").last() {
            return Some(ext.to_string());
        }
    }

    None
}

/// Return the file type from the file name
pub fn file_type_from_file_name(name: &str) -> String {
    match name {
        "env" | "env.example" => "env",

        "html" => "html",
        "md" => "markdown",
        "mdx" => "mdx",
        "gitignore" => "git",
        "js" | "cjs" | "mjs" => "javascript",
        "jsx" => "jsx",
        "json" => "json",
        "prettierrc" => "prettier",
        "rs" => "rust",
        "sass" | "scss" => "sass",
        "sh" => "console",
        "svg" => "svg",
        "table" | "sql" => "database",
        "toml" => "toml",
        "ts" => "typescript",
        "tsx" => "tsx",
        "txt" => "text",
        "vue" => "vue",
        "yaml" | "yml" => "yaml",
        _ => "unknown",
    }
    .to_string()
}
