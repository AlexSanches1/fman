use std::fs;
use std::path::{Path, PathBuf};

use crate::error::FmanError;
use crate::FmanResult;
use crate::validate::{ensure_exists, ensure_is_file, ensure_not_exists};

/// Copies a file to the specified destination path.
///
/// If the destination path is a directory, the file will be copied into that directory
/// using its original filename. If the destination path is a full file path, the file
/// will be copied directly to that location.
///
/// # Arguments
///
/// * `src` - Path to the source file.
/// * `dst` - Destination directory or full destination file path.
/// * `force` - If `true`, the destination file will be overwritten if it exists.
///             If `false`, an error will be returned when the destination file already exists.
///
/// # Errors
///
/// Returns a [`FmanError`] if:
/// - The source path does not exist or is not a regular file.
/// - The destination file already exists and `force` is `false`.
/// - The source path has no filename component (e.g. `/tmp/` or empty path).
/// - An I/O error occurs during copying.
pub(crate) fn copy_file(src: &str, dst: &str, force: bool) -> FmanResult<()> {
    let src_path = Path::new(src);
    let dst_path = Path::new(dst);

    // Validate source
    ensure_exists(src_path)?;
    ensure_is_file(src_path)?;

    // Resolve destination file path (directory or full file path)
    let dst_file_path = resolve_destination_path(src_path, dst_path)?;

    // Overwrite control
    if !force {
        ensure_not_exists(&dst_file_path)?;
    }

    fs::copy(src_path, &dst_file_path)?;
    Ok(())
}

/// Resolves the final destination file path from the given input path.
///
/// If `dst_path` is a directory, appends the filename from `src_path`.
/// If `dst_path` is a file path, returns it directly.
///
/// # Errors
///
/// Returns a [`FmanError::InvalidInput`] if `src_path` does not have a valid filename.
fn resolve_destination_path(src_path: &Path, dst_path: &Path) -> Result<PathBuf, FmanError> {
    if dst_path.is_dir() {
        let filename = src_path
            .file_name()
            .ok_or_else(|| FmanError::InvalidInput(format!(
                "Source path '{}' has no file name", src_path.display()
            )))?;
        Ok(dst_path.join(filename))
    } else {
        Ok(dst_path.to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use crate::test_utils::{cleanup, setup_temp_dir, setup_temp_file};

    use super::*;

    #[test]
    fn test_copy_into_directory_success() {
        let src = setup_temp_file("fman_src.txt", "test copy content");
        let dst_dir = setup_temp_dir("fman_copy_test");

        let result = copy_file(src.to_str().unwrap(), dst_dir.to_str().unwrap(), false);

        assert!(result.is_ok());

        let copied_file = dst_dir.join("fman_src.txt");
        assert!(copied_file.exists());

        let copied_content = fs::read_to_string(copied_file).unwrap();
        assert_eq!(copied_content, "test copy content");

        cleanup(&src);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_copy_overwrite_success() {
        let src = setup_temp_file("fman_src_overwrite.txt", "new content");
        let dst_dir = setup_temp_dir("fman_copy_overwrite");
        let dst_file = dst_dir.join("fman_src_overwrite.txt");

        fs::write(&dst_file, "old content").unwrap();

        let result = copy_file(src.to_str().unwrap(), dst_dir.to_str().unwrap(), true);
        assert!(result.is_ok());

        let content = fs::read_to_string(dst_file).unwrap();
        assert_eq!(content, "new content");

        cleanup(&src);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_copy_fails_when_file_exists_and_force_false() {
        let src = setup_temp_file("fman_src_noforce.txt", "latest content");
        let dst_dir = setup_temp_dir("fman_copy_noforce");
        let dst_file = dst_dir.join("fman_src_noforce.txt");

        fs::write(&dst_file, "existing").unwrap();

        let result = copy_file(src.to_str().unwrap(), dst_dir.to_str().unwrap(), false);
        assert!(matches!(result, Err(FmanError::AlreadyExists(_))));

        cleanup(&src);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_fails_if_source_does_not_exist() {
        let fake_src = temp_dir().join("nonexistent.txt");
        let dst_dir = setup_temp_dir("fman_fail_no_src");

        let result = copy_file(fake_src.to_str().unwrap(), dst_dir.to_str().unwrap(), false);
        assert!(matches!(result, Err(FmanError::NotFound(_))));

        cleanup(&dst_dir);
    }

    #[test]
    fn test_fails_if_source_is_directory() {
        let dir = setup_temp_dir("fman_fail_is_dir");
        let dst_dir = setup_temp_dir("fman_target");

        let result = copy_file(dir.to_str().unwrap(), dst_dir.to_str().unwrap(), false);
        assert!(matches!(result, Err(FmanError::InvalidInput(_))));

        cleanup(&dir);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_fails_if_source_has_no_filename() {
        let dst_dir = setup_temp_dir("fman_fail_no_filename");

        // Simulate a "filename-less" path (e.g., "/")
        let bad_src = Path::new("/").to_str().unwrap();
        let result = copy_file(bad_src, dst_dir.to_str().unwrap(), false);

        assert!(matches!(result, Err(FmanError::InvalidInput(_))));

        cleanup(&dst_dir);
    }

    #[test]
    fn test_resolve_file_path_unchanged_if_dst_is_file() {
        let src = setup_temp_file("fman_resolve1.txt", "...");
        let dst = temp_dir().join("fman_dest_file.txt");

        let resolved = resolve_destination_path(&src, &dst).unwrap();
        assert_eq!(resolved, dst);

        cleanup(&src);
    }

    #[test]
    fn test_resolve_file_path_joins_if_dst_is_dir() {
        let src = setup_temp_file("fman_resolve2.txt", "...");
        let dst_dir = setup_temp_dir("fman_dest_dir");

        let expected = dst_dir.join("fman_resolve2.txt");
        let resolved = resolve_destination_path(&src, &dst_dir).unwrap();

        assert_eq!(resolved, expected);

        cleanup(&src);
        cleanup(&dst_dir);
    }
}
