use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Creates a temporary file with the given name and content.
pub fn setup_temp_file(name: &str, content: &str) -> PathBuf {
    let path = std::env::temp_dir().join(name);
    let mut file = File::create(&path).expect("Failed to create temp file");
    write!(file, "{content}").expect("Failed to write to temp file");
    path
}

/// Creates a temporary directory with the given name.
pub fn setup_temp_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(name);
    fs::create_dir_all(&dir).expect("Failed to create temp dir");
    dir
}

/// Removes the given path, whether file or directory.
pub fn cleanup(path: &Path) {
    if path.is_file() {
        let _ = fs::remove_file(path);
    } else if path.is_dir() {
        let _ = fs::remove_dir_all(path);
    }
}
