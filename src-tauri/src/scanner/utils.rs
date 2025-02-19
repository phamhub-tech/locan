/// Return the file type from the extension
pub fn file_type_from_extension(extension: &str) -> String {
    match extension {
        "md" => "markdown",
        "gitignore" => "git",
        "js" | "cjs" | "mjs" => "javascript",
        "json" => "json",
        "rs" => "rust",
        "sass" | "scss" => "sass",
        "table" | "sql" => "database",
        "toml" => "toml",
        "ts" => "typescript",
        "txt" => "text",
        "vue" => "vue",
        "yaml" | "yml" => "yaml",
        _ => "unknown",
    }
    .to_string()
}
