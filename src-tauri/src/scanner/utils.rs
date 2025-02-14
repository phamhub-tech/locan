/// Gets the name of an entry
pub fn get_entry_name(entry: &walkdir::DirEntry) -> String {
    entry.file_name().to_string_lossy().to_string()
}

/// Gets the path of an entry
pub fn get_entry_path(entry: &walkdir::DirEntry) -> String {
    entry.path().to_string_lossy().to_string()
}
