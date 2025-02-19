/// Return the file type from the extension
pub fn file_type_from_extension(extension: &str) -> String {
    match extension {
        "html" => "html",
        "md" => "markdown",
        "gitignore" => "git",
        "js" | "cjs" | "mjs" => "javascript",
        "json" => "json",
        "rs" => "rust",
        "sass" | "scss" => "sass",
        "svg" => "svg",
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
