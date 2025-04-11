use crate::copy::copy_file;
use crate::error::FmanError;

pub mod cli;
mod copy;
mod error;
mod validate;

/// Result type for all fman operations.
///
/// This wraps all `Result<T, FmanError>` types used throughout the `fman` crate.
pub type FmanResult<T> = Result<T, FmanError>;

/// Shared test helpers available only during testing.
#[cfg(test)]
pub mod test_utils;

/// Copy a file without overwriting the destination.
///
/// If the destination file already exists, this function will return an error.
/// This is the default, safe behavior when copying files.
///
/// # Arguments
///
/// * `src` - Path to the source file.
/// * `dst` - Path to the destination directory or full destination file path.
///
/// # Returns
///
/// A `Result<(), FmanError>` indicating success or failure.
///
/// # Example
///
/// ```no_run
/// use fman::copy_file_safe;
///
/// let result = copy_file_safe("file.txt", "backup/");
/// assert!(result.is_ok());
/// ```
pub fn copy_file_safe(src: &str, dst: &str) -> FmanResult<()> {
    copy_file(src, dst, false)
}

/// Copy a file, overwriting the destination if it already exists.
///
/// This version allows destructive copy behavior. If the destination exists,
/// it will be overwritten without warning.
///
/// # Arguments
///
/// * `src` - Path to the source file.
/// * `dst` - Path to the destination directory or full destination file path.
///
/// # Returns
///
/// A `Result<(), FmanError>` indicating success or failure.
///
/// # Example
///
/// ```no_run
/// use fman::copy_file_force;
///
/// let result = copy_file_force("file.txt", "backup/");
/// assert!(result.is_ok());
/// ```
pub fn copy_file_force(src: &str, dst: &str) -> FmanResult<()> {
    copy_file(src, dst, true)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::cli::try_run;
    use crate::test_utils::{cleanup, setup_temp_dir, setup_temp_file};

    use super::*;

    #[test]
    fn test_copy_file_safe_success() {
        let src = setup_temp_file("lib_safe_src.txt", "safe content");
        let dst_dir = setup_temp_dir("lib_safe_dst");

        let result = copy_file_safe(src.to_str().unwrap(), dst_dir.to_str().unwrap());
        assert!(result.is_ok());

        let copied = dst_dir.join("lib_safe_src.txt");
        assert!(copied.exists());

        let content = fs::read_to_string(&copied).unwrap();
        assert_eq!(content, "safe content");

        cleanup(&src);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_copy_file_safe_fails_if_exists() {
        let src = setup_temp_file("lib_safe_exists.txt", "original");
        let dst_dir = setup_temp_dir("lib_safe_exists_dst");
        let dst_file = dst_dir.join("lib_safe_exists.txt");

        fs::write(&dst_file, "existing").unwrap();

        let result = copy_file_safe(src.to_str().unwrap(), dst_dir.to_str().unwrap());
        assert!(matches!(result, Err(FmanError::AlreadyExists(_))));

        cleanup(&src);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_copy_file_force_overwrites_existing() {
        let src = setup_temp_file("lib_force.txt", "new content");
        let dst_dir = setup_temp_dir("lib_force_dst");
        let dst_file = dst_dir.join("lib_force.txt");

        fs::write(&dst_file, "old content").unwrap();

        let result = copy_file_force(src.to_str().unwrap(), dst_dir.to_str().unwrap());
        assert!(result.is_ok());

        let content = fs::read_to_string(&dst_file).unwrap();
        assert_eq!(content, "new content");

        cleanup(&src);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_cli_copy_safe_success() {
        let src = setup_temp_file("cli_copy_src.txt", "hello!");
        let dst_dir = setup_temp_dir("cli_copy_dst");

        let args = [
            "fman", "copy",
            src.to_str().unwrap(),
            dst_dir.to_str().unwrap()
        ];

        let result = try_run(args);
        assert!(result.is_ok());

        let copied = dst_dir.join("cli_copy_src.txt");
        assert!(copied.exists());

        cleanup(&src);
        cleanup(&dst_dir);
    }

    #[test]
    fn test_cli_copy_force_success() {
        let src = setup_temp_file("cli_copy_force.txt", "new!");
        let dst_dir = setup_temp_dir("cli_copy_force_dst");
        let dst_file = dst_dir.join("cli_copy_force.txt");

        std::fs::write(&dst_file, "old").unwrap();

        let args = [
            "fman", "copy",
            src.to_str().unwrap(),
            dst_dir.to_str().unwrap(),
            "--force"
        ];

        let result = try_run(args);
        assert!(result.is_ok());

        let contents = fs::read_to_string(dst_file).unwrap();
        assert_eq!(contents, "new!");

        cleanup(&src);
        cleanup(&dst_dir);
    }
}
