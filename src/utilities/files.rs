use std::path::Path;
use walkdir::WalkDir;

pub fn get_path_entries(path: &Path) -> Vec<String> {
    WalkDir::new(path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .skip(1) // Skip the first entry (base directory)
        .map(|entry| {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if entry.file_type().is_dir() {
                file_name + "/" // Append "/" to directories
            } else {
                file_name
            }
        })
        .collect()
}
