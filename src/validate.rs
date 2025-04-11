use std::path::Path;

use crate::{FmanError, FmanResult};

/// Ensures that the given path exists.
///
/// Returns [`FmanError::NotFound`] if the path does not exist.
pub(crate) fn ensure_exists(path: &Path) -> FmanResult<()> {
    if !path.exists() {
        return Err(FmanError::NotFound(path.display().to_string()));
    }

    Ok(())
}

/// Ensures that the given path does **not** exist.
///
/// Returns [`FmanError::AlreadyExists`] if the path exists.
pub(crate) fn ensure_not_exists(path: &Path) -> FmanResult<()> {
    if path.exists() {
        return Err(FmanError::AlreadyExists(path.display().to_string()));
    }

    Ok(())
}

/// Ensures that the given path points to a regular file.
///
/// Returns [`FmanError::InvalidInput`] if it exists but is not a file.
pub(crate) fn ensure_is_file(path: &Path) -> FmanResult<()> {
    if !path.is_file() {
        return Err(FmanError::InvalidInput(format!(
            "Not a regular file: {}", path.display()
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env::temp_dir;

    use crate::test_utils::{cleanup, setup_temp_dir, setup_temp_file};

    use super::*;

    #[test]
    fn test_ensure_exists_success() {
        let path = setup_temp_file("fman_validate_exists.txt", "content");
        assert!(ensure_exists(&path).is_ok());
        cleanup(&path);
    }

    #[test]
    fn test_ensure_exists_fails() {
        let missing = temp_dir().join("definitely_missing.txt");
        assert!(matches!(ensure_exists(&missing), Err(FmanError::NotFound(_))));
    }

    #[test]
    fn test_ensure_not_exists_success() {
        let path = temp_dir().join("fman_nonexistent.txt");
        assert!(ensure_not_exists(&path).is_ok());
    }

    #[test]
    fn test_ensure_not_exists_fails() {
        let path = setup_temp_file("fman_exists.txt", "existing");
        assert!(matches!(ensure_not_exists(&path), Err(FmanError::AlreadyExists(_))));
        cleanup(&path);
    }

    #[test]
    fn test_ensure_is_file_success() {
        let path = setup_temp_file("fman_regular_file.txt", "data");
        assert!(ensure_is_file(&path).is_ok());
        cleanup(&path);
    }

    #[test]
    fn test_ensure_is_file_fails_for_directory() {
        let dir = setup_temp_dir("fman_fakefile_dir");
        assert!(matches!(ensure_is_file(&dir), Err(FmanError::InvalidInput(_))));
        cleanup(&dir);
    }
}
