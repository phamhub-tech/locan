/// Gets the name of an entry
pub fn get_entry_name(entry: &walkdir::DirEntry) -> String {
    entry.file_name().to_string_lossy().to_string()
}

/// Gets the path of an entry
pub fn get_entry_path(entry: &walkdir::DirEntry) -> String {
    entry.path().to_string_lossy().to_string()
}

/// Return the file type from the extension
pub fn file_type_from_extension(extension: &str) -> String {
    match extension {
        "md" => "markdown",
        "gitignore" => "git",
        "js" => "javascript",
        "json" => "json",
        "rs" => "rust",
        "toml" => "toml",
        "ts" => "typescript",
        "vue" => "vue",
        "yaml" | "yml" => "yaml",
        _ => "unknown"
    }.to_string()
}
