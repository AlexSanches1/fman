use std::io;

use thiserror::Error;

/// Represents all possible errors that can occur in the `fman` CLI tool.
#[derive(Error, Debug)]
pub enum FmanError {
    /// The specified source file or path does not exist.
    ///
    /// This typically indicates that the user provided a path that cannot be found.
    #[error("Source file not found: {0}")]
    NotFound(String),

    /// The provided input was invalid.
    ///
    /// This can include cases like trying to operate on a directory instead of a file,
    /// or providing a path that is not allowed for the requested operation.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// The destination path already exists.
    ///
    /// Used to prevent overwriting files unless explicitly allowed.
    #[error("Destination file already exists: {0}")]
    AlreadyExists(String),

    /// Wrapper for unexpected I/O errors.
    ///
    /// This includes filesystem issues such as permission denied, disk full, etc.
    /// The original `std::io::Error` is preserved and displayed transparently.
    #[error(transparent)]
    Io(#[from] io::Error),
}
